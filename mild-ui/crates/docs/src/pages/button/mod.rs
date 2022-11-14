use yew::prelude::*;

use mild_core::button::{Button, ButtonKind, ButtonSize};

fn render_kind() -> Vec<Html> {
    let kinds = vec![
        ButtonKind::Default,
        ButtonKind::Primary,
        ButtonKind::Secondary,
        ButtonKind::Info,
        ButtonKind::Success,
        ButtonKind::Warning,
        ButtonKind::Danger,
    ];

    let link_button = html! { <Button kind={ButtonKind::Link} href="#">{ButtonKind::Link}</Button>};
    let mut buttons: Vec<Html> =
        kinds.into_iter().map(|kind| html! {<Button kind={kind.clone()}>{kind}</Button>}).collect();

    buttons.push(link_button);

    buttons
}

#[function_component(ButtonDoc)]
pub(crate) fn button_doc() -> Html {
    html! {
        <div class="button-doc">
            <div class="section">
                <h2>{"Button kind"}</h2>
                {render_kind()}
            </div>

            <div class="section">
                <h2>{"Sizes"}</h2>
                <Button size={ButtonSize::Sm}>{"small"}</Button>
                <Button kind={ButtonKind::Primary}>{"default"}</Button>
                <Button size={ButtonSize::Lg} kind={ButtonKind::Danger}>{"large"}</Button>
                <Button size={ButtonSize::Lg} kind={ButtonKind::Link} href="#">{"large"}</Button>
            </div>

            <div class="section">
                <h2>{"Disabled state"}</h2>
                <Button disabled={true}>{"default"}</Button>
                <Button kind={ButtonKind::Primary} disabled={true}>{"primary"}</Button>
                <Button kind={ButtonKind::Danger} disabled={true}>{"danger"}</Button>
                <Button kind={ButtonKind::Link} href="#" disabled={true}>{"link"}</Button>
            </div>
        </div>
    }
}
