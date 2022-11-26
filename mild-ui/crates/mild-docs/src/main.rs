mod pages;
mod router;
mod utils;

use yew::prelude::*;
use yew_router::prelude::*;

use router::{switch, Route};

#[function_component]
fn App() -> Html {
    html! {
        <div class="doc-container">
            <BrowserRouter>
                <Switch<Route> render={switch}/>
            </BrowserRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
