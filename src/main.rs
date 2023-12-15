use leptos::*;

mod todo_list;
use todo_list::*;

#[component]
fn App() -> impl IntoView {
    let todos = create_signal(vec![
        TodoItem {
            id: new_todo_id(),
            content: "watch movie".to_string(),
        },
        TodoItem {
            id: new_todo_id(),
            content: "read book".to_string(),
        },
    ]);
    view! {
        <div class="todo-app">
            <h1>"代办事项App"</h1>
            <TodoInput initial_todos={todos} />
            <TodoList todos={todos} />
        </div>
    }
}

fn main() {
    leptos::mount_to_body(App);
}
