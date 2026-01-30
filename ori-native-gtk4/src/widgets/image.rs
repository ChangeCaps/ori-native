use std::{borrow::Cow, io};

use glib::subclass::types::ObjectSubclassIsExt;
use ori_native_core::{
    NativeWidget,
    native::{HasImage, NativeImage},
};

use crate::Platform;

impl HasImage for Platform {
    type Image = Image;
}

pub struct Image {
    image: gtk4::Image,
}

impl NativeWidget<Platform> for Image {
    fn widget(&self) -> &gtk4::Widget {
        self.image.as_ref()
    }
}

impl NativeImage<Platform> for Image {
    type Error = io::Error;

    fn build(_plaform: &mut Platform) -> Self {
        let image = gtk4::Image::new();

        Self { image }
    }

    fn teardown(self, _plaform: &mut Platform) {}

    fn load_data(
        &mut self,
        _plaform: &mut Platform,
        data: Cow<'static, [u8]>,
    ) -> Result<(), Self::Error> {
        let svg = Svg::new(&data)?;
        self.image.set_paintable(Some(&svg));

        Ok(())
    }
}

glib::wrapper! {
    struct Svg(ObjectSubclass<imp::Svg>)
        @implements
            gdk4::Paintable;
}

impl Svg {
    fn new(data: &[u8]) -> io::Result<Self> {
        let handle = librsvg::Handle::from_data(data)
            .map_err(io::Error::other)?
            .ok_or_else(|| io::Error::other("no handle"))?;

        let this: Self = glib::Object::builder().build();
        this.imp().handle.replace(handle);

        Ok(this)
    }
}

mod imp {
    use std::cell::RefCell;

    use gdk4::subclass::prelude::PaintableImpl;
    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use gtk4::prelude::SnapshotExt;
    use librsvg::prelude::HandleExt;

    #[derive(Default)]
    pub(super) struct Svg {
        pub(super) handle: RefCell<librsvg::Handle>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Svg {
        const NAME: &'static str = "OriSvg";

        type Type = super::Svg;
        type ParentType = glib::Object;
        type Interfaces = (gdk4::Paintable,);
    }

    impl ObjectImpl for Svg {}

    impl PaintableImpl for Svg {
        fn intrinsic_width(&self) -> i32 {
            self.handle
                .borrow()
                .intrinsic_size_in_pixels()
                .map_or(0, |(x, _)| x.round() as i32)
        }

        fn intrinsic_height(&self) -> i32 {
            self.handle
                .borrow()
                .intrinsic_size_in_pixels()
                .map_or(0, |(_, y)| y.round() as i32)
        }

        fn intrinsic_aspect_ratio(&self) -> f64 {
            self.handle
                .borrow()
                .intrinsic_size_in_pixels()
                .map_or(1.0, |(x, y)| x / y)
        }

        fn snapshot(&self, snapshot: &gdk4::Snapshot, width: f64, height: f64) {
            let cr = snapshot.append_cairo(&graphene::Rect::new(
                0.0,
                0.0,
                width as f32,
                height as f32,
            ));

            let _ = self.handle.borrow().render_document(
                &cr,
                &librsvg::Rectangle::new(0.0, 0.0, width, height),
            );
        }
    }
}
