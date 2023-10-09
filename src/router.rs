use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::pages::{chesscom::Chesscom, lichess::Lichess, home::Home};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/chesscom")]
    Chesscom,
    #[at("/lichess")]
    Lichess,
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => html!{<Home />},
        Route::Chesscom => html!{<Chesscom />},
        Route::Lichess => html!{<Lichess />}
    }
}