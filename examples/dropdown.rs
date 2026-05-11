use ori_native::prelude::*;

fn main() {
    let mut data = Data { is_open: false };

    App::new().run(&mut data, ui).unwrap();
}

struct Data {
    is_open: bool,
}

fn ui(data: &Data) -> impl Effect<Data> + use<> {
    window(
        column(
            popup(
                pressable(|_, state| {
                    let color = match state.hovered {
                        true => Color::BLACK.fade(0.04),
                        false => Color::TRANSPARENT,
                    };

                    column(text("Click to open dropdown"))
                        .background(color)
                        .padding(8.0)
                        .border(1.0, Color::BLACK.fade(0.2))
                        .corner(8.0)
                })
                .on_press(|data: &mut Data| data.is_open = !data.is_open),
                data.is_open.then(|| {
                    column((text("stuff"), text("things")))
                        .margin(12.0)
                        .margin_top(0.0)
                        .padding(8.0)
                        .background(Color::hex("#f8f8f8"))
                        .border(1.0, Color::BLACK.fade(0.2))
                        .corner(8.0)
                        .shadow(4.0, 4.0, 8.0, Color::BLACK.fade(0.4))
                }),
            )
            .side(Side::Bottom)
            .on_dismiss(|data: &mut Data| data.is_open = false),
        )
        .background(Color::WHITE)
        .justify_content(Justify::Center)
        .align_items(Align::Center)
        .flex(1.0),
    )
    .title("Dropdown (examples/dropdown.rs)")
}
