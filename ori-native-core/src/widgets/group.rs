use crate::Platform;

pub trait HasGroup: Platform {
    type Group: NativeGroup<Self>;
}

pub trait NativeGroup<P>
where
    P: Platform,
{
    fn widget(&self) -> &P::Widget;

    fn build(platform: &mut P) -> Self;
    fn teardown(self, platform: &mut P);

    fn insert_child(&mut self, index: usize, child: &P::Widget);

    fn remove_child(&mut self, index: usize);

    fn swap_children(&mut self, index_a: usize, index_b: usize);

    fn set_size(&mut self, width: f32, height: f32);

    fn set_child_position(&mut self, index: usize, x: f32, y: f32);
}
