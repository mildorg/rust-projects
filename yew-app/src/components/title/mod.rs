use stylist::yew::styled_component;
use yew::prelude::*;

use crate::utils::style::create_style;

const STYLE_FILE: &str = include_str!("style.css");

#[derive(Properties, PartialEq)]
pub struct TitleProps {
    pub title: String,
    pub sub_title: Option<String>,
}

#[styled_component(Title)]
pub fn title(props: &TitleProps) -> Html {
    let style = create_style(STYLE_FILE);

    let TitleProps { title, sub_title } = props;

    html! {
        <div class={style}>
            <h1 >{title}</h1>
            if let Some(sub)  = sub_title {
                <p class="sub_title">{sub}</p>
            }
        </div>
    }
}
