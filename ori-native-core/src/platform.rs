pub trait Platform: Sized + 'static {
    type Widget;

    fn quit(&mut self);
}
