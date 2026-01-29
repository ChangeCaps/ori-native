use std::time::Duration;

#[derive(Clone, Debug)]
pub enum Lifecycle {
    Animate(Duration),
    Layout,
}
