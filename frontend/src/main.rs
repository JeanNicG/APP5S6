mod api_call;

use api_call::{fetch_archive, set_device_status};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let archive_data = use_state(|| String::from("No data fetched yet. Click the button."));
    let is_on = use_state(|| false);

    let ad = archive_data.clone();
    let on_fetch = Callback::from(move |_| {
        let ad = ad.clone();
        spawn_local(async move { ad.set(fetch_archive().await) });
    });

    let power = is_on.clone();
    let on_toggle = Callback::from(move |_| {
        let power = power.clone();
        let new_state = !*power;
        power.set(new_state);
        spawn_local(async move { set_device_status(if new_state { "on" } else { "off" }).await });
    });

    html! {
        <main>
            <input type="checkbox" checked={*is_on} onchange={on_toggle} />
            <span>{ " Power: " }{ if *is_on { "ON" } else { "OFF" } }</span>
            
            <br /><br />
            
            <button onclick={on_fetch}>{ "GET Archive" }</button>
            <p>{ &*archive_data }</p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}