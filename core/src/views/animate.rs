use std::time::Duration;

use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{Context, Lifecycle, Platform, WidgetView};

/// [`View`] that animates its contents.
pub fn animate<P, T, A>(animation: A) -> impl WidgetView<P, T>
where
    P: Platform,
    A: Animation<T>,
    A::View: WidgetView<P, T>,
{
    Animate::new(animation)
}

/// An animated [`View`].
pub trait Animation<T> {
    /// The retained state of the animation.
    type State;

    /// The view produced by the animation.
    type View;

    /// Build the animation state, and return whether the animation should start.
    fn build(self, data: &mut T) -> (Self::State, bool);

    /// Rebuild the animation state, and return whether the animation should be running.
    fn rebuild(self, state: &mut Self::State, data: &mut T) -> bool;

    /// Update the state in response to an animation frame, and return whether the animation should
    /// continue.
    fn animate(state: &mut Self::State, data: &mut T, duration: Duration) -> bool;

    /// Build the animated [`View`].
    fn view(state: &Self::State, data: &T) -> Self::View;
}

/// [`View`] that animates its contents.
pub struct Animate<A> {
    animation: A,
}

impl<A> Animate<A> {
    /// Create new [`Animate`].
    pub fn new(animation: A) -> Self {
        Self { animation }
    }
}

impl<A> ViewMarker for Animate<A> {}
impl<P, T, A> View<Context<P>, T> for Animate<A>
where
    P: Platform,
    A: Animation<T>,
    A::View: WidgetView<P, T>,
{
    type Element = <A::View as View<Context<P>, T>>::Element;
    type State = (
        A::State,
        bool,
        <A::View as View<Context<P>, T>>::State,
    );

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let (state, should_animate) = self.animation.build(data);

        if should_animate {
            cx.request_start_animating();
        }

        let view = A::view(&state, data);
        let (element, contents) = view.build(cx, data);

        (
            element,
            (state, should_animate, contents),
        )
    }

    fn rebuild(
        self,
        element: Mut<'_, Self::Element>,
        (state, is_animating, contents): &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        let should_animate = self.animation.rebuild(state, data);

        let view = A::view(state, data);
        view.rebuild(element, contents, cx, data);

        if *is_animating != should_animate {
            match should_animate {
                true => cx.request_start_animating(),
                false => cx.request_stop_animating(),
            }

            *is_animating = should_animate;
        }
    }

    fn message(
        mut element: Mut<'_, Self::Element>,
        (state, is_animating, contents): &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        if let Some(Lifecycle::Animate(delta)) = message.get()
            && *is_animating
        {
            let should_animate = A::animate(state, data, *delta);
            let view = A::view(state, data);

            view.rebuild(element.reborrow(), contents, cx, data);

            if *is_animating != should_animate {
                match should_animate {
                    true => cx.request_start_animating(),
                    false => cx.request_stop_animating(),
                }

                *is_animating = should_animate;
            }
        }

        A::View::message(element, contents, cx, data, message)
    }

    fn teardown(
        element: Self::Element,
        (_state, is_animating, contents): Self::State,
        cx: &mut Context<P>,
    ) {
        A::View::teardown(element, contents, cx);

        if is_animating {
            cx.request_stop_animating();
        }
    }
}
