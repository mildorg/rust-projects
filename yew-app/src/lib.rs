use stylist::yew::styled_component;
use yew::prelude::html;

mod module;
use module::pages::Login;
use module::utilities::style::create_style;

const STYLE_FILE: &str = include_str!("./main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let style = create_style(STYLE_FILE);

    html! {
        <div class={style}>
            <Login />
        </div>
    }
}

// fn render_list(list: &Vec<&str>) -> Html {
//     list.iter()
//         .map(|&item| html!(<li key={item} >{item}</li>))
//         .collect()
// }
