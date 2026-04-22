use std::{marker::PhantomData, time::Duration};

use ori::Element;

use crate::{Context, LayoutNode, Platform, Widget, WidgetMut};

/// [`Widget`] with a callback on animate.
pub struct AnimateWidget<P, W>
where
    P: Platform,
    W: Widget<P>,
{
    contents:   W,
    on_animate: Box<dyn Fn(Duration)>,

    marker: PhantomData<fn(P)>,
}

impl<P, W> AnimateWidget<P, W>
where
    P: Platform,
    W: Widget<P>,
{
    /// Create new [`AnimateWidget`].
    pub fn new(contents: W, on_animate: impl Fn(Duration) + 'static) -> Self {
        Self {
            contents,
            on_animate: Box::new(on_animate),
            marker: PhantomData,
        }
    }

    /// Teardown returning contents.
    pub fn teardown(self) -> W {
        self.contents
    }

    /// Get mutable reference to contents.
    pub fn contents(&mut self) -> &mut W {
        &mut self.contents
    }
}

impl<P, W> Widget<P> for AnimateWidget<P, W>
where
    P: Platform,
    W: Widget<P>,
{
    fn widget_ref(&self) -> P::WidgetRef {
        self.contents.widget_ref()
    }

    fn layout_node(&self) -> LayoutNode {
        self.contents.layout_node()
    }

    fn layout(&mut self, cx: &mut Context<P>) {
        self.contents.layout(cx);
    }

    fn animate(&mut self, cx: &mut Context<P>, dt: Duration) {
        (self.on_animate)(dt);
        self.contents.animate(cx, dt);
    }
}

impl<P, W> Element for AnimateWidget<P, W>
where
    P: Platform,
    W: Widget<P>,
{
    type Mut<'a>
        = WidgetMut<'a, P, Self>
    where
        Self: 'a;
}
