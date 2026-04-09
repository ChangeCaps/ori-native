use ori_native::prelude::*;

fn main() {
    let mut data = Data {};

    App::new().run(&mut data, ui).unwrap();
}

struct Data {}

fn ui(_data: &Data) -> impl Effect<Data> + use<> {
    const PORTAL: ViewId = ViewId::new("portal");

    effects((
        window(
            row(portal(PORTAL))
                .background(Color::WHITE)
                .justify_content(Justify::Center)
                .align_items(Align::Center)
                .gap(6.0)
                .flex(1.0),
        )
        .title("Portal (examples/portal.rs)"),
        teleport(PORTAL, text("Hello,")),
        teleport(PORTAL, text("portal!")),
    ))
}
