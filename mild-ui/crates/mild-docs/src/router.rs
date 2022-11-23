use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::button::ButtonDoc;

#[derive(Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/button")]
    Button,
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Button => html! {<ButtonDoc/>},
    }
}
