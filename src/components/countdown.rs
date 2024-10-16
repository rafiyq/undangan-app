use leptos::prelude::*;
use chrono::{DateTime, Local, TimeDelta};
use std::time::Duration;

#[derive(Clone)]
struct Remaining {
    remaining: TimeDelta,
}

impl Remaining {
    fn from_rfc3339(datetime: &str) -> Self {
        let remaining = DateTime::parse_from_rfc3339(datetime).unwrap()
            .signed_duration_since(Local::now());
        Remaining { remaining }
    }
    fn days(&self) -> i64 {
        self.remaining.num_days()
    }
    fn hours(&self) -> String {
        format!("{:02}", self.remaining.num_hours() % 24)
    }
    fn minutes(&self) -> String {
        format!("{:02}", self.remaining.num_minutes() % 60)
    }
    fn seconds(&self) -> String {
        format!("{:02}", self.remaining.num_seconds() % 60)
    }
    pub fn is_timeout(&self) -> bool {
        self.remaining.num_seconds() < 0
    }
    fn minus_one(&mut self) {
        self.remaining -= TimeDelta::seconds(1)
    }
}

#[component]
pub fn Countdown() -> impl IntoView {
    let css = "py-2 font-['Dancing_Script'] text-center";
    let datetime = "2024-10-27T08:00:00+07:00";
    let remaining = Remaining::from_rfc3339(datetime);

    if remaining.days() > 0 {
        view! {
            <ul class={css}>
                <h1 class="text-4xl">{ remaining.days() }</h1>
                <p>"Hari lagi"</p>
            </ul>
        }.into_any()
    } else if !remaining.is_timeout() {
        let rem_hms = RwSignal::new(remaining);

        use_interval(1000, move || {
            rem_hms.update(|rem| {
                rem.minus_one();
            });
        });

        view! {
            <ul class={format!("{css} flex gap-6")}>
                <li class="w-8"><h1 class="text-left text-3xl">{ move || rem_hms.get().hours() }</h1><p>"Jam"</p></li>
                <li class="w-8"><h1 class="text-left text-3xl">{ move || rem_hms.get().minutes() }</h1><p>"Menit"</p></li>
                <li class="w-8"><h1 class="text-left text-3xl">{ move || rem_hms.get().seconds() }</h1><p>"Detik"</p></li>
            </ul>
        }.into_any()
    } else { view! {}.into_any() }
}

// source: https://github.com/leptos-rs/leptos/blob/main/examples/timer/src/lib.rs
/// Hook to wrap the underlying `setInterval` call and make it reactive w.r.t.
/// possible changes of the timer interval.
pub fn use_interval<T, F>(interval_millis: T, f: F)
where
    F: Fn() + Clone + 'static,
    T: Into<MaybeSignal<u64>> + 'static,
{
    let interval_millis = interval_millis.into();
    Effect::new(move |prev_handle: Option<IntervalHandle>| {
        // effects get their previous return value as an argument
        // each time the effect runs, it will return the interval handle
        // so if we have a previous one, we cancel it
        if let Some(prev_handle) = prev_handle {
            prev_handle.clear();
        };

        // here, we return the handle
        set_interval_with_handle(
            f.clone(),
            // this is the only reactive access, so this effect will only
            // re-run when the interval changes
            Duration::from_millis(interval_millis.get()),
        )
        .expect("could not create interval")
    });
}