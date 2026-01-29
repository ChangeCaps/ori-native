use crate::{LayoutLeaf, Platform, TextSpan};

pub trait HasText: Platform {
    type Text: NativeText<Self>;
}

pub trait NativeText<P>: Sized
where
    P: Platform,
{
    type Layout: LayoutLeaf<P>;

    fn widget(&self) -> &P::Widget;

    fn build(platform: &mut P, spans: Box<[TextSpan]>, text: String) -> (Self, Self::Layout);
    fn teardown(self, platform: &mut P);

    fn set_text(&mut self, spans: Box<[TextSpan]>, text: String) -> Self::Layout;
}
