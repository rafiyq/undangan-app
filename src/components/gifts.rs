use leptos::prelude::*;

#[component]
pub fn Gifts() -> impl IntoView {
    view! {
        <article>
            <h1>"Tanda Kasih"</h1>
            <p>"Begi yang ingin memberikan tanda kasih, dapah mengirimkan melalui fitur dibawah ini:"</p>
            <a>"Bank Transfer"</a>
            <a>"Kirim Hadiah"</a>
            <a>"Konfirmasi"</a>
            <a>"Save the Date"</a>
        </article>
    }
}