use ori::{Action, Message, Mut, Tracker, View, ViewId, ViewMarker};

use crate::{
    Allocation, AvailableSpace, Context, LayoutNode, LayoutStyle, Length, Lifecycle, NativeWidget,
    Platform, Size, WidgetView, native::NativeModal,
};

/// [`Effect`](ori::Effect) that overlays a [`View`] over the window.
pub fn modal<V>(contents: V) -> Modal<V> {
    Modal::new(contents)
}

/// [`Effect`](ori::Effect) that overlays a [`View`] over the window.
pub struct Modal<V> {
    contents: V,
}

impl<V> Modal<V> {
    /// Create new [`Modal`].
    pub fn new(contents: V) -> Self {
        Self { contents }
    }
}

impl<V> ViewMarker for Modal<V> {}
impl<P, T, V> View<Context<P>, T> for Modal<V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    type Element = ();
    type State = ModalState<P, T, V>;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let (element, state) = self.contents.build(cx, data);
        let view_id = ViewId::next();
        cx.register(view_id);

        let modal = P::Modal::build(
            &mut cx.platform,
            element.widget.widget(),
        );

        cx.open_modal(view_id, modal);
        let node = cx.layout.add_node(&[element.node]);

        let state = ModalState {
            node,
            element,
            state,
            view_id,
            width: 0.0,
            height: 0.0,
            allocation: None,
        };

        ((), state)
    }

    fn rebuild(
        self,
        _element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        cx.with_modal(state.view_id, |cx, modal| {
            self.contents.rebuild(
                state.element.as_mut(state.node, 0, modal, 0),
                &mut state.state,
                cx,
                data,
            );
        });
    }

    fn message(
        _element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        cx.with_modal(state.view_id, |cx, modal| {
            if let Some(Lifecycle::Layout) = message.get() {
                let (width, height) = modal.get_size(&mut cx.platform);

                let style = LayoutStyle {
                    size: Size {
                        width:  Some(Length::Length(width)),
                        height: Some(Length::Length(height)),
                    },
                    ..Default::default()
                };

                let space = Size {
                    width:  AvailableSpace::Definite(width),
                    height: AvailableSpace::Definite(height),
                };

                if state.width != width || state.height != height {
                    state.width = width;
                    state.height = height;
                    cx.layout.set_layout(state.node, style);
                }

                (cx.layout).compute_layout(&mut cx.platform, state.node, space);

                if let Some(allocation) = cx.layout.get_allocation(state.element.node)
                    && state.allocation != Some(allocation)
                {
                    state.allocation = Some(allocation);
                    modal.set_content_layout(
                        &mut cx.platform,
                        allocation.x,
                        allocation.y,
                        allocation.size.width,
                        allocation.size.height,
                    );
                }
            }

            V::message(
                state.element.as_mut(state.node, 0, modal, 0),
                &mut state.state,
                cx,
                data,
                message,
            )
        })
        .unwrap_or_default()
    }

    fn teardown(_element: Self::Element, state: Self::State, cx: &mut Context<P>) {
        V::teardown(state.element, state.state, cx);
        cx.close_modal(state.view_id);
        cx.unregister(state.view_id);
        cx.layout.remove_node(state.node);
    }
}

pub struct ModalState<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    node:       LayoutNode,
    element:    V::Element,
    state:      V::State,
    view_id:    ViewId,
    width:      f32,
    height:     f32,
    allocation: Option<Allocation>,
}
