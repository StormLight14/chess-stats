use yew::prelude::*;
use router::{switch, Route};
use yew_router::prelude::*;

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

