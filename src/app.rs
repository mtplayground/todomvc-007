use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/todomvc-007.css"/>
        <Title text="TodoMVC"/>
        <Router>
            <Routes>
                <Route path="/" view=TodoPage/>
                <Route path="/:filter" view=TodoPage/>
            </Routes>
        </Router>
    }
}

#[component]
pub fn TodoPage() -> impl IntoView {
    view! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
                <input
                    class="new-todo"
                    placeholder="What needs to be done?"
                    autofocus=true
                />
            </header>
            <section class="main">
                <ul class="todo-list">
                </ul>
            </section>
            <footer class="footer">
                <span class="todo-count">
                    <strong>"0"</strong>
                    " items left"
                </span>
            </footer>
        </section>
        <footer class="info">
            <p>"Double-click to edit a todo"</p>
        </footer>
    }
}
