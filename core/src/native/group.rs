use ori::{Elements, Mut};

use crate::{
    Allocation, BoxedWidget, Color, Context, Corners, LayoutNode, NativeWidget, Overflow, Platform,
    Shadow, Sides, Unsupported, element::NativeParent, platform::unsupported,
};

/// A native group widget.
///
/// A group is a widget with multiple children, a background, border and a shadow.
pub trait NativeGroup<P>: NativeWidget<P> + NativeParent<P>
where
    P: Platform,
{
    /// Build the widget.
    fn build(platform: &mut P) -> Self;

    /// Teardown the widget.
    fn teardown(self, platform: &mut P);

    /// Insert a `child` at `index`.
    fn insert_child(&mut self, platform: &mut P, index: usize, child: &P::WidgetRef);

    /// Remove the child at `index`.
    fn remove_child(&mut self, platform: &mut P, index: usize);

    /// Swap the order of children at `index_a` and `index_b`.
    fn swap_children(&mut self, platform: &mut P, index_a: usize, index_b: usize);

    /// Set the layout rect of the child at `index`.
    fn set_child_layout(
        &mut self,
        platform: &mut P,
        index: usize,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    );

    /// Set the fill `color` of the background.
    fn set_background_color(&mut self, platform: &mut P, color: Color);

    /// Set the stroke `color` of the border.
    fn set_border_color(&mut self, platform: &mut P, color: Color);

    /// Set the sidewise `widths` of the border.
    fn set_border_width(&mut self, platform: &mut P, widths: Sides<f32>);

    /// Set the radii of each corner.
    fn set_corners(&mut self, platform: &mut P, corners: Corners<f32>);

    /// Set the `overflow` mode.
    fn set_overflow(&mut self, platform: &mut P, overflow: Overflow);

    /// Set the `shadow` drawn behind the background.
    fn set_shadow(&mut self, platform: &mut P, shadow: Shadow);

    /* platform specific */

    /// Set whether to use a hardware layer on `android`.
    fn set_hardware_layer(&mut self, platform: &mut P, enabled: bool) {
        let _ = platform;
        let _ = enabled;
    }
}

impl<P> NativeGroup<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P) -> Self {
        unsupported!("group widget")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn insert_child(&mut self, _platform: &mut P, _index: usize, _child: &P::WidgetRef) {
        unreachable!()
    }

    fn remove_child(&mut self, _platform: &mut P, _index: usize) {
        unreachable!()
    }

    fn swap_children(&mut self, _platform: &mut P, _index_a: usize, _index_b: usize) {
        unreachable!()
    }

    fn set_child_layout(
        &mut self,
        _platform: &mut P,
        _index: usize,
        _x: f32,
        _y: f32,
        _width: f32,
        _height: f32,
    ) {
        unreachable!()
    }

    fn set_background_color(&mut self, _platform: &mut P, _color: Color) {
        unreachable!()
    }

    fn set_border_color(&mut self, _platform: &mut P, _color: Color) {
        unreachable!()
    }

    fn set_border_width(&mut self, _platform: &mut P, _width: Sides<f32>) {
        unreachable!()
    }

    fn set_corners(&mut self, _platform: &mut P, _radii: Corners<f32>) {
        unreachable!()
    }

    fn set_overflow(&mut self, _platform: &mut P, _overflow: Overflow) {
        unreachable!()
    }

    fn set_shadow(&mut self, _platform: &mut P, _shadow: Shadow) {
        unreachable!()
    }
}

/// A utility wrapper for a [`NativeGroup`], maintaining an [`Elements`].
pub struct Group<P>
where
    P: Platform,
{
    group:        P::Group,
    children:     Vec<Child<P>>,
    border_width: Sides<f32>,
}

struct Child<P> {
    element:    BoxedWidget<P>,
    allocation: Option<Allocation>,
}

impl<P> Group<P>
where
    P: Platform,
{
    /// Create new [`Group`].
    pub fn new(cx: &mut Context<P>) -> Self {
        Self {
            group:        P::Group::build(&mut cx.platform),
            children:     Vec::new(),
            border_width: Sides::all(0.0),
        }
    }

    /// Teardown the wrapped [`NativeGroup`].
    pub fn teardown(self, cx: &mut Context<P>) {
        self.group.teardown(&mut cx.platform);
    }

    /// Get the [`Elements`].
    pub fn elements(&mut self, node: LayoutNode) -> impl Elements<Context<P>, BoxedWidget<P>> {
        GroupElements {
            layout:   node,
            index:    0,
            group:    &mut self.group,
            children: &mut self.children,
        }
    }

    /// Set the background `color`.
    pub fn set_background(&mut self, cx: &mut Context<P>, color: Color) {
        self.group.set_background_color(&mut cx.platform, color);
    }

    /// Set the border `color`.
    pub fn set_border_color(&mut self, cx: &mut Context<P>, color: Color) {
        self.group.set_border_color(&mut cx.platform, color);
    }

    /// Set the corner radii.
    pub fn set_corners(&mut self, cx: &mut Context<P>, corners: Corners<f32>) {
        self.group.set_corners(&mut cx.platform, corners);
    }

    /// Set the `overflow` mode.
    pub fn set_overflow(&mut self, cx: &mut Context<P>, overflow: Overflow) {
        self.group.set_overflow(&mut cx.platform, overflow);
    }

    /// Set the `shadow`.
    pub fn set_shadow(&mut self, cx: &mut Context<P>, shadow: Shadow) {
        self.group.set_shadow(&mut cx.platform, shadow);
    }

    /// Set whether hardware layer is enabled.
    pub fn set_hardware_layer(&mut self, cx: &mut Context<P>, enabled: bool) {
        self.group.set_hardware_layer(&mut cx.platform, enabled);
    }

    /// Perform layout on the group.
    pub fn layout(&mut self, cx: &mut Context<P>, node: LayoutNode) {
        if let Some(allocation) = cx.layout.get_allocation(node)
            && self.border_width != allocation.border
        {
            self.border_width = allocation.border;
            (self.group).set_border_width(&mut cx.platform, allocation.border);
        }

        for (index, child) in self.children.iter_mut().enumerate() {
            if let Some(allocation) = cx.layout.get_allocation(child.element.layout)
                && child.allocation != Some(allocation)
            {
                child.allocation = Some(allocation);
                self.group.set_child_layout(
                    &mut cx.platform,
                    index,
                    allocation.x,
                    allocation.y,
                    allocation.size.width,
                    allocation.size.height,
                );
            }
        }
    }
}

impl<P> NativeWidget<P> for Group<P>
where
    P: Platform,
{
    fn widget_ref(&self) -> &P::WidgetRef {
        self.group.widget_ref()
    }
}

struct GroupElements<'a, P>
where
    P: Platform,
{
    layout:   LayoutNode,
    index:    usize,
    group:    &'a mut P::Group,
    children: &'a mut Vec<Child<P>>,
}

impl<P> Elements<Context<P>, BoxedWidget<P>> for GroupElements<'_, P>
where
    P: Platform,
{
    fn next(&mut self, _cx: &mut Context<P>) -> Option<Mut<'_, BoxedWidget<P>>> {
        let child = self.children.get_mut(self.index)?;
        let pod = child.element.as_mut(
            self.layout,
            self.index,
            self.group,
            self.index,
        );

        self.index += 1;
        Some(pod)
    }

    fn insert(&mut self, cx: &mut Context<P>, element: BoxedWidget<P>) {
        (cx.layout).insert_child(self.layout, self.index, element.layout);

        self.group.insert_child(
            &mut cx.platform,
            self.index,
            element.widget.widget_ref(),
        );

        self.children.insert(
            self.index,
            Child {
                element,
                allocation: None,
            },
        );

        self.index += 1;
    }

    fn remove(&mut self, cx: &mut Context<P>) -> Option<BoxedWidget<P>> {
        let child = self.children.remove(self.index);

        self.group.remove_child(&mut cx.platform, self.index);
        cx.layout.remove_child(self.layout, self.index);

        Some(child.element)
    }

    fn swap(&mut self, cx: &mut Context<P>, offset: usize) {
        cx.layout.replace_child(
            self.layout,
            self.index,
            self.children[self.index + offset].element.layout,
        );

        cx.layout.replace_child(
            self.layout,
            self.index + offset,
            self.children[self.index].element.layout,
        );

        self.group.swap_children(
            &mut cx.platform,
            self.index,
            self.index + offset,
        );

        self.children.swap(self.index, self.index + offset);
    }
}
