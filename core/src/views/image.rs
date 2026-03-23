use std::borrow::Cow;

use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{Color, Context, Layout, LayoutStyle, Platform, Pod, native::NativeImage};

/// [`View`] of an image.
pub fn image(data: impl Into<Cow<'static, [u8]>>) -> Image {
    Image::new(data.into())
}

/// [`View`] of an image.
pub struct Image {
    layout: LayoutStyle,
    data:   Cow<'static, [u8]>,
    tint:   Option<Color>,
}

impl Image {
    /// Create new [`Image`].
    pub fn new(data: Cow<'static, [u8]>) -> Self {
        Self {
            layout: Default::default(),
            data,
            tint: None,
        }
    }

    /// Set the tint.
    ///
    /// Setting this will use the image as a mask.
    pub fn tint(mut self, tint: impl Into<Option<Color>>) -> Self {
        self.tint = tint.into();
        self
    }
}

impl Layout for Image {
    fn get_layout_style_mut(&mut self) -> &mut LayoutStyle {
        &mut self.layout
    }
}

impl ViewMarker for Image {}
impl<P, T> View<Context<P>, T> for Image
where
    P: Platform,
{
    type Element = Pod<P, P::Image>;
    type State = ImageState;

    fn build(self, cx: &mut Context<P>, _data: &mut T) -> (Self::Element, Self::State) {
        let mut widget = P::Image::build(&mut cx.platform);
        widget.set_tint(&mut cx.platform, self.tint);

        let hash = seahash::hash(&self.data);
        let node = match widget.load_data(&mut cx.platform, self.data) {
            Ok(layout) => cx.layout.add_leaf(layout),

            Err(error) => {
                tracing::error!(?error, "loading image failed");
                cx.layout.add_node(&[])
            }
        };

        cx.layout.set_layout(node, self.layout);

        let pod = Pod::new(node, widget);

        let state = ImageState {
            layout: self.layout,
            hash,
            tint: self.tint,
        };

        (pod, state)
    }

    fn rebuild(
        self,
        element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        _data: &mut T,
    ) {
        if state.layout != self.layout {
            state.layout = self.layout;
            cx.layout.set_layout(*element.node, self.layout);
        }

        let hash = seahash::hash(&self.data);

        if state.hash != hash {
            state.hash = hash;

            match element.widget.load_data(&mut cx.platform, self.data) {
                Ok(layout) => {
                    cx.layout.set_measure(*element.node, layout);
                }

                Err(error) => tracing::error!(?error, "loading image failed"),
            }
        }

        if state.tint != self.tint {
            state.tint = self.tint;
            element.widget.set_tint(&mut cx.platform, self.tint);
        }
    }

    fn message(
        _element: Mut<'_, Self::Element>,
        _state: &mut Self::State,
        _cx: &mut Context<P>,
        _data: &mut T,
        _message: &mut Message,
    ) -> Action {
        Action::new()
    }

    fn teardown(element: Self::Element, _state: Self::State, cx: &mut Context<P>) {
        element.widget.teardown(&mut cx.platform);
    }
}

pub struct ImageState {
    layout: LayoutStyle,
    hash:   u64,
    tint:   Option<Color>,
}
