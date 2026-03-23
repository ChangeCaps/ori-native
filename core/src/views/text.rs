use std::borrow::Cow;

use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{
    Color, Context, Font, Layout, LayoutStyle, Platform, Pod, Stretch, TextSpan, Weight, Wrap,
    native::NativeText,
};

/// [`View`] of a text paragraph.
pub fn text(text: impl Into<String>) -> Text {
    Text::new(text)
}

/// [`View`] of a text paragraph.
pub struct Text {
    layout: LayoutStyle,
    font:   Font,
    text:   String,
    wrap:   Wrap,
}

impl Text {
    /// Create new [`Text`].
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            layout: LayoutStyle::default(),
            font:   Default::default(),
            text:   text.into(),
            wrap:   Wrap::None,
        }
    }

    /// Set the font size.
    pub fn size(mut self, size: f32) -> Self {
        self.font.size = size;
        self
    }

    /// Set the font family.
    pub fn family(mut self, family: impl Into<Cow<'static, str>>) -> Self {
        self.font.family = Some(family.into());
        self
    }

    /// Set the font weight.
    pub fn weight(mut self, weight: Weight) -> Self {
        self.font.weight = weight;
        self
    }

    /// Set the font stretch.
    pub fn stretch(mut self, stretch: Stretch) -> Self {
        self.font.stretch = stretch;
        self
    }

    /// Set whether the font is italic.
    pub fn italic(mut self, italic: bool) -> Self {
        self.font.italic = italic;
        self
    }

    /// Set whether the font is strikethrough.
    pub fn strikethrough(mut self, strikethrough: bool) -> Self {
        self.font.striketrough = strikethrough;
        self
    }

    /// Set the wrapping mode.
    pub fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = wrap;
        self
    }

    /// Set the color.
    pub fn color(mut self, color: Color) -> Self {
        self.font.color = color;
        self
    }
}

impl Layout for Text {
    fn get_layout_style_mut(&mut self) -> &mut LayoutStyle {
        &mut self.layout
    }
}

impl ViewMarker for Text {}
impl<P, T> View<Context<P>, T> for Text
where
    P: Platform,
{
    type Element = Pod<P, P::Text>;
    type State = TextState;

    fn build(self, cx: &mut Context<P>, _data: &mut T) -> (Self::Element, Self::State) {
        let spans = [TextSpan {
            font:  self.font.clone(),
            range: 0..self.text.len(),
        }];

        let mut widget = P::Text::build(&mut cx.platform);

        let layout = widget.set_text(
            &mut cx.platform,
            spans.into(),
            self.text.clone(),
            self.wrap,
        );

        let node = cx.layout.add_leaf(layout);
        cx.layout.set_layout(node, self.layout);

        let pod = Pod::new(node, widget);

        let state = TextState {
            layout: self.layout,
            font:   self.font,
            text:   self.text,
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
            cx.layout.set_layout(*element.node, self.layout);
        }

        if state.font == self.font && state.text == self.text {
            return;
        }

        state.font = self.font.clone();
        state.text = self.text.clone();

        let spans = [TextSpan {
            font:  self.font,
            range: 0..self.text.len(),
        }];

        let layout = element.widget.set_text(
            &mut cx.platform,
            spans.into(),
            self.text,
            self.wrap,
        );

        cx.layout.set_measure(*element.node, layout);
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
        cx.layout.remove_node(element.node);
    }
}

pub struct TextState {
    layout: LayoutStyle,
    font:   Font,
    text:   String,
}
