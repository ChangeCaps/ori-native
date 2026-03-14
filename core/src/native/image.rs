use std::{borrow::Cow, convert::Infallible, error::Error};

use crate::{Color, Measure, NativeWidget, Platform, Unsupported, platform::unsupported};

pub trait NativeImage<P>: NativeWidget<P>
where
    P: Platform,
{
    type Error: Error;

    fn build(platform: &mut P) -> Self;
    fn teardown(self, platform: &mut P);

    fn load_data(
        &mut self,
        platform: &mut P,
        data: Cow<'static, [u8]>,
    ) -> Result<impl Measure<P>, Self::Error>;

    fn set_tint(&mut self, platform: &mut P, tint: Option<Color>);
}

impl<P> NativeImage<P> for Unsupported
where
    P: Platform,
{
    type Error = Infallible;

    fn build(_platform: &mut P) -> Self {
        unsupported!("image view")
    }

    fn teardown(self, _platform: &mut P) {
        unreachable!()
    }

    #[allow(refining_impl_trait)]
    fn load_data(
        &mut self,
        _platform: &mut P,
        _data: Cow<'static, [u8]>,
    ) -> Result<Infallible, Self::Error> {
        unreachable!()
    }

    fn set_tint(&mut self, _platform: &mut P, _tint: Option<Color>) {
        unreachable!()
    }
}
