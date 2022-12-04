use yew::prelude::*;

use super::helper::{get_styles, ButtonType, ButtonVariant};
use super::ripple_wrapper::RippleWrapper;
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
    pub href: AttrValue,
    pub id: Option<AttrValue>,
    #[prop_or(ButtonVariant::Outlined)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(Size::Medium)]
    pub size: Size,
}

#[function_component]
pub fn Button(
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
                id={id}
                class={styles}
                href={href}
                onclick={handle_click}
            >
                {child_list}
            </a>
        };
    }

    html! {
        <button
            id={id}
            class={styles}
            disabled={*disabled}
            onclick={handle_click}
            type={button_type.to_string()}
        >
            <div>{child_list}</div>
            <RippleWrapper />
        </button>
    }
}
