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
    pub layout: LayoutNode,

    /// The native widget of the [`Element`].
    pub widget: T,

    marker: PhantomData<fn(&P)>,
}

impl<P, T> Pod<P, T> {
    /// Create new [`Pod`].
    pub fn new(layout: LayoutNode, widget: T) -> Self {
        Self {
            layout,
            widget,
            marker: PhantomData,
        }
    }

    /// Map the native widget keeping the `node`.
    pub fn map_widget<U>(self, widget: U) -> Pod<P, U> {
        Pod {
            layout: self.layout,
            widget,

            marker: PhantomData,
        }
    }

    /// Get a [`Platform::WidgetRef`] to the this widget.
    pub fn widget_ref(&self) -> &P::WidgetRef
    where
        P: Platform,
        T: NativeWidget<P>,
    {
        self.widget.widget_ref()
    }

    /// Borrow `self` as a [`PodMut`].
    pub fn as_mut<'a>(
        &'a mut self,
        parent_layout: LayoutNode,
        layout_index: usize,
        parent_widget: &'a mut dyn NativeParent<P>,
        widget_index: usize,
    ) -> PodMut<'a, P, T> {
        PodMut {
            parent_layout: Some(ParentLayout {
                layout: parent_layout,
                index:  layout_index,
            }),

            parent_widget,
            widget_index,

            layout: &mut self.layout,
            widget: &mut self.widget,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ParentLayout {
    pub layout: LayoutNode,
    pub index:  usize,
}

/// A mutable [`Pod`] passed to [`View`]s.
pub struct PodMut<'a, P, T> {
    /// The layout node of the parent.
    pub parent_layout: Option<ParentLayout>,

    /// The native parent widget.
    pub parent_widget: &'a mut dyn NativeParent<P>,

    /// The index of this in the parent widget.
    pub widget_index: usize,

    /// The layout node of this [`Element`].
    pub layout: &'a mut LayoutNode,

    /// The native widget of this [`Element`].
    pub widget: &'a mut T,
}

impl<P, T> PodMut<'_, P, T> {
    /// Reborrow `self` as a new [`PodMut`], useful for when lifetimes get tricky.
    pub fn reborrow(&mut self) -> PodMut<'_, P, T> {
        PodMut {
            parent_layout: self.parent_layout,
            parent_widget: self.parent_widget,
            widget_index:  self.widget_index,

            layout: self.layout,
            widget: self.widget,
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
            parent_layout: self.parent_layout,
            parent_widget: self.widget,
            widget_index: index,
            layout: self.layout,
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
    fn replace_child(&mut self, platform: &mut P, index: usize, child: &P::WidgetRef);
}

/// A native widget.
pub trait NativeWidget<P>: Any
where
    P: Platform,
{
    /// Get a reference to the [`Platform`] base widget.
    fn widget_ref(&self) -> &P::WidgetRef;
}

impl<P> NativeWidget<P> for Box<dyn NativeWidget<P>>
where
    P: Platform,
{
    fn widget_ref(&self) -> &P::WidgetRef {
        self.as_ref().widget_ref()
    }
}

impl<P, T> Is<Context<P>, BoxedWidget<P>> for Pod<P, T>
where
    P: Platform,
    T: NativeWidget<P>,
{
    fn replace(cx: &mut Context<P>, other: Mut<'_, BoxedWidget<P>>, this: Self) -> BoxedWidget<P> {
        match other.parent_layout {
            Some(layout) => (cx.layout).replace_child(layout.layout, layout.index, this.layout),
            None => (cx.layout).replace_node(*other.layout, this.layout),
        }

        other.parent_widget.replace_child(
            &mut cx.platform,
            other.widget_index,
            this.widget.widget_ref(),
        );

        let widget = mem::replace(other.widget, Box::new(this.widget));
        let node = mem::replace(other.layout, this.layout);

        Pod {
            widget,
            layout: node,
            marker: PhantomData,
        }
    }

    fn upcast(_cx: &mut Context<P>, this: Self) -> BoxedWidget<P> {
        Pod {
            layout: this.layout,
            widget: Box::new(this.widget),
            marker: PhantomData,
        }
    }

    fn downcast(this: BoxedWidget<P>) -> Result<Self, BoxedWidget<P>> {
        if this.widget.as_ref().type_id() == TypeId::of::<T>() {
            let shadow = *Box::<dyn Any>::downcast(this.widget)
                .expect("type should be correct, as it was just checked");

            Ok(Pod {
                layout: this.layout,
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
                parent_layout: this.parent_layout,
                parent_widget: this.parent_widget,
                widget_index:  this.widget_index,

                layout: this.layout,
                widget: shadow,
            })
        } else {
            Err(this)
        }
    }
}
