use std::ops::Range;

#[derive(Clone, Debug, PartialEq)]
pub struct FontAttributes {
    pub size:    f32,
    pub family:  String,
    pub weight:  FontWeight,
    pub stretch: FontStretch,
    pub italic:  bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FontWeight(pub u16);

impl FontWeight {
    pub const THIN: Self = Self(100);
    pub const EXTRA_LIGHT: Self = Self(200);
    pub const LIGHT: Self = Self(300);
    pub const NORMAL: Self = Self(400);
    pub const MEDIUM: Self = Self(500);
    pub const SEMI_BOLD: Self = Self(600);
    pub const BOLD: Self = Self(700);
    pub const EXTRA_BOLD: Self = Self(800);
    pub const HEAVY: Self = Self(900);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontStretch {
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UntraExpanded,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TextSpan {
    pub attributes: FontAttributes,
    pub range:      Range<usize>,
}
