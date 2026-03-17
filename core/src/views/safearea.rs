use ori::{BuildMarker, BuildView, views::using_or_default};

use crate::{Context, Layout, Platform, SafeAreaInsets, WidgetView, views::Flex};

/// [`View`](ori::View) that ensures contents isn't overlapped by system elements.
pub fn safe_area<V>(contents: V) -> SafeArea<V> {
    SafeArea::new(contents)
}

/// [`View`](ori::View) that ensures contents isn't overlapped by system elements.
pub struct SafeArea<V> {
    contents: V,
    style:    taffy::Style,
}

impl<V> SafeArea<V> {
    /// Create new [`SafeArea`].
    pub fn new(contents: V) -> Self {
        Self {
            contents,
            style: taffy::Style {
                display: taffy::Display::Flex,
                ..Default::default()
            },
        }
    }
}

impl<V> Layout for SafeArea<V> {
    fn get_layout_mut(&mut self) -> &mut taffy::Style {
        &mut self.style
    }
}

impl<V> BuildMarker for SafeArea<V> {}
impl<P, T, V> BuildView<Context<P>, T> for SafeArea<V>
where
    P: Platform,
    T: 'static,
    V: WidgetView<P, T> + 'static,
{
    #[allow(refining_impl_trait)]
    fn build(mut self) -> impl WidgetView<P, T> + 'static {
        using_or_default(|_, insets: &SafeAreaInsets| {
            self.style.padding.top = taffy::LengthPercentage::length(insets.top);
            self.style.padding.right = taffy::LengthPercentage::length(insets.right);
            self.style.padding.bottom = taffy::LengthPercentage::length(insets.bottom);
            self.style.padding.left = taffy::LengthPercentage::length(insets.left);

            let mut flex = Flex::new(self.contents);
            *flex.get_layout_mut() = self.style;

            flex
        })
    }
}
