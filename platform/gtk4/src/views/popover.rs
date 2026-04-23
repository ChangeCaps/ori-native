use std::time::Duration;

use glib::{
    object::{Cast, IsA},
    subclass::types::ObjectSubclassIsExt,
};
use gtk4::prelude::{FixedExt, PopoverExt, WidgetExt};
use ori::{Action, Element, Message, Mut, View, ViewMarker};
use ori_native_core::{
    AvailableSpace, Context, LayoutNode, Parent, Size, Widget, WidgetMut, WidgetView,
};

use crate::Platform;

pub fn popover<T, V, P>(contents: V, popover: P) -> Popover<T, V, P> {
    Popover::new(contents, popover)
}

pub struct Popover<T, V, P> {
    contents: V,
    popover:  P,
    position: Position,
    is_open:  bool,
    on_close: Box<dyn FnMut(&mut T) -> Action>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

impl<T, V, P> Popover<T, V, P> {
    pub fn new(contents: V, popover: P) -> Self {
        Self {
            contents,
            popover,
            position: Position::Bottom,
            is_open: false,
            on_close: Box::new(|_| Action::new()),
        }
    }

    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    pub fn position(mut self, position: Position) -> Self {
        self.position = position;
        self
    }

    pub fn on_close<A>(mut self, mut on_close: impl FnMut(&mut T) -> A + 'static) -> Self
    where
        A: Into<Action>,
    {
        self.on_close = Box::new(move |data| on_close(data).into());
        self
    }
}

impl<T, V, P> ViewMarker for Popover<T, V, P> {}
impl<T, V, P> View<Context<Platform>, T> for Popover<T, V, P>
where
    V: WidgetView<Platform, T>,
    P: WidgetView<Platform, T>,
{
    type Element = PopoverWidget<V::Element, P::Element>;
    type State = PopoverState<T, V, P>;

    fn build(self, cx: &mut Context<Platform>, data: &mut T) -> (Self::Element, Self::State) {
        let (contents_element, contents_state) = self.contents.build(cx, data);
        let (popover_element, popover_state) = self.popover.build(cx, data);

        let widget = PopoverWidget::new(cx, contents_element, popover_element);
        widget.receiver.set_position(self.position);

        let state = PopoverState {
            contents: contents_state,
            popover:  popover_state,
            position: self.position,
        };

        (widget, state)
    }

    fn rebuild(
        self,
        element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<Platform>,
        data: &mut T,
    ) {
        let widget = WidgetMut::new(
            element.parent,
            &mut element.widget.contents,
        );

        self.contents.rebuild(widget, &mut state.contents, cx, data);

        let mut parent = PopoverParent {
            receiver: &mut element.widget.receiver,
            layout:   element.widget.layout,
        };

        let widget = WidgetMut::new(&mut parent, &mut element.widget.popover);
        self.popover.rebuild(widget, &mut state.popover, cx, data);

        element.receiver.set_open(self.is_open);

        if state.position != self.position {
            state.position = self.position;
            element.receiver.set_position(self.position);
        }
    }

    fn message(
        element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<Platform>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        let mut action = Action::new();

        let widget = WidgetMut::new(
            element.parent,
            &mut element.widget.contents,
        );

        action |= V::message(
            widget,
            &mut state.contents,
            cx,
            data,
            message,
        );

        let mut parent = PopoverParent {
            receiver: &mut element.widget.receiver,
            layout:   element.widget.layout,
        };

        let widget = WidgetMut::new(&mut parent, &mut element.widget.popover);
        action |= P::message(
            widget,
            &mut state.popover,
            cx,
            data,
            message,
        );

        action
    }

    fn teardown(element: Self::Element, state: Self::State, cx: &mut Context<Platform>) {
        let (contents, popover) = element.teardown(cx);
        V::teardown(contents, state.contents, cx);
        P::teardown(popover, state.popover, cx);
    }
}

pub struct PopoverState<T, V, P>
where
    V: WidgetView<Platform, T>,
    P: WidgetView<Platform, T>,
{
    contents: V::State,
    popover:  P::State,
    position: Position,
}

pub struct PopoverWidget<T, U>
where
    T: Widget<Platform>,
    U: Widget<Platform>,
{
    receiver: PopoverReceiver,
    contents: T,
    popover:  U,
    layout:   LayoutNode,
}

impl<T, U> PopoverWidget<T, U>
where
    T: Widget<Platform>,
    U: Widget<Platform>,
{
    pub fn new(cx: &mut Context<Platform>, contents: T, popover: U) -> Self {
        let receiver = PopoverReceiver::new();
        receiver.set_child(Some(&contents.widget_ref()));
        receiver.set_popover_child(Some(&popover.widget_ref()));

        let layout = cx.layout.add_node(&[popover.layout_node()]);

        Self {
            receiver,
            contents,
            popover,
            layout,
        }
    }

    pub fn teardown(self, cx: &mut Context<Platform>) -> (T, U) {
        cx.layout.remove_node(self.layout);
        (self.contents, self.popover)
    }
}

impl<T, U> Element for PopoverWidget<T, U>
where
    T: Widget<Platform>,
    U: Widget<Platform>,
{
    type Mut<'a>
        = WidgetMut<'a, Platform, Self>
    where
        Self: 'a;
}

impl<T, U> Widget<Platform> for PopoverWidget<T, U>
where
    T: Widget<Platform>,
    U: Widget<Platform>,
{
    fn widget_ref(&self) -> gtk4::Widget {
        self.receiver.clone().upcast()
    }

    fn layout_node(&self) -> LayoutNode {
        self.contents.layout_node()
    }

    fn layout(&mut self, cx: &mut Context<Platform>) {
        let space = Size {
            width:  AvailableSpace::MaxContent,
            height: AvailableSpace::MaxContent,
        };

        (cx.layout).compute_layout(&mut cx.platform, self.layout, space);

        if let Some(allocation) = cx.layout.get_allocation(self.layout) {
            self.receiver.set_popover_size(
                allocation.size.width,
                allocation.size.height,
            );
        }

        if let Some(allocation) = cx.layout.get_allocation(self.popover.layout_node()) {
            self.receiver.set_content_layout(
                allocation.x,
                allocation.y,
                allocation.size.width,
                allocation.size.height,
            );
        }

        self.contents.layout(cx);
        self.popover.layout(cx);
    }

    fn animate(&mut self, cx: &mut Context<Platform>, dt: Duration) {
        self.contents.animate(cx, dt);
        self.popover.animate(cx, dt);
    }
}

struct PopoverParent<'a> {
    receiver: &'a mut PopoverReceiver,
    layout:   LayoutNode,
}

impl<'a> Parent<Platform> for PopoverParent<'a> {
    fn replace_child(
        &mut self,
        cx: &mut Context<Platform>,
        widget: gtk4::Widget,
        layout: LayoutNode,
    ) {
        self.receiver.set_child(Some(&widget));
        cx.layout.replace_child(self.layout, 0, layout);
    }
}

impl PopoverReceiver {
    pub fn new() -> Self {
        let this: Self = glib::Object::new();
        this.imp().popover.set_parent(&this);
        this
    }

    pub fn set_child(&self, child: Option<&impl IsA<gtk4::Widget>>) {
        if let Some(ref child) = *self.imp().widget.borrow() {
            child.unparent();
        }

        if let Some(child) = child {
            child.set_parent(self);
            self.imp().widget.replace(Some(child.as_ref().clone()));
        }
    }

    pub fn set_popover_child(&self, popover: Option<&impl IsA<gtk4::Widget>>) {
        if let Some(child) = self.imp().fixed.first_child() {
            self.imp().fixed.remove(&child);
        }

        if let Some(popover) = popover {
            self.imp().fixed.put(popover, 0.0, 0.0);
        }
    }

    pub fn set_open(&self, is_open: bool) {
        if self.imp().popover.is_visible() != is_open {
            match is_open {
                true => self.imp().popover.popup(),
                false => self.imp().popover.popdown(),
            }
        }
    }

    pub fn set_position(&self, position: Position) {
        let position = match position {
            Position::Top => gtk4::PositionType::Top,
            Position::Right => gtk4::PositionType::Right,
            Position::Bottom => gtk4::PositionType::Bottom,
            Position::Left => gtk4::PositionType::Left,
        };

        self.imp().popover.set_position(position);
    }

    pub fn set_popover_size(&self, width: f32, height: f32) {
        self.imp().popover.set_size_request(
            width.round() as i32,
            height.round() as i32,
        );
    }

    pub fn set_content_layout(&self, x: f32, y: f32, width: f32, height: f32) {
        if let Some(child) = self.imp().fixed.first_child() {
            self.imp().fixed.move_(&child, x as f64, y as f64);
            child.set_size_request(
                width.round() as i32,
                height.round() as i32,
            );
        }
    }
}

glib::wrapper! {
    pub struct PopoverReceiver(
        ObjectSubclass<imp::PopoverReceiver>)
        @extends
            gtk4::Widget,
        @implements
            gtk4::Buildable,
            gtk4::Accessible,
            gtk4::ConstraintTarget;
}

mod imp {
    use std::cell::RefCell;

    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use gtk4::{
        prelude::{PopoverExt, WidgetExt},
        subclass::widget::WidgetImpl,
    };

    pub struct PopoverReceiver {
        pub(super) widget:  RefCell<Option<gtk4::Widget>>,
        pub(super) popover: gtk4::Popover,
        pub(super) fixed:   gtk4::Fixed,
    }

    impl Default for PopoverReceiver {
        fn default() -> Self {
            let fixed = gtk4::Fixed::new();
            let popover = gtk4::Popover::new();
            popover.set_autohide(false);
            popover.set_has_arrow(false);
            popover.set_child(Some(&fixed));

            Self {
                widget: Default::default(),
                popover,
                fixed,
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PopoverReceiver {
        const NAME: &'static str = "OriPopupReceiver";
        type Type = super::PopoverReceiver;
        type ParentType = gtk4::Widget;
    }

    impl ObjectImpl for PopoverReceiver {
        fn dispose(&self) {
            if let Some(ref child) = *self.widget.borrow() {
                child.unparent();
            }

            self.popover.unparent();
        }
    }

    impl WidgetImpl for PopoverReceiver {
        fn measure(&self, orientation: gtk4::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            match *self.widget.borrow() {
                Some(ref child) => child.measure(orientation, for_size),
                None => (0, 0, -1, -1),
            }
        }

        fn size_allocate(&self, width: i32, height: i32, _baseline: i32) {
            if let Some(ref child) = *self.widget.borrow() {
                let allocation = gtk4::Allocation::new(0, 0, width, height);
                child.size_allocate(&allocation, -1);
            }

            self.popover.present();
        }
    }
}
