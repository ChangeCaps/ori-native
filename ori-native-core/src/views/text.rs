use ori::{Action, Message, Mut, View, ViewMarker};

use crate::{
    Context, FontAttributes, FontWeight, Pod, TextSpan, shadows::TextShadow, text::FontStretch,
    widgets::HasText,
};

pub fn text(text: impl Into<String>) -> Text {
    Text::new(text)
}

pub struct Text {
    attrs: FontAttributes,
    text:  String,
}

impl Text {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            attrs: FontAttributes {
                size:    16.0,
                family:  String::from("Ubuntu Light"),
                weight:  FontWeight::NORMAL,
                stretch: FontStretch::Normal,
                italic:  false,
            },
            text:  text.into(),
        }
    }
}

impl ViewMarker for Text {}
impl<P, T> View<Context<P>, T> for Text
where
    P: HasText,
{
    type Element = Pod<TextShadow<P>>;
    type State = ();

    fn build(self, cx: &mut Context<P>, _data: &mut T) -> (Self::Element, Self::State) {
        let spans = [TextSpan {
            attributes: self.attrs,
            range:      0..self.text.len(),
        }];

        let (shadow, leaf) = TextShadow::new(cx, spans.into(), self.text);

        let node = cx
            .layout_tree
            .new_leaf_with_context(Default::default(), Box::new(leaf))
            .unwrap();

        let pod = Pod { node, shadow };

        (pod, ())
    }

    fn rebuild(
        self,
        element: Mut<'_, Self::Element>,
        _state: &mut Self::State,
        cx: &mut Context<P>,
        _data: &mut T,
    ) {
        let spans = [TextSpan {
            attributes: self.attrs,
            range:      0..self.text.len(),
        }];

        let leaf = element.shadow.set_text(spans.into(), self.text);
        let _ = (cx.layout_tree).set_node_context(*element.node, Some(Box::new(leaf)));
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
        element.shadow.teardown(cx);
        let _ = cx.layout_tree.remove(element.node);
    }
}
