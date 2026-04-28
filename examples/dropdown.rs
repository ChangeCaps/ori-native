use ori_native::prelude::*;

fn main() {
    let mut data = Data {};

    App::new().run(&mut data, ui).unwrap();
}

struct Data {}

const MODAL: ViewId = ViewId::new("modal");

fn ui(_data: &Data) -> impl Effect<Data> + use<> {
    window(
        column((
            dropdown(
                |_| {
                    column(text("Click to open dropdown"))
                        .padding(8.0)
                        .border(1.0, Color::BLACK.fade(0.2))
                        .corner(8.0)
                },
                column((text("stuff"), text("things")))
                    .padding(8.0)
                    .background(Color::hex("#f8f8f8"))
                    .border(1.0, Color::BLACK.fade(0.2))
                    .corner(8.0)
                    .shadow(4.0, 4.0, 8.0, Color::BLACK.fade(0.4)),
            ),
            portal(MODAL),
        ))
        .background(Color::WHITE)
        .justify_content(Justify::Center)
        .align_items(Align::Center)
        .flex(1.0),
    )
    .title("Dropdown (examples/dropdown.rs)")
}

fn dropdown<T, H>(
    header: impl Fn(PressState) -> H + 'static,
    contents: impl View<T>,
) -> impl View<T>
where
    H: View<T>,
{
    #[derive(Default)]
    struct State {
        x:       f32,
        y:       f32,
        height:  f32,
        is_open: bool,
    }

    with_default(move |state: &State, _| {
        let header = pressable(move |_, state| {
            map(
                header(state),
                |(_, data): &mut (_, T), map| map(data),
            )
        })
        .on_press(|(state, _): &mut (State, _)| state.is_open = !state.is_open);

        let body = state.is_open.then(|| {
            teleport(
                MODAL,
                column(map(
                    contents,
                    |(_, data): &mut (_, T), map| map(data),
                ))
                .position(Position::Absolute)
                .left(state.x)
                .top(state.y + state.height + 4.0),
            )
        });

        effect(
            measure(
                header,
                |(state, _): &mut (State, _), x, y, _, height| {
                    state.x = x;
                    state.y = y;
                    state.height = height;
                },
            ),
            body,
        )
    })
}
