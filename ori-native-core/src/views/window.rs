use std::time::Duration;

use ori::{Action, Message, Mut, Proxied, Proxy, View, ViewId, ViewMarker};

use crate::{
    Context, Lifecycle, Pod, Shadow, ShadowView,
    native::{HasWindow, NativeWindow},
    views::AnimationFrame,
};

pub fn window<V>(contents: V) -> Window<V> {
    Window { contents }
}

pub struct Window<V> {
    contents: V,
}

pub enum WindowMessage {
    AnimationFrame(Duration),
    StartAnimating,
    StopAnimating,
    CloseRequested,
    Relayout,
    Resized,
}

impl<V> ViewMarker for Window<V> {}
impl<P, T, V> View<Context<P>, T> for Window<V>
where
    P: HasWindow + Proxied,
    T: 'static,
    V: ShadowView<P, T> + 'static,
{
    type Element = ();
    type State = WindowState<P, T, V>;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let view_id = ViewId::next();

        let (contents, state) = cx.with_window(view_id, |cx| {
            self.contents.build(cx, data)
        });

        let mut window = P::Window::build(
            &mut cx.platform,
            contents.shadow.widget(),
        );

        window.set_on_resize({
            let proxy = cx.proxy();

            move || {
                proxy.message(Message::new(
                    WindowMessage::Resized,
                    view_id,
                ));
            }
        });

        window.set_on_close_requested({
            let proxy = cx.proxy();

            move || {
                proxy.message(Message::new(
                    WindowMessage::CloseRequested,
                    view_id,
                ));
            }
        });

        window.set_on_animation_frame({
            let proxy = cx.proxy();

            move |delta| {
                proxy.message(Message::new(
                    WindowMessage::AnimationFrame(delta),
                    view_id,
                ));
            }
        });

        let node = cx.new_layout_node(Default::default(), &[contents.node]);

        let (width, height) = window.get_size();

        let mut state = WindowState {
            node,
            view_id,
            window,

            width,
            height,

            animating: 0,

            contents,
            state,
        };

        state.layout(cx, data);

        ((), state)
    }

    fn rebuild(
        self,
        _element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        cx.with_window(state.view_id, |cx| {
            self.contents.rebuild(
                state.contents.as_mut(state.contents.node),
                &mut state.state,
                cx,
                data,
            );
        });

        state.layout(cx, data);
    }

    fn message(
        _element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        match message.take_targeted(state.view_id) {
            Some(WindowMessage::AnimationFrame(delta)) => {
                if state.animating == 0 {
                    return Action::new();
                }

                let mut message = Message::new(AnimationFrame(delta), None);

                cx.with_window(state.view_id, |cx| {
                    V::message(
                        state.contents.as_mut(state.node),
                        &mut state.state,
                        cx,
                        data,
                        &mut message,
                    )
                })
            }

            Some(WindowMessage::StartAnimating) => {
                if state.animating == 0 {
                    state.window.start_animating();
                }

                state.animating += 1;

                Action::new()
            }

            Some(WindowMessage::StopAnimating) => {
                state.animating -= 1;

                if state.animating == 0 {
                    state.window.stop_animating();
                }

                Action::new()
            }

            Some(WindowMessage::CloseRequested) => {
                cx.platform.quit();

                Action::new()
            }

            Some(WindowMessage::Relayout) => {
                state.layout(cx, data);

                Action::new()
            }

            Some(WindowMessage::Resized) => {
                let (width, height) = state.window.get_size();

                if state.width != width || state.height != height {
                    state.layout(cx, data);
                }

                Action::new()
            }

            None => cx.with_window(state.view_id, |cx| {
                V::message(
                    state.contents.as_mut(state.node),
                    &mut state.state,
                    cx,
                    data,
                    message,
                )
            }),
        }
    }

    fn teardown(_element: Self::Element, state: Self::State, cx: &mut Context<P>) {
        cx.with_window(state.view_id, |cx| {
            V::teardown(state.contents, state.state, cx);
        });

        state.window.teardown(&mut cx.platform);
        let _ = cx.remove_layout_node(state.node);
    }
}

#[doc(hidden)]
pub struct WindowState<P, T, V>
where
    P: HasWindow,
    V: ShadowView<P, T>,
{
    node:    taffy::NodeId,
    view_id: ViewId,
    window:  P::Window,

    width:  u32,
    height: u32,

    animating: u32,

    contents: Pod<V::Shadow>,
    state:    V::State,
}

impl<P, T, V> WindowState<P, T, V>
where
    P: HasWindow,
    V: ShadowView<P, T>,
{
    fn layout(&mut self, cx: &mut Context<P>, data: &mut T) {
        let (width, height) = self.window.get_size();

        self.width = width;
        self.height = height;

        let style = taffy::Style {
            max_size: taffy::Size::from_lengths(0.0, 0.0),
            ..Default::default()
        };

        let size = taffy::Size {
            width:  taffy::AvailableSpace::MinContent,
            height: taffy::AvailableSpace::MinContent,
        };

        let _ = cx.set_layout_style(self.node, style);
        let _ = cx.compute_layout(self.node, size);

        if let Ok(layout) = cx.get_computed_layout(self.node) {
            self.window.set_min_size(
                layout.content_size.width as u32,
                layout.content_size.height as u32,
            );
        }

        let style = taffy::Style {
            size: taffy::Size::from_lengths(width as f32, height as f32),
            ..Default::default()
        };

        let size = taffy::Size {
            width:  taffy::AvailableSpace::Definite(width as f32),
            height: taffy::AvailableSpace::Definite(height as f32),
        };

        let _ = cx.set_layout_style(self.node, style);
        let _ = cx.compute_layout(self.node, size);

        let action = cx.with_window(self.view_id, |cx| {
            V::message(
                self.contents.as_mut(self.node),
                &mut self.state,
                cx,
                data,
                &mut Message::new(Lifecycle::Layout, None),
            )
        });

        cx.send_action(action);
    }
}
