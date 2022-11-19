use yew::prelude::*;

mod helper;
use self::helper::get_styles;
pub use self::helper::{ButtonColor, ButtonKind, ButtonSize, ButtonType};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(ButtonColor::Default)]
    pub color: ButtonColor,
    #[prop_or_default]
    pub danger: bool,
    #[prop_or_default]
    pub disabled: bool,
    /// If pass href prop, display a link button
    #[prop_or_default]
    pub href: String,
    pub id: Option<String>,
    #[prop_or(ButtonKind::Default)]
    pub kind: ButtonKind,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(ButtonSize::Medium)]
    pub size: ButtonSize,
}

#[function_component(Button)]
pub fn button(
    Props {
        button_type,
        children,
        class,
        color,
        danger,
        disabled,
        href,
        id,
        kind,
        onclick,
        size,
    }: &Props,
) -> Html {
    let styles = get_styles(kind, size, href, *danger, *disabled, class);

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
            {child_list}
        </button>
    }
}
