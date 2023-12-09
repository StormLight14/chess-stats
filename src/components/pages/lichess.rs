use crate::components::molecules::username_input::UsernameInput;
use gloo::console::log;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use yew::prelude::*;

use wasm_bindgen_futures;

#[derive(Debug, Deserialize)]
struct TimeControlStats {
    games: u32,
    rating: u32,
    rd: u32,
    prog: u32,
    prov: bool,
}

#[function_component(Lichess)]
pub fn lichess() -> Html {
    let username_input_submit = Callback::from(|username| {
        wasm_bindgen_futures::spawn_local(async move {
            let client = Client::new();

            let username_response: serde_json::Value = client
                .get(format!("https://lichess.org/api/user/{}", username))
                .header("Content-Type", "application/json")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            log!(format!("JSON Response: {:?}", username_response));

            if let (Some(user_id), Some(username), Some(perfs)) = (
                username_response.get("id"),
                username_response.get("username"),
                username_response.get("perfs"),
            ) {
                log!(format!("GOT USERNAME: {}", username));
                log!(format!("GOT ID: {}", user_id));

                if let (Some(bullet), Some(blitz), Some(rapid)) =
                    (perfs.get("bullet"), perfs.get("blitz"), perfs.get("rapid"))
                {
                    let bullet: TimeControlStats = serde_json::from_value(bullet.clone()).unwrap();
                    let blitz: TimeControlStats = serde_json::from_value(blitz.clone()).unwrap();
                    let rapid: TimeControlStats = serde_json::from_value(rapid.clone()).unwrap();

                    log!(format!("{:?}", bullet.rating));
                }
            } else {
                log!("User most likely does not exist.")
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

