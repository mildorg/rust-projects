mod pages;
mod router;

use yew::prelude::*;
use yew::start_app;
use yew_router::prelude::*;

use router::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}

fn main() {
    start_app::<App>();
}
