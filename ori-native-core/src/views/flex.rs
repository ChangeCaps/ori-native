use ori::{Action, Message, Mut, View, ViewMarker, ViewSeq};
use taffy::FlexDirection;

use crate::{
    AnyShadow, Context, FlexContainer, FlexItem, Layout, Pod, shadows::GroupShadow,
    widgets::HasGroup,
};

pub fn flex_row<V>(contents: V) -> Flex<V> {
    Flex::new(contents, FlexDirection::Row)
}

pub fn flex_column<V>(contents: V) -> Flex<V> {
    Flex::new(contents, FlexDirection::Column)
}

pub struct Flex<V> {
    contents: V,
    style:    taffy::Style,
}

impl<V> Flex<V> {
    pub fn new(contents: V, direction: FlexDirection) -> Self {
        Self {
            contents,
            style: taffy::Style {
                display: taffy::Display::Flex,
                flex_direction: direction,
                ..Default::default()
            },
        }
    }
}

impl<V> Layout for Flex<V> {
    fn style_mut(&mut self) -> &mut taffy::Style {
        &mut self.style
    }
}

impl<V> FlexItem for Flex<V> {}
impl<V> FlexContainer for Flex<V> {}

impl<V> ViewMarker for Flex<V> {}
impl<P, T, V> View<Context<P>, T> for Flex<V>
where
    P: HasGroup,
    V: ViewSeq<Context<P>, T, AnyShadow<P>>,
{
    type Element = Pod<GroupShadow<P>>;
    type State = V::State;

    fn build(self, cx: &mut Context<P>, data: &mut T) -> (Self::Element, Self::State) {
        let node = cx.layout_tree.new_leaf(self.style).unwrap();

        let mut shadow = GroupShadow::new(cx);

        let state = self
            .contents
            .seq_build(&mut shadow.elements(node), cx, data);

        let pod = Pod { node, shadow };

        (pod, state)
    }

    fn rebuild(
        self,
        element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
    ) {
        cx.layout_tree.set_style(*element.node, self.style).unwrap();

        self.contents.seq_rebuild(
            &mut element.shadow.elements(*element.node),
            state,
            cx,
            data,
        );
    }

    fn message(
        element: Mut<'_, Self::Element>,
        state: &mut Self::State,
        cx: &mut Context<P>,
        data: &mut T,
        message: &mut Message,
    ) -> Action {
        V::seq_message(
            &mut element.shadow.elements(*element.node),
            state,
            cx,
            data,
            message,
        )
    }

    fn teardown(mut element: Self::Element, state: Self::State, cx: &mut Context<P>) {
        V::seq_teardown(
            &mut element.shadow.elements(element.node),
            state,
            cx,
        );
        element.shadow.teardown(cx);
        cx.layout_tree.remove(element.node).unwrap();
    }
}
