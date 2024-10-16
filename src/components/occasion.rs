use leptos::prelude::*;

#[component]
pub fn Occasion() -> impl IntoView {
    view! {
        <section class="font-['Trajan_Pro'] font-bold text-center text-colour2">
            <p class="tracking-widest">"THE WEDDING OF"</p>
            <h1 class="mt-2 text-5xl">"DIAN"</h1>
            <h2 class="text-3xl -tracking-widest">"\u{2015}\u{2015}\u{2015} & \u{2015}\u{2015}\u{2015}"</h2>
            <h1 class="mb-2 text-5xl">"RAFIYQ"</h1>
            <p class="tracking-widest">"27 . 10 . 2024"</p>
        </section>
    }
}