use yew::prelude::*;

/// Variants of the [MsgBox]
#[allow(dead_code)] // TODO: Remove if some variants never get used.
#[derive(PartialEq, Clone)]
pub enum MsgBoxVariant {
    Success,
    Info,
    Warning,
    Danger,
}

/// Properties for [MsgBox]
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The variant of the message box.
    pub variant: MsgBoxVariant,
    /// The title of the message box.
    pub title: AttrValue,
    /// The text of the message box. Defaults to an empty string.
    #[prop_or_default]
    pub text: AttrValue,
    #[prop_or_default]
    pub children: Children,
}

/// The [MsgBox] component provides a styled message box with four variants.
/// This uses a bootstrap card.
/// See <https://getbootstrap.com/docs/5.3/components/card/>
#[function_component(MsgBox)]
pub fn msg(props: &Props) -> Html {
    let get_variant = |variant: &MsgBoxVariant| match variant {
        MsgBoxVariant::Success => "text-success border-success",
        MsgBoxVariant::Info => "text-info border-info",
        MsgBoxVariant::Warning => "text-warning border-warning",
        MsgBoxVariant::Danger => "text-danger border-danger",
    };

    html! {
        <div class={format!("card shadow {}", get_variant(&props.variant))}>
            <div class="card-body">
                <h5 class="card-title"> { props.title.clone() } </h5>
                <p class="card-text"> { props.text.clone() } </p>
                { for props.children.iter() }
            </div>
        </div>
    }
}
