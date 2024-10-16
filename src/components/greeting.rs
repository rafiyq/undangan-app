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
        <section class="w-full flex flex-col items-center font-['Gowun_Batang'] text-center">
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
                                <div class="my-1 p-2 min-w-[4/6] max-w-full rounded-lg border border-colour3 leading-tight">
                                    <h1 class="font-normal text-lg">{guest.title}</h1>
                                    <h1 class="font-['Merriweather'] text-lg font-extrabold">{guest.fullname}</h1>
                                    <p>di</p>
                                    <h1 class="font-normal text-lg">"address"</h1>
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