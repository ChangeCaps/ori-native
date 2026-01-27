use ori_native::{App, Effect};
use ori_native_core::{
    Align, FlexContainer, FlexItem, Justify,
    views::{flex_row, text, window},
};

fn main() {
    let mut data = Data { count: 0 };

    App::new().run(&mut data, ui);
}

struct Data {
    count: u32,
}

fn ui(_data: &Data) -> impl Effect<Data> + use<> {
    window(
        flex_row((text("hello"), text("wahoo")))
            .flex(1.0)
            .justify_contents(Justify::SpaceAround)
            .align_items(Align::Center),
    )
}
