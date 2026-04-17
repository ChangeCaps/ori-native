use ori::{Action, Message, Mut, Proxied, Proxy, Tracker, View, ViewId, ViewMarker};

use crate::{
    Allocation, Context, Lifecycle, NativeWidget, Platform, Pod, WidgetView, native::NativeMeasure,
};

/// [`View`] that measures the position and size of its contents.
pub fn measure<T, V, A>(
    contents: V,
    on_measure: impl FnMut(&mut T, f32, f32, f32, f32) -> A + 'static,
) -> Measure<T, V>
where
    A: Into<Action>,
{
    Measure::new(contents, on_measure)
}

/// [`View`] that measures the position and size of its contents.
#[allow(clippy::type_complexity)]
pub struct Measure<T, V> {
    contents:   V,
    on_measure: Box<dyn FnMut(&mut T, f32, f32, f32, f32) -> Action>,
}

impl<T, V> Measure<T, V> {
    /// Create new [`Measure`].
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

enum MeasureMessage {
    PositionChanged(f32, f32),
}

impl<T, V> ViewMarker for Measure<T, V> {}
impl<P, T, V> View<Context<P>, T> for Measure<T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    type Element = Pod<P, P::Measure>;
    type State = MeasureState<P, T, V>;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let (contents, state) = self.contents.build(cx, data);

        let view_id = ViewId::next();
        cx.register(view_id);

        let on_position_changed = {
            let proxy = cx.proxy();

            move |x, y| {
                proxy.message(Message::new(
                    MeasureMessage::PositionChanged(x, y),
                    Some(view_id),
                ));
            }
        };

        let widget = P::Measure::build(
            &mut cx.platform,
            contents.widget.widget_ref(),
            on_position_changed,
        );

        let pod = Pod::new(contents.layout, widget);
        let state = MeasureState {
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
            && let Some(allocation) = cx.layout.get_allocation(*element.layout)
            && state.allocation != Some(allocation)
        {
            state.allocation = Some(allocation);
            element.widget.set_content_size(
                &mut cx.platform,
                allocation.size.width,
                allocation.size.height,
            );
        }

        if let Some(MeasureMessage::PositionChanged(x, y)) = message.take(state.view_id)
            && let Some(allocation) = cx.layout.get_allocation(*element.layout)
        {
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
        let pod = Pod::new(element.layout, state.widget);
        V::teardown(pod, state.state, cx);

        element.widget.teardown(&mut cx.platform);
        cx.unregister(state.view_id);
    }
}

#[allow(clippy::type_complexity)]
pub struct MeasureState<P, T, V>
where
    P: Platform,
    V: WidgetView<P, T>,
{
    view_id:    ViewId,
    widget:     V::Widget,
    state:      V::State,
    allocation: Option<Allocation>,
    on_measure: Box<dyn FnMut(&mut T, f32, f32, f32, f32) -> Action>,
}
