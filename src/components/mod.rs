use leptos::prelude::*;
use leptos_meta::Title;

use event::{MainEvent, NextEvent, PastEvent};
use gifts::Gifts;
use introduction::Introduction;
use beranda::Beranda;
use countdown::Countdown;
use greeting::Greeting;
use occasion::Occasion;
use prologue::Prologue;
use quote::Quote;

mod countdown;
mod greeting;
mod occasion;
mod beranda;
mod quote;
mod prologue;
mod gifts;
mod countdown_;
mod introduction;
mod event;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <Title text="Beranda - Dian Rafiyq"/>
        <main class="flex min-h-dvh flex-col justify-center sm:py-12 bg-colour1 text-colour3 font-['Inter']">
            <article class="px-6 pt-10 pb-8 flex flex-col gap-6 justify-center sm:mx-auto sm:aspect-[3/4] sm:rounded-lg sm:px-10 sm:shadow-nm-flat">
                <Occasion />
                <Countdown />
                <Greeting />
                <a class="btn-primary" href="/undangan">Buka Undangan</a>
            </article>
        </main>
    }
}

#[component]
pub fn UndanganPage() -> impl IntoView {
    view! {
        <Title text="Undangan - Dian Rafiyq"/>
        <main class="flex flex-col justify-center bg-gray-50">
            <video autoplay loop controls controlslist="play timeline volume" class="h-dvh aspect-[9/16] mx-auto shadow-nm-flat"> 
                <source src="https://r2.dianrafiyq.site/Dian%26Rafiyq%20Live_20241015_183829_0001.mp4" type="video/mp4"/>
            </video>
        </main>
    }
}

#[component]
pub fn TamuPage() -> impl IntoView {
    view! {
        <Title text="Dian & Rafiyq Wedding"/>
        <main>
            <Beranda />
            <Quote />
            <Prologue />
            <Introduction />
            <PastEvent />
            <MainEvent />
            <NextEvent />
            <countdown_::Countdown />
            <Gifts />
        </main>
    }
}