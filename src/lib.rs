use yew::prelude::*;
use router::{switch, Route};
use yew_router::prelude::*;

use std::fmt::{Display, self, Debug, Formatter};
use std::error::Error;

use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

mod components;
mod router;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <main>
                    <h1>{ "Chess Stats Lookup" }</h1>
                    <li><Link<Route> to={Route::Chesscom}>{ "Chess.com" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Lichess}>{ "Lichess" }</Link<Route>></li>
                </main>
                <Switch<Route>render={switch} />
            </BrowserRouter>
        </>
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.err, f)
    }
}

impl Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self {err: value}
    }
}

pub enum FetchState<T> {
    NotFetching,
    Fetching,
    Success(T),
    Failed(FetchError),
}