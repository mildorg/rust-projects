mod helper;
pub use self::helper::{ButtonType, ButtonVariant};

use yew::prelude::*;

use self::helper::get_styles;
use crate::styles::{Color, Size};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(Color::Primary)]
    pub color: Color,
    #[prop_or_default]
    pub disabled: bool,
    /// If pass href prop, display a link button
    #[prop_or_default]
    pub href: String,
    pub id: Option<String>,
    #[prop_or(ButtonVariant::Outlined)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(Size::Medium)]
    pub size: Size,
}

#[function_component(Button)]
pub fn button(
    Props {
        button_type,
        children,
        class,
        color,
        disabled,
        href,
        id,
        variant,
        onclick,
        size,
    }: &Props,
) -> Html {
    let styles = get_styles(variant, color, size, href, *disabled, class);

    let child_list = children.iter().collect::<Html>();

    let handle_click = {
        let onclick = onclick.clone();
        Callback::from(move |event| onclick.emit(event))
    };

    if !href.is_empty() {
        return html! {
            <a
                id={id.clone()}
                class={styles}
                href={href.clone()}
                onclick={handle_click}
            >
                {child_list}
            </a>
        };
    }

    html! {
        <button
            id={id.clone()}
            class={styles}
            disabled={*disabled}
            onclick={handle_click}
            type={button_type.to_string()}
        >
            <div>{child_list}</div>
        </button>
    }
}
