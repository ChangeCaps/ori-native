use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{
    Affine, Allocation, Context, Lifecycle, NativeWidget, Platform, Pod, WidgetView,
    native::NativeTransform,
};

/// [`View`] that transforms its contents.
pub fn transform<V>(contents: V) -> Transform<V> {
    Transform::new(contents)
}

/// [`View`] that transforms its contents.
pub struct Transform<V> {
    contents: V,
    affine:   Affine,
}

impl<V> Transform<V> {
    /// Create new [`Transform`].
    pub fn new(contents: V) -> Self {
        Self {
            contents,
            affine: Affine::default(),
        }
    }

    /// Translate the contents by `x` and `y`.
    pub fn translate(mut self, x: f32, y: f32) -> Self {
        self.affine.offset_x = x;
        self.affine.offset_y = y;
        self
    }

    /// Rotate the contents by `degrees`.
    pub fn rotate(mut self, degrees: f32) -> Self {
        self.affine.rotation = degrees;
        self
    }

    /// Scale the contents by `sx` and `sy`.
    pub fn scale(mut self, sx: f32, sy: f32) -> Self {
        self.affine.scale_x = sx;
        self.affine.scale_y = sy;
        self
    }
}

impl<V> ViewMarker for Transform<V> {}
impl<P, V, T> View<Context<P>, T> for Transform<V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    type Element = Pod<P, P::Transform>;
    type State = TransformState<P, T, V>;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let (contents, state) = self.contents.build(cx, data);

        let widget = P::Transform::build(
            &mut cx.platform,
            contents.widget.widget(),
        );

        let pod = Pod::new(contents.node, widget);
        let state = TransformState {
            widget: contents.widget,
            state,
            affine: self.affine,
            allocation: None,
        };

        (pod, state)
    }

    fn rebuild(
        self,
        mut element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        let pod = element.map_widget(&mut state.widget);
        self.contents.rebuild(pod, &mut state.state, cx, data);

        if state.affine != self.affine
            && let Some(allocation) = cx.layout.get_computed_layout(*element.node)
        {
            state.affine = self.affine;
            state.allocation = Some(allocation);
            element.widget.set_content_transform(
                &mut cx.platform,
                allocation.size.width,
                allocation.size.height,
                state.affine,
            );
        }
    }

    fn message(
        mut element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        if let Some(Lifecycle::Layout) = message.get()
            && let Some(allocation) = cx.layout.get_computed_layout(*element.node)
            && state.allocation != Some(allocation)
        {
            state.allocation = Some(allocation);
            element.widget.set_content_transform(
                &mut cx.platform,
                allocation.size.width,
                allocation.size.height,
                state.affine,
            );
        }

        let pod = element.map_widget(&mut state.widget);
        V::message(pod, &mut state.state, cx, data, message)
    }

    fn teardown(element: Self::Element, state: Self::State, cx: &mut Context<P>) {
        let pod = Pod::new(element.node, state.widget);
        V::teardown(pod, state.state, cx);

        element.widget.teardown(&mut cx.platform);
    }
}

pub struct TransformState<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    widget:     V::Widget,
    state:      V::State,
    affine:     Affine,
    allocation: Option<Allocation>,
}
