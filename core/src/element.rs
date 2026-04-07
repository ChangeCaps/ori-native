use std::{
    any::{Any, TypeId},
    marker::PhantomData,
    mem,
};

use ori::{Element, Is, Mut, View, ViewSeq};

use crate::{Context, LayoutNode, Platform};

/// An [`Element`] in the [`View`] tree.
pub struct Pod<P, T> {
    /// The layout node of the [`Element`].
    pub node: LayoutNode,

    /// The native widget of the [`Element`].
    pub widget: T,

    marker: PhantomData<fn(&P)>,
}

impl<P, T> Pod<P, T> {
    /// Create new [`Pod`].
    pub fn new(node: LayoutNode, widget: T) -> Self {
        Self {
            node,
            widget,
            marker: PhantomData,
        }
    }

    /// Map the native widget keeping the `node`.
    pub fn map_widget<U>(self, widget: U) -> Pod<P, U> {
        Pod {
            node: self.node,
            widget,

            marker: PhantomData,
        }
    }

    /// Borrow `self` as a [`PodMut`].
    pub fn as_mut<'a>(
        &'a mut self,
        parent_node: LayoutNode,
        node_index: usize,
        parent_widget: &'a mut dyn NativeParent<P>,
        widget_index: usize,
    ) -> PodMut<'a, P, T> {
        PodMut {
            parent_node,
            parent_widget,
            node_index,
            widget_index,
            node: &mut self.node,
            widget: &mut self.widget,
        }
    }
}

/// A mutable [`Pod`] passed to [`View`]s.
pub struct PodMut<'a, P, T> {
    /// The layout node of the parent.
    pub parent_node: LayoutNode,

    /// The native parent widget.
    pub parent_widget: &'a mut dyn NativeParent<P>,

    /// The index of this in the parent layout node.
    pub node_index: usize,

    /// The index of this in the parent widget.
    pub widget_index: usize,

    /// The layout node of this [`Element`].
    pub node: &'a mut LayoutNode,

    /// The native widget of this [`Element`].
    pub widget: &'a mut T,
}

impl<P, T> PodMut<'_, P, T> {
    /// Reborrow `self` as a new [`PodMut`], useful for when lifetimes get tricky.
    pub fn reborrow(&mut self) -> PodMut<'_, P, T> {
        PodMut {
            parent_node:   self.parent_node,
            parent_widget: self.parent_widget,
            node_index:    self.node_index,
            widget_index:  self.widget_index,
            node:          self.node,
            widget:        self.widget,
        }
    }

    /// Map the widget of `self` with another widget, settings the old `widget` as the new
    /// `parent`.
    pub fn map_widget<'a, U>(&'a mut self, widget: &'a mut U, index: usize) -> PodMut<'a, P, U>
    where
        P: Platform,
        T: NativeParent<P>,
    {
        PodMut {
            parent_node: self.parent_node,
            parent_widget: self.widget,
            node_index: self.node_index,
            widget_index: index,
            node: self.node,
            widget,
        }
    }
}

impl<P, T> Element for Pod<P, T> {
    type Mut<'a>
        = PodMut<'a, P, T>
    where
        Self: 'a;
}

/// Type erased [`Pod`].
pub type BoxedWidget<P> = Pod<P, Box<dyn NativeWidget<P>>>;

/// A [`View`] with a [`Pod`] as its element.
pub trait WidgetView<P, T>: View<Context<P>, T, Element = Pod<P, Self::Widget>>
where
    P: Platform,
{
    /// The native widget.
    type Widget: NativeWidget<P>;
}

impl<P, T, V, W> WidgetView<P, T> for V
where
    P: Platform,
    V: View<Context<P>, T, Element = Pod<P, W>>,
    W: NativeWidget<P>,
{
    type Widget = W;
}

/// A [`ViewSeq`] with [`BoxedWidget`]s as elements.
pub trait WidgetViewSeq<P, T>: ViewSeq<Context<P>, T, BoxedWidget<P>>
where
    P: Platform,
{
}

impl<P, T, V> WidgetViewSeq<P, T> for V
where
    P: Platform,
    V: ViewSeq<Context<P>, T, BoxedWidget<P>>,
{
}

/// A native widget that has children.
pub trait NativeParent<P>
where
    P: Platform,
{
    /// Replace the child of `self` at `index`.
    fn replace_child(&mut self, platform: &mut P, index: usize, child: &P::Widget);
}

/// A native widget.
pub trait NativeWidget<P>: Any
where
    P: Platform,
{
    /// Get a reference to the [`Platform`] base widget.
    fn widget(&self) -> &P::Widget;
}

impl<P> NativeWidget<P> for Box<dyn NativeWidget<P>>
where
    P: Platform,
{
    fn widget(&self) -> &P::Widget {
        self.as_ref().widget()
    }
}

impl<P, T> Is<Context<P>, BoxedWidget<P>> for Pod<P, T>
where
    P: Platform,
    T: NativeWidget<P>,
{
    fn replace(cx: &mut Context<P>, other: Mut<'_, BoxedWidget<P>>, this: Self) -> BoxedWidget<P> {
        cx.layout.replace_child(
            other.parent_node,
            other.node_index,
            this.node,
        );

        other.parent_widget.replace_child(
            &mut cx.platform,
            other.widget_index,
            this.widget.widget(),
        );

        let widget = mem::replace(other.widget, Box::new(this.widget));
        let node = mem::replace(other.node, this.node);

        Pod {
            widget,
            node,
            marker: PhantomData,
        }
    }

    fn upcast(_cx: &mut Context<P>, this: Self) -> BoxedWidget<P> {
        Pod {
            node:   this.node,
            widget: Box::new(this.widget),
            marker: PhantomData,
        }
    }

    fn downcast(this: BoxedWidget<P>) -> Result<Self, BoxedWidget<P>> {
        if this.widget.as_ref().type_id() == TypeId::of::<T>() {
            let shadow = *Box::<dyn Any>::downcast(this.widget)
                .expect("type should be correct, as it was just checked");

            Ok(Pod {
                node:   this.node,
                widget: shadow,
                marker: PhantomData,
            })
        } else {
            Err(this)
        }
    }

    fn downcast_mut(
        this: Mut<'_, BoxedWidget<P>>,
    ) -> Result<Self::Mut<'_>, Mut<'_, BoxedWidget<P>>> {
        if this.widget.as_ref().type_id() == TypeId::of::<T>() {
            let shadow = <dyn Any>::downcast_mut(this.widget.as_mut())
                .expect("type should be correct, as it was just checked");

            Ok(PodMut {
                parent_node:   this.parent_node,
                parent_widget: this.parent_widget,

                node_index:   this.node_index,
                widget_index: this.widget_index,

                node:   this.node,
                widget: shadow,
            })
        } else {
            Err(this)
        }
    }
}
