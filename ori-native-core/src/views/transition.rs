use std::{f32::consts::PI, time::Duration};

use crate::{Color, Platform, ShadowView, views::animate};

const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;

pub trait Transition {
    fn duration(&self) -> f32;
    fn curve(&self, t: f32) -> f32;
}

pub struct Linear(pub f32);

impl Transition for Linear {
    fn duration(&self) -> f32 {
        self.0
    }

    fn curve(&self, t: f32) -> f32 {
        t
    }
}

pub struct Ease(pub f32);

impl Transition for Ease {
    fn duration(&self) -> f32 {
        self.0
    }

    fn curve(&self, t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t)
    }
}

pub struct Elastic(pub f32);

impl Transition for Elastic {
    fn duration(&self) -> f32 {
        self.0
    }

    fn curve(&self, t: f32) -> f32 {
        1.0 + f32::powf(2.0, -10.0 * t) * f32::sin((10.0 * t - 0.75) * PI * 2.0 / 3.0)
    }
}

pub struct ElasticIn(pub f32);

impl Transition for ElasticIn {
    fn duration(&self) -> f32 {
        self.0
    }

    fn curve(&self, t: f32) -> f32 {
        -f32::powf(2.0, 10.0 * t - 10.0) * f32::sin((10.0 * t - 10.75) * PI * 2.0 / 3.0)
    }
}

pub struct Back(pub f32);

impl Transition for Back {
    fn duration(&self) -> f32 {
        self.0
    }

    fn curve(&self, t: f32) -> f32 {
        1.0 + (C1 + 1.0) * f32::powi(t - 1.0, 3) + C1 * f32::powi(t - 1.0, 2)
    }
}

pub struct BackIn(pub f32);

impl Transition for BackIn {
    fn duration(&self) -> f32 {
        self.0
    }

    fn curve(&self, t: f32) -> f32 {
        (C1 + 1.0) * f32::powi(t, 3) - C1 * f32::powi(t, 2)
    }
}

pub struct BackInOut(pub f32);

impl Transition for BackInOut {
    fn duration(&self) -> f32 {
        self.0
    }

    fn curve(&self, t: f32) -> f32 {
        if t < 0.5 {
            (f32::powi(2.0 * t, 2) * ((C2 + 1.0) * 2.0 * t - C2)) / 2.0
        } else {
            (f32::powi(2.0 * t - 2.0, 2) * ((C2 + 1.0) * (2.0 * t - 2.0) + C2) + 2.0) / 2.0
        }
    }
}

pub fn transition<P, T, U, V>(
    value: U,
    transition: impl Transition,
    build: impl Fn(U, &T) -> V,
) -> impl ShadowView<P, T>
where
    P: Platform,
    U: Clone + PartialEq + Lerp,
    V: ShadowView<P, T>,
{
    let state = State::new(value.clone(), transition);

    animate(
        move || state,
        move |state| state.animating(value),
        |state, delta_time| state.animate(delta_time),
        move |state, data| build(state.value(), data),
    )
}

struct State<U, T> {
    current:    Option<U>,
    target:     U,
    time:       f32,
    transition: T,
}

impl<U, X> State<U, X>
where
    U: Clone + PartialEq + Lerp,
    X: Transition,
{
    fn new(value: U, transition: X) -> Self {
        Self {
            current: None,
            target: value,
            time: 1.0,
            transition,
        }
    }

    fn animating(&mut self, target: U) -> bool {
        if self.target != target {
            self.current = Some(self.value());
            self.target = target;
            self.time = 0.0;
        }

        self.time < 1.0
    }

    fn animate(&mut self, delta_time: Duration) -> bool {
        self.time += delta_time.as_secs_f32() / self.transition.duration();
        self.time = self.time.clamp(0.0, 1.0);
        self.time < 1.0
    }

    fn value(&self) -> U {
        match self.current {
            Some(ref current) => {
                let t = self.transition.curve(self.time);
                U::lerp(current, &self.target, t)
            }

            None => self.target.clone(),
        }
    }
}

pub trait Lerp {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        *a * (1.0 - t) + *b * t
    }
}

impl Lerp for Color {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Color {
            r: f32::lerp(&a.r, &b.r, t),
            g: f32::lerp(&a.g, &b.g, t),
            b: f32::lerp(&a.b, &b.b, t),
            a: f32::lerp(&a.a, &b.a, t),
        }
    }
}
