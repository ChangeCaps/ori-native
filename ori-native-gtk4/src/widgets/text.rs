use gtk4::prelude::{TextBufferExt, TextBufferExtManual, TextTagExt, TextViewExt, WidgetExt};
use ori_native_core::{
    FontStretch, FontWeight, LayoutLeaf, TextSpan,
    widgets::{HasText, NativeText},
};

use crate::Gtk4Platform;

pub struct Text {
    view: gtk4::TextView,
}

impl NativeText<Gtk4Platform> for Text {
    type Leaf = TextLeaf;

    fn widget(&self) -> &gtk4::Widget {
        self.view.as_ref()
    }

    fn build(
        _platform: &mut Gtk4Platform,
        spans: Box<[TextSpan]>,
        text: String,
    ) -> (Self, Self::Leaf) {
        let view = gtk4::TextView::new();
        view.set_editable(false);
        view.set_cursor_visible(false);

        let buffer = view.buffer();
        let mut iter = buffer.start_iter();

        for span in &spans {
            let tag = gtk4::TextTag::new(None);
            tag.set_size((span.attributes.size * pango::SCALE as f32) as i32);
            tag.set_family(Some(&span.attributes.family));
            tag.set_weight(span.attributes.weight.0 as i32);
            tag.set_stretch(convert_stretch(span.attributes.stretch));
            tag.set_style(match span.attributes.italic {
                false => pango::Style::Normal,
                true => pango::Style::Italic,
            });

            buffer.tag_table().add(&tag);
            buffer.insert_with_tags(&mut iter, &text, &[&tag]);
        }

        let leaf = TextLeaf {
            view: view.clone(),
            spans,
            text,
        };

        let text = Self { view };

        (text, leaf)
    }

    fn teardown(self, _platform: &mut Gtk4Platform) {}

    fn set_text(&mut self, spans: Box<[TextSpan]>, text: String) -> Self::Leaf {
        self.view.buffer().set_text(&text);

        TextLeaf {
            view: self.view.clone(),
            spans,
            text,
        }
    }

    fn set_size(&mut self, width: f32, height: f32) {
        self.view.set_width_request(width as i32 + 1);
        self.view.set_height_request(height as i32);
    }
}

impl HasText for Gtk4Platform {
    type Text = Text;
}

pub struct TextLeaf {
    view:  gtk4::TextView,
    spans: Box<[TextSpan]>,
    text:  String,
}

impl LayoutLeaf<Gtk4Platform> for TextLeaf {
    fn measure(
        &mut self,
        _platform: &mut Gtk4Platform,
        _known_size: taffy::Size<Option<f32>>,
        _available_space: taffy::Size<taffy::AvailableSpace>,
    ) -> taffy::Size<f32> {
        let context = self.view.pango_context();
        let layout = pango::Layout::new(&context);

        layout.set_text(&self.text);

        let attrs = pango::AttrList::new();

        for span in &self.spans {
            let mut desc = pango::FontDescription::new();
            desc.set_size((span.attributes.size * pango::SCALE as f32) as i32);
            desc.set_family(&span.attributes.family);
            desc.set_weight(convert_weight(span.attributes.weight));
            desc.set_stretch(convert_stretch(span.attributes.stretch));
            desc.set_style(match span.attributes.italic {
                false => pango::Style::Normal,
                true => pango::Style::Italic,
            });

            let mut attr = pango::AttrFontDesc::new(&desc);
            attr.set_start_index(span.range.start as u32);
            attr.set_end_index(span.range.end as u32);

            attrs.insert(attr);
        }

        layout.set_attributes(Some(&attrs));

        let (width, height) = layout.pixel_size();

        taffy::Size {
            width:  width as f32,
            height: height as f32,
        }
    }
}

fn convert_weight(weight: FontWeight) -> pango::Weight {
    match weight {
        FontWeight(100) => pango::Weight::Thin,
        FontWeight(200) => pango::Weight::Ultralight,
        FontWeight(300) => pango::Weight::Light,
        FontWeight(350) => pango::Weight::Semilight,
        FontWeight(380) => pango::Weight::Book,
        FontWeight(400) => pango::Weight::Normal,
        FontWeight(500) => pango::Weight::Medium,
        FontWeight(600) => pango::Weight::Semibold,
        FontWeight(700) => pango::Weight::Bold,
        FontWeight(800) => pango::Weight::Ultrabold,
        FontWeight(900) => pango::Weight::Heavy,
        FontWeight(1000) => pango::Weight::Ultraheavy,
        FontWeight(..) => pango::Weight::Normal,
    }
}

fn convert_stretch(stretch: FontStretch) -> pango::Stretch {
    match stretch {
        FontStretch::UltraCondensed => pango::Stretch::UltraCondensed,
        FontStretch::ExtraCondensed => pango::Stretch::ExtraCondensed,
        FontStretch::Condensed => pango::Stretch::Condensed,
        FontStretch::SemiCondensed => pango::Stretch::SemiCondensed,
        FontStretch::Normal => pango::Stretch::Normal,
        FontStretch::SemiExpanded => pango::Stretch::SemiExpanded,
        FontStretch::Expanded => pango::Stretch::Expanded,
        FontStretch::ExtraExpanded => pango::Stretch::ExtraExpanded,
        FontStretch::UntraExpanded => pango::Stretch::UltraExpanded,
    }
}
