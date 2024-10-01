use leptos::*;
use chrono::{DateTime, Local};
use leptos_dom::helpers::IntervalHandle;
use std::time::Duration;

#[component]
pub fn Countdown(rfc3339: &'static str) -> impl IntoView {
    let remaining = DateTime::parse_from_rfc3339(rfc3339).unwrap()
        .signed_duration_since(Local::now());
    let rem_days = remaining.num_days();

    if rem_days > 0 {
        view! {
            <ul id="countdown">
                <h2>{ rem_days }</h2>
                <p>"Hari lagi"</p>
            </ul>
        }
    } else {
        let rem_hms = create_rw_signal(remaining.num_seconds() as i32);

        use_interval(1000, move || {
            rem_hms.update(|r| *r -= 1);
        });

        view! {
            <ul id="countdown" class="flex gap-6">
                <li><h3>{ rem_hms.get() / 3600 }</h3><p>"Menit"</p></li>
                <li><h3>{ (rem_hms.get() % 3600) / 60 }</h3><p>"Jam"</p></li>
                <li><h3>{ rem_hms.get() % 60 }</h3><p>"Detik"</p></li>
            </ul>
        }
    }
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