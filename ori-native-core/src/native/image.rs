use std::{borrow::Cow, error::Error};

use crate::{Color, LayoutLeaf, NativeWidget, Platform};

pub trait HasImage: Platform {
    type Image: NativeImage<Self>;
}

pub trait NativeImage<P>: NativeWidget<P>
where
    P: Platform,
{
    type Error: Error;

    fn build(plaform: &mut P) -> Self;
    fn teardown(self, plaform: &mut P);

    fn load_data(
        &mut self,
        plaform: &mut P,
        data: Cow<'static, [u8]>,
    ) -> Result<impl LayoutLeaf<P>, Self::Error>;

    fn set_tint(&mut self, tint: Option<Color>);
}
