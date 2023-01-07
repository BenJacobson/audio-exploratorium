#![allow(unused_must_use)]

use log::info;
use wasm_bindgen::*;
use web_sys::*;
use yew::prelude::*;

fn play_sound() -> Result<(), JsValue> {
    let audio_context = AudioContext::new()?;
    let oscillator_node = audio_context.create_oscillator()?;
    oscillator_node.set_type(OscillatorType::Sine);
    oscillator_node.connect_with_audio_node(&audio_context.destination());
    oscillator_node.start();

    Ok(())
}

#[function_component]
fn App() -> Html {
    info!("running app");
    let started = use_state(|| false);
    let onclick = move |_| {
        if !*started {
            play_sound();
            started.set(true);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "Start" }</button>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
