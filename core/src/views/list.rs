use std::collections::VecDeque;

use ori::{Action, Message, Mut, Proxied, Proxy, Tracker, View, ViewId, ViewMarker};

use crate::{
    Allocation, Context, Direction, FlexStyle, Layout, LayoutNode, LayoutStyle, Length, Lifecycle,
    NativeWidget, Overflow, Padding, Platform, Pod, Position, Sides, Size, WidgetView,
    native::{NativeGroup, NativeScroll},
};

/// [`View`] that can display a large scrollable list.
pub fn list<T, V>(count: usize, build: impl Fn(&T, usize) -> V) -> List<impl Fn(&T, usize) -> V> {
    List::new(count, build)
}

/// [`View`] that can display a large scrollable list.
pub struct List<F> {
    layout:    LayoutStyle,
    direction: Direction,
    padding:   Sides<Length>,
    gap:       f32,
    min_views: usize,
    buffer:    usize,
    count:     usize,
    build:     F,
}

impl<F> List<F> {
    /// Create new [`List`].
    pub fn new(count: usize, build: F) -> Self {
        Self {
            layout: LayoutStyle::default(),
            direction: Direction::Column,
            padding: Sides::all(Length::Length(0.0)),
            gap: 0.0,
            min_views: 10,
            buffer: 5,
            count,
            build,
        }
    }

    /// Set the direction.
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Set the gap between items.
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Set the minimum number of active views.
    pub fn min_views(mut self, min_views: usize) -> Self {
        self.min_views = min_views;
        self
    }

    /// Set the number of buffer views at the start and end.
    pub fn buffer(mut self, buffer: usize) -> Self {
        self.buffer = buffer;
        self
    }
}

impl<F> Layout for List<F> {
    fn get_layout_style_mut(&mut self) -> &mut LayoutStyle {
        &mut self.layout
    }
}

impl<F> Padding for List<F> {
    fn get_padding_mut(&mut self) -> &mut Sides<Length> {
        &mut self.padding
    }
}

struct ListMessage(f32, f32);

impl<F> ViewMarker for List<F> {}
impl<P, T, F, V> View<Context<P>, T> for List<F>
where
    P: Platform,
    F: Fn(&T, usize) -> V,
    V: WidgetView<P, T>,
{
    type Element = Pod<P, P::Scroll>;
    type State = ListState<P, T, F, V>;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        // build the content group widget
        let group = P::Group::build(&mut cx.platform);

        // build the scroll widget
        let mut scroll = P::Scroll::build(&mut cx.platform, group.widget());
        scroll.set_direction(&mut cx.platform, self.direction);

        // add the contents layout node
        let node = cx.layout.add_node(&[]);
        cx.layout.set_flex(node, content_flex(self.direction));
        cx.layout.set_layout(
            node,
            content_layout(self.direction, 0.0),
        );

        // add the scroll layout node
        let scroll_node = cx.layout.add_node(&[node]);
        cx.layout.set_layout(scroll_node, self.layout);
        cx.layout.set_flex(scroll_node, scroll_flex(self.direction));
        cx.layout.set_padding(scroll_node, self.padding);
        cx.layout.set_overflow(
            scroll_node,
            scroll_overflow(self.direction),
        );

        // register on scroll callback
        let view_id = ViewId::next();
        cx.register(view_id);

        let proxy = cx.proxy();
        scroll.set_on_scroll(&mut cx.platform, move |x, y| {
            proxy.message(Message::new(ListMessage(x, y), view_id));
        });

        let pod = Pod::new(scroll_node, scroll);

        let mut state = ListState {
            view_id,

            direction: self.direction,
            layout: self.layout,
            padding: self.padding,
            gap: self.gap,

            node,
            scroll_allocation: None,
            content_allocation: None,
            group,

            sizes: vec![None; self.count],
            window_size: 0.0,
            average_size: 0.0,
            content_size: 0.0,

            scroll: 0.0,
            start: 0,
            views: VecDeque::new(),

            min_views: self.min_views,
            buffer: self.buffer,
            count: self.count,
            build: self.build,
        };

        // initialize active views
        let count = state.compute_active_view_count(0);
        state.build_active_views(cx, data, count);

        (pod, state)
    }

    fn rebuild(
        self,
        element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        if state.direction != self.direction {
            state.direction = self.direction;

            // set direction of scroll
            (element.widget).set_direction(&mut cx.platform, state.direction);

            // set layout direction of scroll
            cx.layout.set_flex(
                *element.node,
                scroll_flex(state.direction),
            );

            // set layout overflow of scroll
            cx.layout.set_overflow(
                *element.node,
                scroll_overflow(state.direction),
            );

            // set content layout
            cx.layout.set_layout(
                state.node,
                content_layout(state.direction, state.content_size),
            );

            // set content flex
            cx.layout.set_flex(
                state.node,
                content_flex(state.direction),
            );

            // set the layout of each child
            for child in state.views.iter() {
                cx.layout.set_flex(child.node, child_flex(state.direction));
                cx.layout.set_layout(
                    child.node,
                    child_layout(state.direction),
                );
            }
        }

        if state.layout != self.layout {
            state.layout = self.layout;
            cx.layout.set_layout(*element.node, state.layout);
        }

        if state.padding != self.padding {
            state.padding = self.padding;
            cx.layout.set_padding(*element.node, state.padding);
        }

        state.gap = self.gap;
        state.min_views = self.min_views;
        state.buffer = self.buffer;
        state.build = self.build;
        state.count = self.count;

        state.sizes.resize(self.count, None);

        if state.layout_changed(cx) {
            state.update_average_size();
            state.update_content_size(cx);
        }

        state.update_active_views(cx, data);
        state.rebuild_active_views(cx, data);
        state.layout_active_views(cx);
    }

    fn message(
        element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        if let Some(Lifecycle::Layout) = message.get() {
            if let Some(allocation) = cx.layout.get_allocation(*element.node) {
                state.window_size = match state.direction {
                    Direction::Row => allocation.size.width,
                    Direction::Column => allocation.size.height,
                };
            }

            // check for layout changes
            if state.layout_changed(cx) {
                state.update_average_size();
                state.update_content_size(cx);
            }

            // update layout of scroll contents
            if let Some(allocation) = cx.layout.get_allocation(state.node)
                && state.content_allocation != Some(allocation)
            {
                element.widget.set_content_layout(
                    &mut cx.platform,
                    allocation.x,
                    allocation.y,
                    allocation.content_size.width,
                    allocation.content_size.height,
                );

                state.update_active_views(cx, data);
            }

            if let Some(allocation) = cx.layout.get_allocation(*element.node)
                && state.scroll_allocation != Some(allocation)
            {
                element.widget.set_content_size(
                    &mut cx.platform,
                    allocation.content_size.width,
                    allocation.content_size.height,
                );
            }

            state.scroll_allocation = cx.layout.get_allocation(*element.node);
            state.content_allocation = cx.layout.get_allocation(state.node);

            // layout the active views
            state.layout_active_views(cx);
        }

        if let Some(ListMessage(x, y)) = message.take_targeted(state.view_id) {
            state.scroll = match state.direction {
                Direction::Row => x,
                Direction::Column => y,
            };

            if state.layout_changed(cx) {
                state.update_average_size();
                state.update_content_size(cx);
            }

            state.update_active_views(cx, data);
            state.layout_active_views(cx);

            return Action::new();
        }

        let mut action = Action::new();

        for (i, child) in state.views.iter_mut().enumerate() {
            let pod = child.element.as_mut(child.node, 0, &mut state.group, i);
            action |= V::message(pod, &mut child.state, cx, data, message);
        }

        action
    }

    fn teardown(element: Self::Element, mut state: Self::State, cx: &mut Context<P>) {
        while !state.views.is_empty() {
            state.teardown_active_view(cx, state.views.len() - 1);
        }

        state.group.teardown(&mut cx.platform);
        element.widget.teardown(&mut cx.platform);
        cx.layout.remove_node(element.node);
        cx.unregister(state.view_id);
    }
}

pub struct ListState<P, T, F, V>
where
    P: Platform,
    F: Fn(&T, usize) -> V,
    V: WidgetView<P, T>,
{
    view_id: ViewId,

    direction: Direction,
    layout:    LayoutStyle,
    padding:   Sides<Length>,
    gap:       f32,

    node:               LayoutNode,
    scroll_allocation:  Option<Allocation>,
    content_allocation: Option<Allocation>,
    group:              P::Group,

    sizes:        Vec<Option<f32>>,
    window_size:  f32,
    average_size: f32,
    content_size: f32,

    scroll: f32,
    start:  usize,
    views:  VecDeque<ListChild<P, T, V>>,

    min_views: usize,
    buffer:    usize,
    count:     usize,
    build:     F,
}

struct ListChild<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    node:       LayoutNode,
    offset:     f32,
    allocation: Option<Allocation>,
    element:    V::Element,
    state:      V::State,
}

impl<P, T, F, V> ListState<P, T, F, V>
where
    P: Platform,
    F: Fn(&T, usize) -> V,
    V: WidgetView<P, T>,
{
    fn compute_average_size(&self) -> f32 {
        let size_sum = self.sizes.iter().flatten().copied().sum::<f32>();
        let size_count = self.sizes.iter().flatten().count() as f32;

        size_sum / size_count
    }

    fn compute_active_view_offset(&self) -> f32 {
        let gap_sum = self.gap * self.start as f32;
        let size_sum = (0..self.start)
            .map(|i| self.get_size_estimate(i))
            .sum::<f32>();

        size_sum + gap_sum
    }

    fn compute_content_size(&self) -> f32 {
        let gap_sum = self.gap * self.count.saturating_sub(1) as f32;
        let size_sum = (0..self.count)
            .map(|i| self.get_size_estimate(i))
            .sum::<f32>();

        size_sum + gap_sum
    }

    fn update_average_size(&mut self) {
        self.average_size = self.compute_average_size();
    }

    fn get_size_estimate(&self, index: usize) -> f32 {
        self.sizes[index].unwrap_or(self.average_size)
    }

    fn update_content_size(&mut self, cx: &mut Context<P>) {
        let content_size = self.compute_content_size();

        if self.content_size != content_size {
            self.content_size = content_size;
            cx.layout.set_layout(
                self.node,
                content_layout(self.direction, content_size),
            );
        }
    }

    fn update_active_views(&mut self, cx: &mut Context<P>, data: &mut T) {
        let start = self.compute_start_index();
        let count = self.compute_active_view_count(start);

        if self.start == start && self.views.len() == count {
            return;
        }

        // if the difference is too big, don't bother reusing views
        if self.start.abs_diff(start) >= count {
            self.start = start;
            self.truncate_active_views(cx, count);
            self.rebuild_active_views(cx, data);
            self.build_active_views(cx, data, count);
            return;
        }

        while self.views.len() > count {
            if self.start < start {
                self.teardown_active_view(cx, 0);
                self.start += 1;
            } else {
                self.teardown_active_view(cx, self.views.len() - 1);
            }
        }

        while self.views.len() < count {
            if self.start > start {
                self.start -= 1;
                self.build_active_view(cx, data, 0);
            } else {
                self.build_active_view(cx, data, self.views.len());
            }
        }

        while self.start > start {
            self.start -= 1;
            self.rotate_backward(cx, data);
        }

        while self.start < start {
            self.rotate_forward(cx, data);
            self.start += 1;
        }
    }

    fn rotate_backward(&mut self, cx: &mut Context<P>, data: &mut T) {
        let Some(mut child) = self.views.pop_back() else {
            return;
        };

        child.allocation = None;

        let widget = child.element.widget.widget();
        let index = self.views.len();

        self.group.remove_child(&mut cx.platform, index);
        self.group.insert_child(&mut cx.platform, 0, widget);

        cx.layout.remove_child(self.node, index);
        cx.layout.insert_child(self.node, 0, child.node);

        let pod = child.element.as_mut(child.node, 0, &mut self.group, 0);
        let view = (self.build)(data, self.start);
        view.rebuild(pod, &mut child.state, cx, data);

        self.views.push_front(child);
    }

    fn rotate_forward(&mut self, cx: &mut Context<P>, data: &mut T) {
        let Some(mut child) = self.views.pop_front() else {
            return;
        };

        child.allocation = None;

        let widget = child.element.widget.widget();
        let index = self.views.len();

        self.group.remove_child(&mut cx.platform, 0);
        self.group.insert_child(&mut cx.platform, index, widget);

        cx.layout.remove_child(self.node, 0);
        cx.layout.insert_child(self.node, index, child.node);

        let pod = child.element.as_mut(child.node, 0, &mut self.group, index);
        let view = (self.build)(data, self.start + index + 1);
        view.rebuild(pod, &mut child.state, cx, data);

        self.views.push_back(child);
    }

    fn compute_start_index(&mut self) -> usize {
        let mut offset = 0.0;

        for i in 0..self.count {
            offset += self.get_size_estimate(i) + self.gap;

            if offset >= self.scroll {
                return i.saturating_sub(self.buffer);
            }
        }

        self.count.saturating_sub(self.buffer)
    }

    fn compute_active_view_count(&self, start: usize) -> usize {
        let mut offset = self.compute_active_view_offset();
        let remaining = self.count.saturating_sub(start);

        for i in self.start..self.count {
            if offset >= self.scroll + self.window_size {
                let size = (i - self.start).max(self.min_views);
                return remaining.min(size + self.buffer * 2);
            }

            offset += self.get_size_estimate(i) + self.gap;
        }

        remaining
    }

    fn allocation_size(direction: Direction, allocation: Allocation) -> f32 {
        match direction {
            Direction::Row => {
                allocation.size.width + allocation.margin.left + allocation.margin.right
            }

            Direction::Column => {
                allocation.size.height + allocation.margin.top + allocation.margin.bottom
            }
        }
    }

    fn layout_changed(&mut self, cx: &mut Context<P>) -> bool {
        let mut changed = false;

        for (i, child) in self.views.iter_mut().enumerate() {
            if let Some(allocation) = cx.layout.get_allocation(child.element.node) {
                let size = Self::allocation_size(self.direction, allocation);

                if self.sizes[self.start + i] != Some(size) {
                    self.sizes[self.start + i] = Some(size);
                    changed = true;
                }
            }
        }

        changed
    }

    fn layout_active_views(&mut self, cx: &mut Context<P>) {
        let mut offset = self.compute_active_view_offset();

        for (i, child) in self.views.iter_mut().enumerate() {
            if let Some(allocation) = cx.layout.get_allocation(child.element.node) {
                let size = Self::allocation_size(self.direction, allocation);

                if child.allocation != Some(allocation) || child.offset != offset {
                    child.allocation = Some(allocation);
                    child.offset = offset;

                    let x_offset = match self.direction {
                        Direction::Row => offset,
                        Direction::Column => 0.0,
                    };

                    let y_offset = match self.direction {
                        Direction::Row => 0.0,
                        Direction::Column => offset,
                    };

                    self.group.set_child_layout(
                        &mut cx.platform,
                        i,
                        allocation.x + x_offset,
                        allocation.y + y_offset,
                        allocation.size.width,
                        allocation.size.height,
                    );
                }

                offset += size + self.gap;
            }
        }
    }

    fn rebuild_active_views(&mut self, cx: &mut Context<P>, data: &mut T) {
        for (i, child) in self.views.iter_mut().enumerate() {
            let pod = child.element.as_mut(child.node, 0, &mut self.group, i);
            let view = (self.build)(data, self.start + i);
            view.rebuild(pod, &mut child.state, cx, data);
        }
    }

    fn build_active_views(&mut self, cx: &mut Context<P>, data: &mut T, count: usize) {
        for index in self.views.len()..count {
            self.build_active_view(cx, data, index);
        }
    }

    fn truncate_active_views(&mut self, cx: &mut Context<P>, count: usize) {
        while self.views.len() > count {
            self.teardown_active_view(cx, self.views.len() - 1);
        }
    }

    fn build_active_view(&mut self, cx: &mut Context<P>, data: &mut T, index: usize) {
        let view = (self.build)(data, self.start + index);
        let (element, state) = view.build(cx, data);

        self.group.insert_child(
            &mut cx.platform,
            index,
            element.widget.widget(),
        );

        let node = cx.layout.add_node(&[element.node]);
        cx.layout.set_layout(node, child_layout(self.direction));
        cx.layout.set_flex(node, child_flex(self.direction));

        cx.layout.insert_child(self.node, index, node);

        let child = ListChild {
            node,
            offset: 0.0,
            allocation: None,
            element,
            state,
        };

        self.views.insert(index, child);
    }

    fn teardown_active_view(&mut self, cx: &mut Context<P>, index: usize) {
        if let Some(child) = self.views.remove(index) {
            self.group.remove_child(&mut cx.platform, index);

            V::teardown(child.element, child.state, cx);
            cx.layout.remove_node(child.node);
        }
    }
}

fn content_layout(direction: Direction, size: f32) -> LayoutStyle {
    let size = match direction {
        Direction::Row => Size {
            width:  Some(Length::Length(size)),
            height: Some(Length::Fract(1.0)),
        },

        Direction::Column => Size {
            width:  Some(Length::Fract(1.0)),
            height: Some(Length::Length(size)),
        },
    };

    LayoutStyle {
        min_size: size,
        max_size: size,
        ..Default::default()
    }
}

fn content_flex(direction: Direction) -> FlexStyle {
    FlexStyle {
        direction,
        ..Default::default()
    }
}

fn scroll_overflow(direction: Direction) -> Size<Overflow> {
    match direction {
        Direction::Row => Size {
            width:  Overflow::Hidden,
            height: Overflow::Visible,
        },

        Direction::Column => Size {
            width:  Overflow::Visible,
            height: Overflow::Hidden,
        },
    }
}

fn scroll_flex(direction: Direction) -> FlexStyle {
    FlexStyle {
        direction,
        ..Default::default()
    }
}

fn child_layout(direction: Direction) -> LayoutStyle {
    let inset = match direction {
        Direction::Row => Sides {
            left:   None,
            right:  None,
            top:    Some(Length::Length(0.0)),
            bottom: Some(Length::Length(0.0)),
        },

        Direction::Column => Sides {
            left:   Some(Length::Length(0.0)),
            right:  Some(Length::Length(0.0)),
            top:    None,
            bottom: None,
        },
    };

    LayoutStyle {
        position: Position::Absolute,
        inset,
        ..Default::default()
    }
}

fn child_flex(direction: Direction) -> FlexStyle {
    FlexStyle {
        direction,
        ..Default::default()
    }
}
