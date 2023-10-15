use yew::prelude::*;
use crate::components::molecules::username_input::UsernameInput;
use crate::{FetchError, FetchState};

#[function_component(Chesscom)]
pub fn chesscom() -> Html {
    let username_input_submit = Callback::from(|username| {

    });
    html! {
        <main>
            <p>{"Enter Chess.com username: "}</p>
            <UsernameInput onsubmit={username_input_submit}/>
        </main>
    }
}