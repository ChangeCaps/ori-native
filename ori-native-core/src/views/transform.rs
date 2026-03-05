use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{
    Affine, Context, Lifecycle, NativeWidget, Pod, WidgetView,
    native::{HasTransform, NativeTransform},
};

pub fn transform<V>(contents: V) -> Transform<V> {
    Transform::new(contents)
}

pub struct Transform<V> {
    contents: V,
    affine:   Affine,
}

impl<V> Transform<V> {
    pub fn new(contents: V) -> Self {
        Self {
            contents,
            affine: Affine::default(),
        }
    }

    pub fn translate(mut self, x: f32, y: f32) -> Self {
        self.affine.offset_x = x;
        self.affine.offset_y = y;
        self
    }

    pub fn rotate(mut self, degrees: f32) -> Self {
        self.affine.rotation = degrees;
        self
    }

    pub fn scale(mut self, sx: f32, sy: f32) -> Self {
        self.affine.scale_x = sx;
        self.affine.scale_y = sy;
        self
    }
}

impl<V> ViewMarker for Transform<V> {}
impl<P, V, T> View<Context<P>, T> for Transform<V>
where
    P: HasTransform,
    V: WidgetView<P, T>,
{
    type Element = Pod<P, P::Transform>;
    type State = (V::Widget, V::State, Affine);

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let (contents, state) = self.contents.build(cx, data);

        let widget = P::Transform::build(
            &mut cx.platform,
            contents.widget.widget(),
        );

        let pod = Pod::new(contents.node, widget);

        (
            pod,
            (contents.widget, state, self.affine),
        )
    }

    fn rebuild(
        self,
        mut element: Mut<'_, Self::Element>,
        (contents, state, affine): &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        let pod = element.map_widget(contents);
        self.contents.rebuild(pod, state, cx, data);

        *affine = self.affine;
    }

    fn message(
        mut element: Mut<'_, Self::Element>,
        (contents, state, affine): &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        if let Some(Lifecycle::Layout) = message.get()
            && let Ok(layout) = cx.get_computed_layout(*element.node).cloned()
        {
            element.widget.set_content_transform(
                &mut cx.platform,
                layout.size.width,
                layout.size.height,
                *affine,
            );
        }

        let pod = element.map_widget(contents);
        V::message(pod, state, cx, data, message)
    }

    fn teardown(element: Self::Element, (contents, state, _): Self::State, cx: &mut Context<P>) {
        let pod = Pod::new(element.node, contents);
        V::teardown(pod, state, cx);

        element.widget.teardown(&mut cx.platform);
    }
}
