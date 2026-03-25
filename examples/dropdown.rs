use ori_native::prelude::*;

fn main() {
    let mut data = Data {};

    App::new().run(&mut data, ui).unwrap();
}

struct Data {}

fn ui(_data: &Data) -> impl Effect<Data> + use<> {
    window(
        column(dropdown(
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
                .shadow_color(Color::BLACK.fade(0.4))
                .shadow_radius(8.0)
                .shadow_offset(4.0, 4.0),
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
    struct State {
        x:       f32,
        y:       f32,
        height:  f32,
        is_open: bool,
    }

    with(
        |_| State {
            x:       0.0,
            y:       0.0,
            height:  0.0,
            is_open: false,
        },
        move |state, _| {
            let header = pressable(move |_, state| {
                map(
                    header(state),
                    |(_, data): &mut (_, T), map| map(data),
                )
            })
            .on_press(|(state, _): &mut (State, _)| state.is_open = !state.is_open);

            let body = state.is_open.then(|| {
                modal(
                    column(map(
                        contents,
                        |(_, data): &mut (_, T), map| map(data),
                    ))
                    .left(state.x)
                    .top(state.y + state.height + 4.0),
                )
            });

            effect(
                on_measure(
                    header,
                    |(state, _): &mut (State, _), x, y, _, height| {
                        state.x = x;
                        state.y = y;
                        state.height = height;
                    },
                ),
                body,
            )
        },
    )
}
