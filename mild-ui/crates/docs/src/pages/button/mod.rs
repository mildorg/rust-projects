use yew::prelude::*;

use mild_core::{
    button::{Button, ButtonVariant},
    styles::{Color, Size},
};

use crate::utils::string::to_first_upper;

fn get_colors() -> Vec<Color> {
    vec![
        Color::Primary,
        Color::Secondary,
        Color::Success,
        Color::Info,
        Color::Warning,
        Color::Error,
    ]
}

fn render_variants_color_buttons() -> Vec<Html> {
    let variants = vec![
        ButtonVariant::Circle,
        ButtonVariant::Contained,
        ButtonVariant::Outlined,
        ButtonVariant::Text,
    ];

    let get_buttons = |variant: &ButtonVariant| {
        get_colors()
            .iter()
            .map(|color| {
                html! {
                    <Button
                        key={format!("{variant}-{color}")}
                        variant={variant.clone()}
                        color={color.clone()}
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

    let buttons = variants
        .iter()
        .map(|variant| {
            html! {<div key={variant.to_string()} class="mb-4">{get_buttons(variant)}</div>}
        })
        .collect();

    buttons
}

fn render_link_buttons() -> Vec<Html> {
    get_colors()
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

fn render_button_sizes() -> Vec<Html> {
    let sizes = vec![Size::Small, Size::Medium, Size::Large];

    let buttons=  sizes.iter().map(|size| {
        html! {
            <Button key={size.to_string()} size={size.clone()}>{to_first_upper(&size.to_string())}</Button>
        }
    }).collect::<Vec<Html>>();

    let link_buttons =sizes.iter().map(|size| {
        html! {
            <Button href="#" key={size.to_string()} size={size.clone()} >{to_first_upper(&size.to_string())}</Button>
        }
    }).collect::<Vec<Html>>();

    [buttons, link_buttons].concat()
}

#[function_component(ButtonDoc)]
pub(crate) fn button_doc() -> Html {
    let danger = "Danger";
    let primary = "Primary";
    let link = "Link";

    html! {
        <div class="button-doc">
            <div class="section">
                <h2>{"Variants & Colors"}</h2>
                {render_variants_color_buttons()}
            </div>

            <div class="section">
                <h2 id="link-button">{"Link Buttons"}</h2>
                {render_link_buttons()}
            </div>

            <div class="section">
                <h2>{"Sizes"}</h2>
                {render_button_sizes()}
            </div>

            <div class="section">
                <h2>{"Danger state"}</h2>
                <Button >{danger}</Button>
                <Button href="#" >{link}</Button>
            </div>

            <div class="section">
                <h2>{"Event"}</h2>
                <Button variant={ButtonVariant::Contained}>{primary}</Button>
            </div>
        </div>
    }
}
