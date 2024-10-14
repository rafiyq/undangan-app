use leptos::{either::Either, prelude::*};
use leptos_router::hooks::use_params_map;
use send_wrapper::SendWrapper;
use crate::api::get_guest;

#[component]
pub fn Greeting() -> impl IntoView {
    let params = use_params_map();
    let guest = Resource::new(
        move || params.read().get("username").unwrap_or_default(),
        move |username| {
            SendWrapper::new(async move {
                if username.is_empty() {
                    None
                } else {
                    get_guest(username.to_string()).await.unwrap()
                }
            })
        },
    );

    view! {
        <section id="greeting">
        <Suspense fallback=|| {
            view! { "Loading..." }
        }>
            {move || Suspend::new(async move {
                match guest.await.clone() {
                    None => Either::Left(view! {}),
                    Some(guest) => {
                        Either::Right(
                            view! {
                                <p class="font-bold leading-none">"Kepada Yang Terhormat"</p>
                                <p>"Bapak/Ibu/Saudara/i"</p>
                                <div id="guest-box">
                                    <h6 class="font-normal">{guest.title}</h6>
                                    <h6 class="font-['Merriweather'] font-extrabold">{guest.fullname}</h6>
                                    <p>di</p>
                                    <h6 class="font-normal">"address"</h6>
                                    <p>"Pukul " {guest.session}</p>
                                </div>
                                <p class="text-[0.5em] overline ">"Mohon Maaf Bila Terdapat Kesalahan dalam Penulisan Nama dan Gelar "</p>
                            }
                        )
                    }
                }
            })}
        </Suspense>
        </section>
    }
}