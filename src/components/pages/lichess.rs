use yew::prelude::*;
use crate::components::molecules::username_input::UsernameInput;
use reqwest::{Client, StatusCode};
use gloo::console::log;

use wasm_bindgen_futures;

#[function_component(Lichess)]
pub fn lichess() -> Html {
    let username_input_submit = Callback::from(|username| {
        wasm_bindgen_futures::spawn_local(async move {
            let client = Client::new();

            let username_response = client
                .post(format!("https://lichess.org/api/user/{}", username))
                .send()
                .await;

            match username_response {
                Ok(username_response) => {
                    if username_response.status() == StatusCode::CREATED {
                        log!("Got user data.");
                    }
                }
                Err(err) => {
                    println!("HTTP request Error: {:?}", err);
                }
            }
        });
    });
    html! {
        <main>
            <p>{ "Enter Lichess username: " }</p>
            <UsernameInput onsubmit={username_input_submit}/>
        </main>
    }
}