use ori_native_core::{WidgetView, WidgetViewSeq};

use crate::{Context, Platform};

/// A [`View`](ori::View) in the selected [`Context`].
pub trait View<T>: WidgetView<Platform, T> {}

/// A [`ViewSeq`](ori::ViewSeq) in the selected [`Context`].
pub trait ViewSeq<T>: WidgetViewSeq<Platform, T> {}

/// An [`Effect`](ori::Effect) in the selected [`Context`].
pub trait Effect<T>: ori::Effect<Context, T> {}

impl<T, V> View<T> for V where V: WidgetView<Platform, T> {}
impl<T, V> ViewSeq<T> for V where V: WidgetViewSeq<Platform, T> {}
impl<T, V> Effect<T> for V where V: ori::Effect<Context, T> {}
