use std::{borrow::Cow, io};

use gdk4::prelude::PaintableExt;
use glib::subclass::types::ObjectSubclassIsExt;
use librsvg::prelude::HandleExt;
use ori_native_core::{
    Color, LayoutLeaf, NativeWidget,
    native::{HasImage, NativeImage},
};

use crate::Platform;

impl HasImage for Platform {
    type Image = Image;
}

pub struct Image {
    image: gtk4::Picture,
    svg:   Option<Svg>,
    tint:  Option<Color>,
}

impl NativeWidget<Platform> for Image {
    fn widget(&self) -> &gtk4::Widget {
        self.image.as_ref()
    }
}

impl NativeImage<Platform> for Image {
    type Error = io::Error;

    fn build(_plaform: &mut Platform) -> Self {
        let image = gtk4::Picture::new();

        Self {
            image,
            svg: None,
            tint: None,
        }
    }

    fn teardown(self, _plaform: &mut Platform) {}

    fn load_data(
        &mut self,
        _plaform: &mut Platform,
        data: Cow<'static, [u8]>,
    ) -> Result<impl LayoutLeaf<Platform>, Self::Error> {
        let svg = Svg::new(&data)?;
        svg.set_tint(self.tint);
        self.image.set_paintable(Some(&svg));

        Ok(Layout { svg })
    }

    fn set_tint(&mut self, tint: Option<Color>) {
        self.tint = tint;

        if let Some(ref svg) = self.svg {
            svg.set_tint(tint);
        }
    }
}

struct Layout {
    svg: Svg,
}

impl LayoutLeaf<Platform> for Layout {
    fn measure(
        &mut self,
        _platform: &mut Platform,
        _known_size: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32> {
        let (width, height) = self.svg.intrinsic_size().unwrap_or((0.0, 0.0));

        taffy::Size {
            width:  width as f32,
            height: height as f32,
        }
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

    fn set_tint(&self, tint: Option<Color>) {
        if self.imp().tint.replace(tint) != tint {
            self.invalidate_contents();
        }
    }

    fn intrinsic_size(&self) -> Option<(f64, f64)> {
        self.imp().handle.borrow().intrinsic_size_in_pixels()
    }
}

mod imp {
    use std::cell::{Cell, RefCell};

    use gdk4::subclass::prelude::PaintableImpl;
    use glib::subclass::{object::ObjectImpl, types::ObjectSubclass};
    use gtk4::prelude::SnapshotExt;
    use librsvg::prelude::HandleExt;
    use ori_native_core::Color;

    #[derive(Default)]
    pub(super) struct Svg {
        pub(super) handle: RefCell<librsvg::Handle>,
        pub(super) tint:   Cell<Option<Color>>,
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
        fn snapshot(&self, snapshot: &gdk4::Snapshot, width: f64, height: f64) {
            let cr = snapshot.append_cairo(&graphene::Rect::new(
                0.0,
                0.0,
                width as f32,
                height as f32,
            ));

            if self.tint.get().is_some() {
                cr.push_group();
            }

            let _ = self.handle.borrow().render_document(
                &cr,
                &librsvg::Rectangle::new(0.0, 0.0, width, height),
            );

            if let Some(tint) = self.tint.get()
                && let Ok(mask) = cr.pop_group()
            {
                cr.set_source_rgba(
                    tint.r as f64,
                    tint.g as f64,
                    tint.b as f64,
                    tint.a as f64,
                );

                let _ = cr.mask(&mask);
            }
        }
    }
}
