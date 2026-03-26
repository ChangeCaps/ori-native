use glib::{
    object::{Cast, IsA},
    subclass::types::ObjectSubclassIsExt,
};
use gtk4::prelude::{FixedExt, PopoverExt, WidgetExt};
use ori::{Action, Message, Mut, View, ViewMarker};
use ori_native_core::{
    AvailableSpace, Context, LayoutNode, Lifecycle, NativeParent, NativeWidget, Pod, Size,
    WidgetView,
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
    type Element = Pod<Platform, PopoverReceiver>;
    type State = PopoverState<T, V, P>;

    fn build(self, cx: &mut Context<Platform>, data: &mut T) -> (Self::Element, Self::State) {
        let (contents_element, contents_state) = self.contents.build(cx, data);
        let (popover_element, popover_state) = self.popover.build(cx, data);

        let receiver = PopoverReceiver::new();
        receiver.set_child(Some(contents_element.widget.widget()));
        receiver.set_popover_child(Some(popover_element.widget.widget()));
        receiver.set_position(self.position);
        receiver.set_open(self.is_open);

        let popover_node = cx.layout.add_node(&[popover_element.node]);

        let pod = Pod::new(contents_element.node, receiver);
        let state = PopoverState {
            contents_widget: contents_element.widget,
            contents_state,
            popover_element,
            popover_state,
            popover_node,
            position: self.position,
        };

        (pod, state)
    }

    fn rebuild(
        self,
        mut element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<Platform>,
        data: &mut T,
    ) {
        let pod = element.map_widget(&mut state.contents_widget);
        (self.contents).rebuild(pod, &mut state.contents_state, cx, data);

        let pod = state
            .popover_element
            .as_mut(state.popover_node, element.widget, 1);

        (self.popover).rebuild(pod, &mut state.popover_state, cx, data);

        element.widget.set_open(self.is_open);

        if state.position != self.position {
            state.position = self.position;
            element.widget.set_position(self.position);
        }
    }

    fn message(
        mut element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<Platform>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        let mut action = Action::new();

        if let Some(Lifecycle::Layout) = message.get() {
            let space = Size {
                width:  AvailableSpace::MaxContent,
                height: AvailableSpace::MaxContent,
            };

            cx.layout.compute_layout(
                &mut cx.platform,
                state.popover_node,
                space,
            );

            if let Some(allocation) = cx.layout.get_allocation(state.popover_node) {
                element.widget.set_popover_size(
                    allocation.size.width,
                    allocation.size.height,
                );
            }

            if let Some(allocation) = cx.layout.get_allocation(state.popover_element.node) {
                element.widget.set_content_layout(
                    allocation.x,
                    allocation.y,
                    allocation.size.width,
                    allocation.size.height,
                );
            }
        }

        let pod = element.map_widget(&mut state.contents_widget);
        action |= V::message(
            pod,
            &mut state.contents_state,
            cx,
            data,
            message,
        );

        let pod = state
            .popover_element
            .as_mut(state.popover_node, element.widget, 1);

        action |= P::message(
            pod,
            &mut state.popover_state,
            cx,
            data,
            message,
        );

        action
    }

    fn teardown(element: Self::Element, state: Self::State, cx: &mut Context<Platform>) {
        V::teardown(
            Pod::new(element.node, state.contents_widget),
            state.contents_state,
            cx,
        );

        P::teardown(
            state.popover_element,
            state.popover_state,
            cx,
        );

        cx.layout.remove_node(state.popover_node);
    }
}

pub struct PopoverState<T, V, P>
where
    V: WidgetView<Platform, T>,
    P: WidgetView<Platform, T>,
{
    contents_widget: V::Widget,
    contents_state:  V::State,
    popover_element: P::Element,
    popover_state:   P::State,
    popover_node:    LayoutNode,
    position:        Position,
}

impl NativeParent<Platform> for PopoverReceiver {
    fn replace_child(&mut self, _platform: &mut Platform, index: usize, child: &gtk4::Widget) {
        match index {
            0 => {
                self.set_child(Some(child));
            }

            1 => {
                self.set_popover_child(Some(child));
            }

            _ => {}
        }
    }
}

impl NativeWidget<Platform> for PopoverReceiver {
    fn widget(&self) -> &gtk4::Widget {
        self.upcast_ref()
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
