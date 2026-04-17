use std::{borrow::Cow, convert::Infallible, error::Error};

use crate::{Color, Measurable, NativeWidget, Platform, Unsupported, platform::unsupported};

/// A native image widget.
pub trait NativeImage<P>: NativeWidget<P>
where
    P: Platform,
{
    /// An error that might occur when loading an image.
    type Error: Error;

    /// Build an empty image.
    fn build(platform: &mut P) -> Self;

    /// Teardown the image.
    fn teardown(self, platform: &mut P);

    /// Load an image from `data`.
    fn load_data(
        &mut self,
        platform: &mut P,
        data: Cow<'static, [u8]>,
    ) -> Result<impl Measurable<P>, Self::Error>;

    /// Set the `tint`.
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
