use crate::components::molecules::username_input::UsernameInput;
use gloo::console::log;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::fmt;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Debug, Deserialize, Default)]
struct TimeControlStats {
    rating: i32,
    rd: i32,
}

impl fmt::Display for TimeControlStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
            Rating: {},\n
            RD: {},\n",
            self.rating, self.rd
        )
    }
}

#[function_component(Chesscom)]
pub fn chesscom() -> Html {
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
                .get(format!(
                    "https://api.chess.com/pub/player/{}/stats",
                    username
                ))
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            //log!(format!("JSON Response: {:?}", username_response));

            if let (Some(chess_bullet), Some(chess_blitz), Some(chess_rapid)) = (
                username_response.get("chess_bullet").unwrap().get("last"),
                username_response.get("chess_blitz").unwrap().get("last"),
                username_response.get("chess_rapid").unwrap().get("last"),
            ) {
                let bullet: TimeControlStats =
                    serde_json::from_value(chess_bullet.clone()).unwrap();
                let blitz: TimeControlStats = serde_json::from_value(chess_blitz.clone()).unwrap();
                let rapid: TimeControlStats = serde_json::from_value(chess_rapid.clone()).unwrap();

                log!(format!("{}", bullet));
                log!(format!("{}", blitz));
                log!(format!("{}", rapid));

                bullet_setter.set(bullet);
                blitz_setter.set(blitz);
                rapid_setter.set(rapid);
            } else {
                log!("User most likely does not exist.")
            }
        });
    });
    html! {
        <main>
            <p>{ "Enter Chess.com username: " }</p>
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

