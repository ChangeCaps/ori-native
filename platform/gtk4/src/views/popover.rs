use glib::{
    object::{Cast, IsA},
    subclass::types::ObjectSubclassIsExt,
};
use gtk4::prelude::{PopoverExt, WidgetExt};
use ori::{Action, Message, Mut, View, ViewMarker};
use ori_native_core::{
    AutoLength, AvailableSpace, Context, LayoutNode, LayoutStyle, Lifecycle, NativeParent,
    NativeWidget, Pod, Size, WidgetView,
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

        let popover = gtk4::Popover::new();
        popover.set_autohide(false);
        popover.set_has_arrow(false);
        popover.set_child(Some(popover_element.widget.widget()));

        if self.is_open {
            popover.connect_realize(|popover| {
                popover.popup();
            });
        }

        let position = match self.position {
            Position::Top => gtk4::PositionType::Top,
            Position::Right => gtk4::PositionType::Right,
            Position::Bottom => gtk4::PositionType::Bottom,
            Position::Left => gtk4::PositionType::Left,
        };

        popover.set_position(position);

        let receiver = PopoverReceiver::new();
        receiver.set_child(Some(contents_element.widget.widget()));
        receiver.set_popover(Some(&popover));

        let popover_node = cx.layout.add_node(&[popover_element.node]);

        let pod = Pod::new(contents_element.node, receiver);
        let state = PopoverState {
            contents_widget: contents_element.widget,
            contents_state,
            popover_element,
            popover_state,
            popover_node,
            popover,
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

        if state.popover.is_visible() != self.is_open {
            match self.is_open {
                true => state.popover.popup(),
                false => state.popover.popdown(),
            }
        }

        let position = match self.position {
            Position::Top => gtk4::PositionType::Top,
            Position::Right => gtk4::PositionType::Right,
            Position::Bottom => gtk4::PositionType::Bottom,
            Position::Left => gtk4::PositionType::Left,
        };

        if state.popover.position() != position {
            state.popover.set_position(position);
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
                let width = allocation.size.width.round() as i32;
                let height = allocation.size.height.round() as i32;

                state.popover.set_size_request(width, height);

                let popover_contents = state.popover_element.widget.widget();
                popover_contents.set_size_request(width, height);
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
    popover:         gtk4::Popover,
}

impl NativeParent<Platform> for PopoverReceiver {
    fn replace_child(&mut self, _platform: &mut Platform, index: usize, child: &gtk4::Widget) {
        match index {
            0 => {
                self.set_child(Some(child));
            }

            1 => {
                if let Some(ref popover) = *self.imp().popover.borrow() {
                    popover.set_child(Some(child));
                }
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
        glib::Object::new()
    }

    pub fn set_child(&self, child: Option<&impl IsA<gtk4::Widget>>) {
        if let Some(ref child) = *self.imp().child.borrow() {
            child.unparent();
        }

        if let Some(child) = child {
            child.set_parent(self);
            self.imp().child.replace(Some(child.as_ref().clone()));
        }
    }

    pub fn set_popover(&self, popover: Option<&impl IsA<gtk4::Popover>>) {
        if let Some(ref popover) = *self.imp().popover.borrow() {
            popover.unparent();
        }

        if let Some(popover) = popover {
            let popover = popover.as_ref();
            popover.set_parent(self);

            self.imp().popover.replace(Some(popover.clone()));
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

    #[derive(Default)]
    pub struct PopoverReceiver {
        pub(super) child:   RefCell<Option<gtk4::Widget>>,
        pub(super) popover: RefCell<Option<gtk4::Popover>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PopoverReceiver {
        const NAME: &'static str = "OriPopupReceiver";
        type Type = super::PopoverReceiver;
        type ParentType = gtk4::Widget;
    }

    impl ObjectImpl for PopoverReceiver {
        fn dispose(&self) {
            if let Some(ref child) = *self.child.borrow() {
                child.unparent();
            }

            if let Some(ref popover) = *self.popover.borrow() {
                popover.unparent();
            }
        }
    }

    impl WidgetImpl for PopoverReceiver {
        fn measure(&self, orientation: gtk4::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            match *self.child.borrow() {
                Some(ref child) => child.measure(orientation, for_size),
                None => (0, 0, -1, -1),
            }
        }

        fn size_allocate(&self, width: i32, height: i32, _baseline: i32) {
            if let Some(ref child) = *self.child.borrow() {
                let allocation = gtk4::Allocation::new(0, 0, width, height);
                child.size_allocate(&allocation, -1);
            }

            if let Some(ref popover) = *self.popover.borrow() {
                popover.present();
            }
        }
    }
}
