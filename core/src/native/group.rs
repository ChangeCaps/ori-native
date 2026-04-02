use ori::{Elements, Mut};

use crate::{
    Allocation, BoxedWidget, Color, Context, LayoutNode, NativeWidget, Overflow, Platform, PodMut,
    Shadow, Unsupported, element::NativeParent, platform::unsupported,
};

pub trait NativeGroup<P>: NativeWidget<P> + NativeParent<P>
where
    P: Platform,
{
    fn build(platform: &mut P) -> Self;
    fn teardown(self, platform: &mut P);

    fn insert_child(&mut self, platform: &mut P, index: usize, child: &P::Widget);
    fn remove_child(&mut self, platform: &mut P, index: usize);
    fn swap_children(&mut self, platform: &mut P, index_a: usize, index_b: usize);

    fn set_child_layout(
        &mut self,
        platform: &mut P,
        index: usize,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    );

    fn set_background_color(&mut self, platform: &mut P, color: Color);
    fn set_border_color(&mut self, platform: &mut P, color: Color);
    fn set_border_width(&mut self, platform: &mut P, width: [f32; 4]);
    fn set_corner_radii(&mut self, platform: &mut P, radii: [f32; 4]);
    fn set_overflow(&mut self, platform: &mut P, overflow: Overflow);
    fn set_shadow(&mut self, platform: &mut P, shadow: Shadow);

    /* backend specific */

    fn set_hardware_layer(&mut self, platform: &mut P, enabled: bool) {
        let _ = platform;
        let _ = enabled;
    }
}

impl<P> NativeGroup<P> for Unsupported
where
    P: Platform,
{
    fn build(_platform: &mut P) -> Self {
        unsupported!("group widget")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    fn insert_child(&mut self, _platform: &mut P, _index: usize, _child: &P::Widget) {
        unreachable!()
    }

    fn remove_child(&mut self, _platform: &mut P, _index: usize) {
        unreachable!()
    }

    fn swap_children(&mut self, _platform: &mut P, _index_a: usize, _index_b: usize) {
        unreachable!()
    }

    fn set_child_layout(
        &mut self,
        _platform: &mut P,
        _index: usize,
        _x: f32,
        _y: f32,
        _width: f32,
        _height: f32,
    ) {
        unreachable!()
    }

    fn set_background_color(&mut self, _platform: &mut P, _color: Color) {
        unreachable!()
    }

    fn set_border_color(&mut self, _platform: &mut P, _color: Color) {
        unreachable!()
    }

    fn set_border_width(&mut self, _platform: &mut P, _width: [f32; 4]) {
        unreachable!()
    }

    fn set_corner_radii(&mut self, _platform: &mut P, _radii: [f32; 4]) {
        unreachable!()
    }

    fn set_overflow(&mut self, _platform: &mut P, _overflow: Overflow) {
        unreachable!()
    }

    fn set_shadow(&mut self, _platform: &mut P, _shadow: Shadow) {
        unreachable!()
    }
}

pub struct Group<P>
where
    P: Platform,
{
    group:        P::Group,
    children:     Vec<Child<P>>,
    border_width: [f32; 4],
}

struct Child<P> {
    element:    BoxedWidget<P>,
    allocation: Option<Allocation>,
}

impl<P> Group<P>
where
    P: Platform,
{
    pub fn new(cx: &mut Context<P>) -> Self {
        Self {
            group:        P::Group::build(&mut cx.platform),
            children:     Vec::new(),
            border_width: [0.0; 4],
        }
    }

    pub fn teardown(self, cx: &mut Context<P>) {
        self.group.teardown(&mut cx.platform);
    }

    pub fn elements(&mut self, node: LayoutNode) -> impl Elements<Context<P>, BoxedWidget<P>> {
        GroupElements {
            node,
            index: 0,
            group: &mut self.group,
            children: &mut self.children,
        }
    }

    pub fn set_background(&mut self, cx: &mut Context<P>, color: Color) {
        self.group.set_background_color(&mut cx.platform, color);
    }

    pub fn set_border_color(&mut self, cx: &mut Context<P>, color: Color) {
        self.group.set_border_color(&mut cx.platform, color);
    }

    pub fn set_corner_radii(&mut self, cx: &mut Context<P>, radii: [f32; 4]) {
        self.group.set_corner_radii(&mut cx.platform, radii);
    }

    pub fn set_overflow(&mut self, cx: &mut Context<P>, overflow: Overflow) {
        self.group.set_overflow(&mut cx.platform, overflow);
    }

    pub fn set_shadow(&mut self, cx: &mut Context<P>, shadow: Shadow) {
        self.group.set_shadow(&mut cx.platform, shadow);
    }

    pub fn set_hardware_layer(&mut self, cx: &mut Context<P>, enabled: bool) {
        self.group.set_hardware_layer(&mut cx.platform, enabled);
    }

    pub fn layout(&mut self, cx: &mut Context<P>, node: LayoutNode) {
        if let Some(allocation) = cx.layout.get_allocation(node) {
            let border_width = [
                allocation.border.top,
                allocation.border.right,
                allocation.border.bottom,
                allocation.border.left,
            ];

            if self.border_width != border_width {
                self.border_width = border_width;
                self.group.set_border_width(&mut cx.platform, border_width);
            }
        }

        for (index, child) in self.children.iter_mut().enumerate() {
            if let Some(allocation) = cx.layout.get_allocation(child.element.node)
                && child.allocation != Some(allocation)
            {
                child.allocation = Some(allocation);
                self.group.set_child_layout(
                    &mut cx.platform,
                    index,
                    allocation.x,
                    allocation.y,
                    allocation.size.width,
                    allocation.size.height,
                );
            }
        }
    }
}

impl<P> NativeWidget<P> for Group<P>
where
    P: Platform,
{
    fn widget(&self) -> &P::Widget {
        self.group.widget()
    }
}

struct GroupElements<'a, P>
where
    P: Platform,
{
    node:     LayoutNode,
    index:    usize,
    group:    &'a mut P::Group,
    children: &'a mut Vec<Child<P>>,
}

impl<P> Elements<Context<P>, BoxedWidget<P>> for GroupElements<'_, P>
where
    P: Platform,
{
    fn next(&mut self, _cx: &mut Context<P>) -> Option<Mut<'_, BoxedWidget<P>>> {
        let child = self.children.get_mut(self.index)?;
        let pod = PodMut {
            parent_node:   self.node,
            parent_widget: self.group,

            node_index:   self.index,
            widget_index: self.index,

            node:   &mut child.element.node,
            widget: &mut child.element.widget,
        };

        self.index += 1;
        Some(pod)
    }

    fn insert(&mut self, cx: &mut Context<P>, element: BoxedWidget<P>) {
        cx.layout.insert_child(self.node, self.index, element.node);

        self.group.insert_child(
            &mut cx.platform,
            self.index,
            element.widget.widget(),
        );

        self.children.insert(
            self.index,
            Child {
                element,
                allocation: None,
            },
        );

        self.index += 1;
    }

    fn remove(&mut self, cx: &mut Context<P>) -> Option<BoxedWidget<P>> {
        self.group.remove_child(&mut cx.platform, self.index);
        let child = self.children.remove(self.index);
        cx.layout.remove_child(self.node, self.index);

        Some(child.element)
    }

    fn swap(&mut self, cx: &mut Context<P>, offset: usize) {
        cx.layout.replace_child(
            self.node,
            self.index,
            self.children[self.index + offset].element.node,
        );

        cx.layout.replace_child(
            self.node,
            self.index + offset,
            self.children[self.index].element.node,
        );

        self.group.swap_children(
            &mut cx.platform,
            self.index,
            self.index + offset,
        );

        self.children.swap(self.index, self.index + offset);
    }
}
