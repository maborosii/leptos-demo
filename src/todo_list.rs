use leptos::*;
use rand::Rng;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TodoItem {
    pub id: u32,
    pub content: String,
}

#[component]
pub(crate) fn TodoInput(
    initial_todos: (ReadSignal<Vec<TodoItem>>, WriteSignal<Vec<TodoItem>>),
) -> impl IntoView {
    let (_, set_new_todo) = initial_todos;
    let (default_value, set_default_value) = create_signal("");
    view! {
        <input type="text" class= "new-todo" autofocus=true placeholder="Add todo"
        on:keydown= move |event| {
            if event.key() == "Enter" && !event_target_value(&event).is_empty() {
                let input_value = event_target_value(&event);
                let new_todo_item = TodoItem { id: new_todo_id(), content: input_value.clone() };
                set_new_todo.try_update(|todo| todo.push(new_todo_item));
                set_default_value.set("");
            }}
        prop:value=default_value
        />
    }
}

#[component]
pub(crate) fn TodoList(
    todos: (ReadSignal<Vec<TodoItem>>, WriteSignal<Vec<TodoItem>>),
) -> impl IntoView {
    let (todo_list_state, set_todo_list_state) = todos;
    let my_todos = move || {
        todo_list_state
            .get()
            .iter()
            .map(|item| (item.id, item.clone()))
            .collect::<Vec<_>>()
    };
    view! {
        <ul class="todo-list">
        <For each=my_todos key=|todo_key| todo_key.0
            children=move |item| {
                view! {
                    <li class="new-todo" > {item.1.content}
                        <button
                        class="remove"
                            on:click=move |_| {
                                set_todo_list_state.update(|todos| {
                                    todos.retain(|todo| &todo.id != &item.1.id)
                                });
                            }
                        >
                        </button>
                    </li>
                }
            }
        />
        </ul>
    }
}

pub(crate) fn new_todo_id() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}
