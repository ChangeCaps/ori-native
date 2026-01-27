use std::any::{Any, TypeId};

use ori::{Element, Is, Mut, View};

use crate::{Context, Platform};

pub trait Shadow<P>: Any
where
    P: Platform,
{
    fn widget(&self) -> &P::Widget;

    fn layout(&mut self, cx: &mut Context<P>, node: taffy::NodeId);
}

pub trait ShadowView<P, T>: View<Context<P>, T, Element = Pod<Self::Shadow>>
where
    P: Platform,
{
    type Shadow: Shadow<P>;
}

impl<P, T, V, S> ShadowView<P, T> for V
where
    P: Platform,
    V: View<Context<P>, T, Element = Pod<S>>,
    S: Shadow<P>,
{
    type Shadow = S;
}

pub struct Pod<T> {
    pub node:   taffy::NodeId,
    pub shadow: T,
}

impl<T> Pod<T> {
    pub fn as_mut(&mut self, parent: taffy::NodeId) -> PodMut<'_, T> {
        PodMut {
            parent,
            node: &mut self.node,
            shadow: &mut self.shadow,
        }
    }
}

pub struct PodMut<'a, T> {
    pub parent: taffy::NodeId,
    pub node:   &'a mut taffy::NodeId,
    pub shadow: &'a mut T,
}

impl<T> Element for Pod<T> {
    type Mut<'a>
        = PodMut<'a, T>
    where
        Self: 'a;
}

pub type AnyShadow<P> = Pod<Box<dyn Shadow<P>>>;

impl<P, T> Is<Context<P>, AnyShadow<P>> for Pod<T>
where
    P: Platform,
    T: Shadow<P>,
{
    fn replace(_cx: &mut Context<P>, _other: Mut<'_, AnyShadow<P>>, _this: Self) -> AnyShadow<P> {
        todo!()
    }

    fn upcast(_cx: &mut Context<P>, this: Self) -> AnyShadow<P> {
        Pod {
            node:   this.node,
            shadow: Box::new(this.shadow),
        }
    }

    fn downcast(this: AnyShadow<P>) -> Result<Self, AnyShadow<P>> {
        if this.shadow.as_ref().type_id() == TypeId::of::<T>() {
            let shadow = *Box::<dyn Any>::downcast(this.shadow)
                .expect("type should be correct, as it was just checked");

            Ok(Pod {
                node: this.node,
                shadow,
            })
        } else {
            Err(this)
        }
    }

    fn downcast_mut(this: Mut<'_, AnyShadow<P>>) -> Result<Self::Mut<'_>, Mut<'_, AnyShadow<P>>> {
        if this.shadow.as_ref().type_id() == TypeId::of::<T>() {
            let shadow = <dyn Any>::downcast_mut(this.shadow.as_mut())
                .expect("type should be correct, as it was just checked");

            Ok(PodMut {
                parent: this.parent,
                node: this.node,
                shadow,
            })
        } else {
            Err(this)
        }
    }
}
