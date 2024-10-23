use leptos::{either::Either, prelude::*};
use leptos_router::hooks::use_params_map;
use send_wrapper::SendWrapper;
use crate::{api::get_guest, components::HomePage, };

#[component]
pub fn Beranda() -> impl IntoView {
    
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
        <article class="w-full flex flex-col items-center font-['Gowun_Batang'] text-center">
            <section class="font-bold text-center text-colour2">
                <h1 class="mt-2 text-5xl">"Dian &"</h1>
                <h1 class="mb-2 text-5xl">"Rafiyq"</h1>
            </section>
            <Suspense fallback=|| {
                view! { "Loading..." }
            }>
                {move || Suspend::new(async move {
                    match guest.await.clone() {
                        None => Either::Left(HomePage()),
                        Some(guest) => {
                            Either::Right(
                                view! {
                                    <p>"Kepada Bapak/Ibu/Saudara/i"</p>
                                    <h1 class="font-['Merriweather'] text-lg font-extrabold">{guest.fullname}</h1>
                                    <a class="btn" href="/undangan">Buka Undangan</a>
                                    <p class="text-[0.5em]">"Mohon Maaf Bila Terdapat Kesalahan dalam Penulisan Nama dan Gelar "</p>
                                }
                            )
                        }
                    }
                })}
            </Suspense>
        </article>
    }
}