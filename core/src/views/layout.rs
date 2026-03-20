use ori::{Action, Message, Mut, Proxied, View, ViewMarker};

use crate::{Context, Lifecycle, Platform, WidgetView};

/// [`View`] with a callback when layout changes.
pub fn on_layout<T, V, A>(
    contents: V,
    on_layout: impl FnMut(&mut T, f32, f32) -> A,
) -> OnLayout<V, impl FnMut(&mut T, f32, f32) -> A> {
    OnLayout::new(contents, on_layout)
}

/// [`View`] with a callback when layout changes.
pub struct OnLayout<V, F> {
    contents:  V,
    on_layout: F,
}

impl<V, F> OnLayout<V, F> {
    /// Create new [`OnLayout`].
    pub fn new(contents: V, on_layout: F) -> Self {
        Self {
            contents,
            on_layout,
        }
    }
}

impl<V, F> ViewMarker for OnLayout<V, F> {}
impl<P, T, V, F, A> View<Context<P>, T> for OnLayout<V, F>
where
    P: Platform,
    V: WidgetView<P, T>,
    F: FnMut(&mut T, f32, f32) -> A + 'static,
    A: Into<Action>,
{
    type Element = V::Element;
    type State = (V::State, F, Option<taffy::Layout>);

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let (element, state) = self.contents.build(cx, data);

        (element, (state, self.on_layout, None))
    }

    fn rebuild(
        self,
        element: Mut<'_, Self::Element>,
        (state, on_layout, _current_layout): &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        self.contents.rebuild(element, state, cx, data);
        *on_layout = self.on_layout;
    }

    fn message(
        element: Mut<'_, Self::Element>,
        (state, on_layout, current_layout): &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        if let Some(Lifecycle::Layout) = message.get()
            && let Ok(layout) = cx.get_computed_layout(*element.node).copied()
            && *current_layout != Some(layout)
        {
            *current_layout = Some(layout);

            let action = on_layout(
                data,
                layout.size.width,
                layout.size.height,
            );

            cx.send_action(action.into());
        }

        V::message(element, state, cx, data, message)
    }

    fn teardown(
        element: Self::Element,
        (state, _on_layout, _current_layout): Self::State,
        cx: &mut Context<P>,
    ) {
        V::teardown(element, state, cx);
    }
}
