mod pages;
mod router;
mod utils;

use yew::prelude::*;
use yew::start_app;
use yew_router::prelude::*;

use router::{switch, Route};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="doc-container">
            <BrowserRouter>
                <Switch<Route> render={Switch::render(switch)}/>
            </BrowserRouter>
        </div>
    }
}

fn main() {
    start_app::<App>();
}
