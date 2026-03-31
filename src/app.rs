use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    view! {
        <main>
            <h1>"Leptos Basic Template"</h1>
            <p>"Welcome to the Leptos WASM frontend template."</p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "Count: " {count}
            </button>
        </main>
    }
}
