use leptos::*;
use leptos_meta::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/example.css" />
        <Title text="Cargo Leptos" />
        <main class="my-0 mx-auto max-w-3xl text-center">
            <h1 class="p-6 text-4xl">"Hi from your Leptos WASM!"</h1>
            <p class="px-10 pb-10 text-left">"This setup includes Tailwind and SASS"</p>
        </main>
    }
}
