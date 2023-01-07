use wasm_bindgen::*;
use yew::prelude::*;

pub fn unsupported(js_value: &JsValue) -> Html {
    html! {
        <div>
            {"Unsupported"}
            {js_value.as_string().unwrap_or_default()}
        </div>
    }
}