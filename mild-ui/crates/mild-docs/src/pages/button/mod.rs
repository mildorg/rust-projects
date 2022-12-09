use gloo_console::log;
use yew::prelude::*;

use mild_core::{
    button::{Button, ButtonVariant},
    styles::{Color, Size},
};

const COLORS: [Color; 6] = [
    Color::Primary,
    Color::Secondary,
    Color::Success,
    Color::Info,
    Color::Warning,
    Color::Error,
];

const SIZES: [Size; 3] = [Size::Small, Size::Medium, Size::Large];

const VARIANTS: [ButtonVariant; 4] = [
    ButtonVariant::Circle,
    ButtonVariant::Contained,
    ButtonVariant::Outlined,
    ButtonVariant::Text,
];

fn render_variants_color_buttons(size: Option<Size>) -> Vec<Html> {
    let size = size.unwrap_or(Size::Medium);

    let get_buttons = |variant: &ButtonVariant| {
        COLORS
            .iter()
            .map(|color| {
                html! {
                    <Button
                        key={format!("{variant}-{color}")}
                        variant={variant.clone()}
                        color={color.clone()}
                        size={size.clone()}
                    >
                        {
                            if *variant == ButtonVariant::Circle { String::from("√") }
                            else {color.to_string().to_uppercase()}
                        }
                    </Button>
                }
            })
            .collect::<Vec<Html>>()
    };

    let buttons = VARIANTS
        .iter()
        .map(|variant| {
            html! {<div key={variant.to_string()} class="mb-4">{get_buttons(variant)}</div>}
        })
        .collect();

    buttons
}

fn render_link_buttons() -> Vec<Html> {
    COLORS
        .into_iter()
        .map(|color| {
            html! {
                <Button
                    href="#link-buttons"
                    id={color.to_string()}
                    key={color.to_string()}
                    color={color.clone()}
                >
                     {color.to_string().to_uppercase()}
                </Button>
            }
        })
        .collect()
}

fn render_size_buttons() -> Vec<Html> {
    let get_buttons = |variant: &ButtonVariant| {
        SIZES
            .into_iter()
            .map(move |size| {
                html! {
                    <Button key={size.to_string()} variant={variant.clone()} size={size.clone()} >
                        {
                            if *variant == ButtonVariant::Circle { String::from("√") }
                            else {size.to_string().to_uppercase()}
                        }
                    </Button>
                }
            })
            .collect::<Vec<Html>>()
    };

    let buttons = VARIANTS
        .iter()
        .map(|variant| {
            html! { <div key={variant.to_string()} class="mb-4">{get_buttons(variant)}</div>}
        })
        .collect();

    let link_buttons = SIZES
        .iter()
        .map(|size| {
            html! {
                <Button href="#" key={size.to_string()} size={size.clone()} >
                    {size.to_string().to_uppercase()}
                </Button>
            }
        })
        .collect::<Vec<Html>>();

    [buttons, link_buttons].concat()
}

#[function_component]
pub(crate) fn ButtonDoc() -> Html {
    let handle_click = Callback::from(move |e: MouseEvent| {
        let info = format!("Action: {}", e.type_());
        log!(info);
    });

    html! {
        <div class="button-doc">
            <div class="section">
                <h2>{"Variants & Colors"}</h2>
                {render_variants_color_buttons(None)}
            </div>

            <div class="section">
                <h2 id="link-button">{"Link Buttons"}</h2>
                {render_link_buttons()}
            </div>

            <div class="section">
                <h2>{"Sizes"}</h2>
                {render_size_buttons()}
            </div>

            <div class="section">
                <h2>{"Event"}</h2>
                <p>{"Open the browser console, click the button and see what happened."}</p>
                <Button variant={ButtonVariant::Contained} onclick={handle_click}>
                    {Color::Primary.to_string().to_uppercase()}
                </Button>
            </div>
        </div>
    }
}
