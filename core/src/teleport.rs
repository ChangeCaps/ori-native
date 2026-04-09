use ori::{Message, Mut, Split, Teleportable};

use crate::{Context, Lifecycle, NativeWidget, Platform, Pod, PodMut, native::NativeGroup};

impl<P> Teleportable for Context<P>
where
    P: Platform,
{
    type Left = Pod<P, WidgetProxy<P>>;
}

pub struct WidgetProxy<P>(P::WidgetRef)
where
    P: Platform;

impl<P> NativeWidget<P> for WidgetProxy<P>
where
    P: Platform,
{
    fn widget_ref(&self) -> &P::WidgetRef {
        &self.0
    }
}

impl<P, T> Split<Context<P>> for Pod<P, T>
where
    P: Platform,
    T: NativeWidget<P>,
{
    type Right = PodRight<P, T>;

    fn split(self, cx: &mut Context<P>) -> (Pod<P, WidgetProxy<P>>, Self::Right) {
        let mut group = P::Group::build(&mut cx.platform);
        group.insert_child(&mut cx.platform, 0, self.widget_ref());

        let proxy = WidgetProxy::<P>(group.widget_ref().clone());
        let left = Pod::new(self.layout, proxy);
        let right = PodRight { group, pod: self };

        (left, right)
    }

    fn as_mut<'a>(right: &'a mut Self::Right, _cx: &mut Context<P>) -> Mut<'a, Self> {
        PodMut {
            parent_layout: None,
            parent_widget: &mut right.group,
            widget_index:  0,

            layout: &mut right.pod.layout,
            widget: &mut right.pod.widget,
        }
    }

    fn message(right: &mut Self::Right, cx: &mut Context<P>, message: &mut Message) {
        if let Some(Lifecycle::Layout) = message.get()
            && let Some(allocation) = cx.layout.get_allocation(right.pod.layout)
        {
            right.group.set_child_layout(
                &mut cx.platform,
                0,
                0.0,
                0.0,
                allocation.size.width,
                allocation.size.height,
            );
        }
    }

    fn teardown(right: Self::Right, cx: &mut Context<P>) -> Self {
        right.group.teardown(&mut cx.platform);
        right.pod
    }
}

pub struct PodRight<P, T>
where
    P: Platform,
{
    group: P::Group,
    pod:   Pod<P, T>,
}
