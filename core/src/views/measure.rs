use ori::{Action, Message, Mut, Tracker, View, ViewId, ViewMarker};

use crate::{
    Allocation, Context, Lifecycle, NativeWidget, Platform, Pod, WidgetView, native::NativeMeasure,
};

/// [`View`] that measures the position and size of its contents.
pub fn on_measure<T, V, A>(
    contents: V,
    on_measure: impl FnMut(&mut T, f32, f32, f32, f32) -> A + 'static,
) -> OnMeasure<T, V>
where
    A: Into<Action>,
{
    OnMeasure::new(contents, on_measure)
}

/// [`View`] that measures the position and size of its contents.
#[allow(clippy::type_complexity)]
pub struct OnMeasure<T, V> {
    contents:   V,
    on_measure: Box<dyn FnMut(&mut T, f32, f32, f32, f32) -> Action>,
}

impl<T, V> OnMeasure<T, V> {
    /// Create new [`OnMeasure`].
    pub fn new<A>(
        contents: V,
        mut on_measure: impl FnMut(&mut T, f32, f32, f32, f32) -> A + 'static,
    ) -> Self
    where
        A: Into<Action>,
    {
        Self {
            contents,
            on_measure: Box::new(move |data, x, y, width, height| {
                on_measure(data, x, y, width, height).into()
            }),
        }
    }
}

impl<T, V> ViewMarker for OnMeasure<T, V> {}
impl<P, T, V> View<Context<P>, T> for OnMeasure<T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    type Element = Pod<P, P::Measure>;
    type State = OnMeasureState<P, T, V>;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let (contents, state) = self.contents.build(cx, data);

        let widget = P::Measure::build(
            &mut cx.platform,
            contents.widget.widget(),
        );

        let view_id = ViewId::next();
        cx.register(view_id);

        let pod = Pod::new(contents.node, widget);
        let state = OnMeasureState {
            widget: contents.widget,
            state,
            view_id,
            allocation: None,
            on_measure: self.on_measure,
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
        let pod = element.map_widget(&mut state.widget, 0);
        self.contents.rebuild(pod, &mut state.state, cx, data);
    }

    fn message(
        mut element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        let mut action = Action::new();

        if let Some(Lifecycle::Layout) = message.get()
            && let Some(allocation) = cx.layout.get_allocation(*element.node)
            && state.allocation != Some(allocation)
        {
            state.allocation = Some(allocation);
            element.widget.set_content_size(
                &mut cx.platform,
                allocation.size.width,
                allocation.size.height,
            );
        }

        if let Some(Lifecycle::Layout) = message.get()
            && let Some(allocation) = cx.layout.get_allocation(*element.node)
        {
            let (x, y) = element.widget.measure(&mut cx.platform);
            action |= (state.on_measure)(
                data,
                x,
                y,
                allocation.size.width,
                allocation.size.height,
            );
        }

        let pod = element.map_widget(&mut state.widget, 0);
        action | V::message(pod, &mut state.state, cx, data, message)
    }

    fn teardown(element: Self::Element, state: Self::State, cx: &mut Context<P>) {
        let pod = Pod::new(element.node, state.widget);
        V::teardown(pod, state.state, cx);

        element.widget.teardown(&mut cx.platform);
        cx.unregister(state.view_id);
    }
}

#[allow(clippy::type_complexity)]
pub struct OnMeasureState<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    widget:     V::Widget,
    state:      V::State,
    view_id:    ViewId,
    allocation: Option<Allocation>,
    on_measure: Box<dyn FnMut(&mut T, f32, f32, f32, f32) -> Action>,
}
