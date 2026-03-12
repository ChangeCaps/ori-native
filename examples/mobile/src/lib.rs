use ori_native::prelude::*;

#[ori_native::main]
fn main() {
    App::init_log();

    let mut data = Data {};

    App::new().run(&mut data, ui).unwrap();
}

struct Data {}

fn ui(_data: &Data) -> impl Effect<Data> + use<> {
    window(
        column(
            row(text("hello").size(30.0).weight(Weight::BOLD))
                .background_color(Color::RED)
                .corner(8.0)
                .border(2.0)
                .padding(16.0)
                .border_color(Color::BLACK),
        )
        .flex(1.0)
        .justify_contents(Justify::Center)
        .align_items(Align::Center),
    )
}
