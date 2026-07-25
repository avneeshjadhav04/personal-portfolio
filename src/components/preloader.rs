//! Preloader — port of `src/components/Preloader.tsx`.
//!
//! Three-phase state machine: enter → settle → exit → onComplete.
//! Also plays a per-character staggered reveal of "Avneesh" and a progress bar
//! from 0% to 100%.
//!
//! `onComplete` fires when the window has loaded AND the minimum visual delay
//! has elapsed, matching the TS behaviour exactly.

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;

use crate::motion::animate_presence::AnimatePresence;
use crate::motion::easing::EASE_STANDARD;
use crate::motion::variants::Variant;

#[derive(Clone, Copy, PartialEq)]
enum Phase {
    Enter,
    Settle,
    Exit,
}

/// `<Preloader on_complete=... />`.
#[component]
pub fn Preloader(on_complete: Callback<()>) -> impl IntoView {
    let phase = RwSignal::new(Phase::Enter);

    Effect::new(move || {
        let done = std::rc::Rc::new(std::cell::Cell::new(false));
        let on_complete_inner = on_complete;

        // Schedule phase transitions.
        let phase_settle = phase;
        let done_settle = done.clone();
        let settle = Closure::<dyn FnMut()>::new(move || {
            if !done_settle.get() {
                phase_settle.set(Phase::Settle);
            }
        });
        if let Some(w) = window() {
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                settle.as_ref().unchecked_ref(),
                800,
            );
        }
        std::mem::forget(settle);

        let phase_exit = phase;
        let done_exit = done.clone();
        let exit = Closure::<dyn FnMut()>::new(move || {
            if !done_exit.get() {
                phase_exit.set(Phase::Exit);
            }
        });
        if let Some(w) = window() {
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                exit.as_ref().unchecked_ref(),
                1600,
            );
        }
        std::mem::forget(exit);

        // Hard fallback: complete at 2.2s regardless.
        let finish = Closure::<dyn FnMut()>::new(move || {
            if !done.get() {
                done.set(true);
                on_complete_inner.run(());
            }
        });
        if let Some(w) = window() {
            let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                finish.as_ref().unchecked_ref(),
                2200,
            );
        }
        std::mem::forget(finish);

        // Fire onComplete when window.load fires, but no earlier than 1.6s
        // (matching the TS minDelay). Hard fallback above covers the case
        // where load already fired.
        let start = crate::utils::raf::now_seconds();
        let min_delay = 1.6_f64;
        let done_load = done.clone();
        let on_complete_load = on_complete;
        let on_load = Closure::<dyn FnMut()>::new(move || {
            let elapsed = crate::utils::raf::now_seconds() - start;
            let remaining = min_delay - elapsed;
            if remaining > 0.0 {
                let done_inner = done_load.clone();
                let cb = on_complete_load;
                let timer = Closure::<dyn FnMut()>::new(move || {
                    if !done_inner.get() {
                        done_inner.set(true);
                        cb.run(());
                    }
                });
                if let Some(w) = window() {
                    let _ = w.set_timeout_with_callback_and_timeout_and_arguments_0(
                        timer.as_ref().unchecked_ref(),
                        (remaining * 1000.0) as i32,
                    );
                }
                std::mem::forget(timer);
            } else if !done_load.get() {
                done_load.set(true);
                on_complete_load.run(());
            }
        });
        if let Some(w) = window() {
            if w.document().map(|d| d.ready_state()).as_deref() == Some("complete") {
                on_load.as_ref().unchecked_ref::<js_sys::Function>().call0(&w.into()).ok();
            } else {
                let opts = web_sys::AddEventListenerOptions::new();
                opts.set_once(true);
                let _ = w.add_event_listener_with_callback_and_add_event_listener_options(
                    "load",
                    on_load.as_ref().unchecked_ref(),
                    &opts,
                );
            }
        }
        std::mem::forget(on_load);
    });

    // Progress bar width: 0% in Enter, 100% in Settle/Exit.
    let progress_width = move || {
        let p = phase.get();
        if p == Phase::Settle || p == Phase::Exit { "100%" } else { "0%" }.to_string()
    };

    let chars = "Avneesh".chars().collect::<Vec<_>>();

    // Exit variant: fade out (opacity 1 -> 0).
    let exit_variant = Variant {
        opacity: Some(0.0),
        ..Variant::new()
    };

    let present = Signal::derive(move || phase.get() != Phase::Exit);

    view! {
        <AnimatePresence present=present exit=exit_variant duration=Some(0.5) ease=EASE_STANDARD>
            <div class="fixed inset-0 z-[9999] flex items-center justify-center bg-background">
                <div class="relative flex flex-col items-center gap-8">
                    <div class="flex items-center gap-1 overflow-hidden">
                        {chars.clone().into_iter().enumerate().map(|(i, c)| {
                            view! {
                                <span
                                    class="text-4xl md:text-6xl font-bold tracking-tight text-gradient"
                                    style=move || format!(
                                        "display: inline-block; opacity: 0; transform: translateY(80px) rotateX(-90deg); animation: preloaderChar 0.5s ease {}s forwards;",
                                        0.1 + (i as f64) * 0.05,
                                    )
                                >{c}</span>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                    <div class="w-48 h-[2px] bg-surface-light rounded-full overflow-hidden">
                        <div
                            class="h-full bg-accent"
                            style:width=progress_width()
                            style:transition="width 0.8s cubic-bezier(0.4, 0, 0.2, 1)"
                        />
                    </div>
                    <span
                        class="text-sm text-text-secondary font-mono-accent tracking-widest uppercase"
                        style="opacity: 0; animation: preloaderSub 0.4s ease 0.4s forwards;"
                    >
                        "Loading"
                    </span>
                </div>
            </div>
        </AnimatePresence>
    }
}