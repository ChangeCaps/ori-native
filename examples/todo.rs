use ori_native::prelude::*;

fn main() {
    let mut data = Data { todos: Vec::new() };

    App::new().run(&mut data, ui);
}

struct Data {
    todos: Vec<Todo>,
}

struct Todo {
    name: String,
}

fn ui(data: &Data) -> impl Effect<Data> + use<> {
    window(
        column(
            column((input(), todos(data)))
                .width(300.0)
                .align_items(Align::Stretch)
                .border(1.0)
                .border_color(Color::BLACK),
        )
        .flex(1.0)
        .justify_contents(Justify::Center)
        .align_items(Align::Center),
    )
}

fn input() -> impl View<Data> + use<> {
    with(
        |_| String::new(),
        |name, _| {
            row(textinput()
                .text(name)
                .placeholder("What do you want to do?")
                .newline(Newline::None)
                .accept_tab(false)
                .on_change(|(name, _), text| *name = text)
                .on_submit(|(name, data), text| {
                    add_todo(data, text);
                    name.clear();
                })
                .flex(1.0))
            .padding(8.0)
            .border(1.0)
        },
    )
}

fn todos(data: &Data) -> impl View<Data> + use<> {
    let todos = data
        .todos
        .iter()
        .enumerate()
        .map(|(i, x)| todo(i, x))
        .rev()
        .collect::<Vec<_>>();

    vscroll(column(todos)).max_height(400.0).flex(1.0)
}

fn todo(index: usize, todo: &Todo) -> impl View<Data> + use<> {
    row(text(&todo.name).family("Fira Code").strikethrough(true))
        .padding(8.0)
        .border_top(1.0)
        .border_color(Color::BLACK)
}

fn add_todo(data: &mut Data, name: String) {
    data.todos.push(Todo { name })
}
