use leptos::*;
use chrono::{DateTime, Local, TimeDelta};
use leptos_dom::helpers::IntervalHandle;
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
pub fn Countdown(rfc3339: &'static str) -> impl IntoView {
    let remaining = Remaining::from_rfc3339(rfc3339);

    if remaining.days() > 0 {
        view! {
            <ul id="countdown">
                <h2>{ remaining.days() }</h2>
                <p>"Hari lagi"</p>
            </ul>
        }
    } else if !remaining.is_timeout() {
        let rem_hms = create_rw_signal(remaining);

        use_interval(1000, move || {
            rem_hms.update(|rem| {
                rem.minus_one();
            });
        });

        view! {
            <ul id="countdown" class="flex gap-6">
                <li><h3>{ move || rem_hms.get().hours() }</h3><p>"Jam"</p></li>
                <li><h3>{ move || rem_hms.get().minutes() }</h3><p>"Menit"</p></li>
                <li><h3>{ move || rem_hms.get().seconds() }</h3><p>"Detik"</p></li>
            </ul>
        }
    } else {view! {<ul></ul>}}
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