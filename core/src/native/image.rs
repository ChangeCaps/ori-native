use std::{borrow::Cow, convert::Infallible, error::Error};

use crate::{Color, LayoutLeaf, NativeWidget, Platform, Unsupported, platform::unsupported};

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

impl<P> NativeImage<P> for Unsupported
where
    P: Platform,
{
    type Error = Infallible;

    fn build(_plaform: &mut P) -> Self {
        unsupported!("image view")
    }

    fn teardown(self, _plaform: &mut P) {
        unreachable!()
    }

    #[allow(refining_impl_trait)]
    fn load_data(
        &mut self,
        _plaform: &mut P,
        _data: Cow<'static, [u8]>,
    ) -> Result<Infallible, Self::Error> {
        unreachable!()
    }

    fn set_tint(&mut self, _tint: Option<Color>) {
        unreachable!()
    }
}
