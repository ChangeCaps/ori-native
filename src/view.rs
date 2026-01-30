use ori_native_core::WidgetView;

use crate::{Context, Platform};

pub trait View<T>: WidgetView<Platform, T> {}
pub trait Effect<T>: ori::Effect<Context, T> {}

impl<T, V> View<T> for V where V: WidgetView<Platform, T> {}
impl<T, V> Effect<T> for V where V: ori::Effect<Context, T> {}
