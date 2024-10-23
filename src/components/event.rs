use leptos::prelude::*;

#[component]
pub fn PastEvent() -> impl IntoView {
    view! {
        <article>
            <h1>"Akad Nikah"</h1>
            <p>"Telah dilaksanakan pada:"</p>
            <h1>"19 Maret 2024"</h1>
            <p>"Di kediaman mempelai wanita"</p>
            <p>"Jl. Jendral Sudirman No 136"</p>
            <p>"Blora - Jawa Tengah"</p>
        </article>
    }
}

#[component]
pub fn MainEvent() -> impl IntoView {
    view! {
        <article>
            <h1>"Resepsi"</h1>
            <p>"Resepsi Pernikahan akan dilaksanakan pada:"</p>
            <h1>"Minggu, 27 Oktober 2024"</h1>
            <h1>"Pukul: 11.00 - 12.00 WIB"</h1>
            <p>"Bertempat di:"</p>
            <p>"Di kediaman mempelai wanita"</p>
            <p>"Jl. Jendral Sudirman No 136"</p>
            <p>"Blora - Jawa Tengah"</p>
            <a>"Google Map"</a>
        </article>
    }
}

#[component]
pub fn NextEvent() -> impl IntoView {
    view! {
        <article>
            <h1>"Ngunduh Mantu"</h1>
            <p>"Akan dilaksanakan pada:"</p>
            <h1>"03 November 2024"</h1>
            <p>"Pukul:"</p>
            <p>"12.30 - 14.30 WIB"</p>
            <p>"Bertempat di:"</p>
            <h1>"Graha Djiwo Wasibagna"</h1>
            <p>"Jl. Kp. Baru, Dusun I, Bonyokan, Jatinom"</p>
            <p>"Blora - Jawa Tengah"</p>
            <a>"Google Map"</a>
        </article>
    }
}