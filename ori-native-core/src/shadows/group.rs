use ori::{Elements, Mut};

use crate::{
    AnyShadow, Context, PodMut, Shadow,
    widgets::{HasGroup, NativeGroup},
};

pub struct GroupShadow<P>
where
    P: HasGroup,
{
    group:    P::Group,
    children: Vec<AnyShadow<P>>,
}

impl<P> GroupShadow<P>
where
    P: HasGroup,
{
    pub fn new(cx: &mut Context<P>) -> Self {
        Self {
            group:    P::Group::build(&mut cx.platform),
            children: Vec::new(),
        }
    }

    pub fn teardown(self, cx: &mut Context<P>) {
        self.group.teardown(&mut cx.platform);
    }

    pub fn elements(&mut self, node: taffy::NodeId) -> impl Elements<Context<P>, AnyShadow<P>> {
        GroupElements {
            node,
            index: 0,
            group: &mut self.group,
            children: &mut self.children,
        }
    }
}

impl<P> Shadow<P> for GroupShadow<P>
where
    P: HasGroup,
{
    fn widget(&self) -> &P::Widget {
        self.group.widget()
    }

    fn layout(&mut self, cx: &mut Context<P>, node: taffy::NodeId) {
        let layout = cx.layout_tree.layout(node).unwrap();
        self.group.set_size(layout.size.width, layout.size.height);

        for (index, child) in self.children.iter_mut().enumerate() {
            let layout = cx.layout_tree.layout(child.node).unwrap();

            self.group.set_child_position(
                index,
                layout.location.x,
                layout.location.y,
            );

            child.shadow.layout(cx, child.node);
        }
    }
}

struct GroupElements<'a, P>
where
    P: HasGroup,
{
    node:     taffy::NodeId,
    index:    usize,
    group:    &'a mut P::Group,
    children: &'a mut Vec<AnyShadow<P>>,
}

impl<P> Elements<Context<P>, AnyShadow<P>> for GroupElements<'_, P>
where
    P: HasGroup,
{
    fn next(&mut self, _cx: &mut Context<P>) -> Option<Mut<'_, AnyShadow<P>>> {
        let child = self.children.get_mut(self.index)?;

        let pod = PodMut {
            parent: self.node,
            node:   &mut child.node,
            shadow: &mut child.shadow,
        };

        Some(pod)
    }

    fn insert(&mut self, cx: &mut Context<P>, element: AnyShadow<P>) {
        cx.layout_tree
            .insert_child_at_index(self.node, self.index, element.node)
            .unwrap();

        self.group.insert_child(self.index, element.shadow.widget());
        self.children.insert(self.index, element);
        self.index += 1;
    }

    fn remove(&mut self, cx: &mut Context<P>) -> Option<AnyShadow<P>> {
        self.group.remove_child(self.index);
        let child = self.children.remove(self.index);
        cx.layout_tree.remove(child.node).unwrap();

        Some(child)
    }

    fn swap(&mut self, cx: &mut Context<P>, offset: usize) {
        cx.layout_tree
            .replace_child_at_index(
                self.node,
                self.index,
                self.children[self.index + offset].node,
            )
            .unwrap();

        cx.layout_tree
            .replace_child_at_index(
                self.node,
                self.index + offset,
                self.children[self.index].node,
            )
            .unwrap();

        self.group.swap_children(self.index, self.index + offset);
        self.children.swap(self.index, self.index + offset);
    }
}
