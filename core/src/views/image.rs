use std::borrow::Cow;

use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{Color, Context, Layout, Platform, Pod, native::NativeImage};

/// [`View`] of an image.
pub fn image(data: impl Into<Cow<'static, [u8]>>) -> Image {
    Image::new(data.into())
}

/// [`View`] of an image.
pub struct Image {
    style: taffy::Style,
    data:  Cow<'static, [u8]>,
    tint:  Option<Color>,
}

impl Image {
    /// Create new [`Image`].
    pub fn new(data: Cow<'static, [u8]>) -> Self {
        Self {
            style: Default::default(),
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
    fn style_mut(&mut self) -> &mut taffy::Style {
        &mut self.style
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
            Ok(layout) => cx.new_layout_leaf(self.style, layout),

            Err(error) => {
                tracing::error!(?error, "loading image failed");
                cx.new_layout_node(self.style, &[])
            }
        };

        let pod = Pod::new(node, widget);

        let state = ImageState {
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
        let _ = cx.set_layout_style(*element.node, self.style);

        let hash = seahash::hash(&self.data);

        if state.hash != hash {
            state.hash = hash;

            match element.widget.load_data(&mut cx.platform, self.data) {
                Ok(layout) => {
                    let _ = cx.set_layout_measure(*element.node, layout);
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
    hash: u64,
    tint: Option<Color>,
}
