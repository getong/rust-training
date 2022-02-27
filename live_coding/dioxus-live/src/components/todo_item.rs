use crate::Todos;
use dioxus::prelude::*;
use tracing::info;

#[derive(Props)]
pub struct TodoItemProps<'a> {
    pub id: u32,
    pub set_todos: &'a UseState<Todos>,
}

pub fn todo_item<'a>(cx: Scope<'a, TodoItemProps<'a>>) -> Element {
    let (is_editing, set_is_editing) = use_state(&cx, || false);
    let id = cx.props.id;
    let set_todos = cx.props.set_todos;
    let todos = set_todos.get();
    let todo = &todos[&id];
    let completed = if todo.completed { "completed" } else { "" };
    let editing = if *is_editing { "editing" } else { "" };

    rsx! { cx, li {
        class: "{completed} {editing}",
        div { class: "view",
            input {
                class: "toggle",
                r#type: "checkbox",
                id: "todo-{todo.id}",
                checked: "{todo.completed}",
                onclick: move |e| {
                    info!("todo item clicked: {e:?}");
                    let mut todos = set_todos.make_mut();
                    todos.get_mut(&id).map(|todo| {
                        todo.completed = !todo.completed;
                    });
                    todos.save();
                }
            },
            label {
                // r#for: "todo-{todo.id}",
                onclick: move |e| {
                    info!("clicked label: {e:?}");
                    set_is_editing(true);
                },
                "{todo.title}"
            }
        }
        is_editing.then(|| rsx! {
            input {
                class: "edit",
                value: "{todo.title}",
                oninput: move |e| {
                    info!("todo item edited: {e:?}");
                    let mut todos = set_todos.make_mut();
                    todos.get_mut(&id).map(|todo| {
                        todo.title = e.value.clone();
                    });
                },
                autofocus: "true",
                onkeydown: move |e| {
                    match e.key.as_str() {
                        "Enter" | "Escape" | "Tab" => {
                            set_is_editing(false);
                            let todos = set_todos.get();
                            todos.save();
                        },

                        _ => {}
                    }
                },
            }
        })
    }}
}
