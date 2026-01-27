use ori::{Action, AnyView, Base, Proxied};

use crate::{AnyShadow, Platform};

pub trait LayoutLeaf<P> {
    fn measure(
        &mut self,
        platform: &mut P,
        known_size: taffy::Size<Option<f32>>,
        available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32>;
}

pub struct Context<P> {
    pub platform:    P,
    pub layout_tree: taffy::TaffyTree<Box<dyn LayoutLeaf<P>>>,
}

impl<P> Context<P> {
    pub fn new(platform: P) -> Self {
        Self {
            platform,
            layout_tree: taffy::TaffyTree::new(),
        }
    }

    pub fn compute_layout(
        &mut self,
        node: taffy::NodeId,
        available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::TaffyResult<()>
    where
        P: Platform,
    {
        self.layout_tree.compute_layout_with_measure(
            node,
            available_space,
            |known_size, available_space, _node, context, _style| match context {
                Some(leaf) => leaf.measure(
                    &mut self.platform,
                    known_size,
                    available_space,
                ),

                None => taffy::Size::ZERO,
            },
        )
    }
}

pub type BoxedEffect<P, T> = Box<dyn AnyView<Context<P>, T, ()>>;

impl<P> Base for Context<P> {
    type Element = AnyShadow<P>;
}

impl<P> Proxied for Context<P>
where
    P: Proxied,
{
    type Proxy = P::Proxy;

    fn proxy(&mut self) -> Self::Proxy {
        self.platform.proxy()
    }

    fn send_action(&mut self, action: Action) {
        self.platform.send_action(action);
    }
}
