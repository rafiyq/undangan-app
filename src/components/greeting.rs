use leptos::prelude::*;

#[component]
pub fn Greeting() -> impl IntoView {
    // let guest = create_resource(move || async move { get_guest("budi".to_string()).await });

    

    // let guest = || async move {get_guest("budi".to_string()).await.unwrap()};
    // console_log(guest);
    view! {
        <section id="greeting">
            <p class="font-bold leading-none">"Kepada Yang Terhormat"</p>
            <p>"Bapak/Ibu/Saudara/i"</p>
            <div id="guest-box">
                <h6 class="font-normal">"title"</h6>
                <h6 class="font-['Merriweather'] font-extrabold">"fullname"</h6>
                <p>di</p>
                <h6 class="font-normal">"address"</h6>
                <p>"Pukul guest.session"</p>
            </div>
            <p class="text-[0.5em] overline ">"Mohon Maaf Bila Terdapat Kesalahan dalam Penulisan Nama dan Gelar "</p>
        </section>
    }
}