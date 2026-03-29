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
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
            </header>
        </section>
    }
}
