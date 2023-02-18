/*
 * Mosaic client source file (Web Client via Wasm, Reqwest)
 */
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World" }</h1>
    }
}

fn main(){
    yew::Renderer::<App>::new().render();
}
