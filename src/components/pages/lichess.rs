use crate::components::molecules::username_input::UsernameInput;
use gloo::console::log;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::fmt;
use yew::platform::spawn_local;
use yew::prelude::*;

use wasm_bindgen_futures;

#[derive(Debug, Deserialize, Default)]
struct TimeControlStats {
    games: i32,
    rating: i32,
    rd: i32,
    prog: i32,
    prov: bool,
}

impl fmt::Display for TimeControlStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
            Games: {},\n
            Rating: {},\n
            RD: {},\n
            Progression: {},\n",
            self.games, self.rating, self.rd, self.prog
        )
    }
}

#[function_component(Lichess)]
pub fn lichess() -> Html {
    let bullet_state = use_state(|| TimeControlStats::default());
    let bullet_setter = bullet_state.setter();

    let blitz_state = use_state(|| TimeControlStats::default());
    let blitz_setter = blitz_state.setter();

    let rapid_state = use_state(|| TimeControlStats::default());
    let rapid_setter = rapid_state.setter();

    let username_input_submit = Callback::from(move |username| {
        let bullet_setter = bullet_setter.clone();
        let blitz_setter = blitz_setter.clone();
        let rapid_setter = rapid_setter.clone();

        spawn_local(async move {
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

            //log!(format!("JSON Response: {:?}", username_response));

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

                    log!(format!("{}", bullet));
                    log!(format!("{}", blitz));
                    log!(format!("{}", rapid));

                    bullet_setter.set(bullet);
                    blitz_setter.set(blitz);
                    rapid_setter.set(rapid);
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
            <h2>{"Bullet"}</h2>
            <p>{format!("{}", &*bullet_state)}</p>
            <h2>{"Blitz"}</h2>
            <p>{format!("{}", &*blitz_state)}</p>
            <h2>{"Rapid"}</h2>
            <p>{format!("{}", &*rapid_state)}</p>
        </main>
    }
}
