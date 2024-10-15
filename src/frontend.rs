use leptos::*;

#[component]
pub fn App(cx: Scope) -> Element {
    view! { cx,
        <h1>"Hello, Leptos + Actix + SurrealDB!"</h1>
        <button>"Click me!"</button>
    }
}
