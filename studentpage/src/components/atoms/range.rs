use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Properties for [Range]
#[derive(PartialEq, Properties)]
pub struct Props {
    pub label: String,
    pub id: String,
    #[prop_or_default]
    pub handle_oninput: Callback<String>,
    #[prop_or(true)]
    pub required: bool,
}

/// The [Range] component provides a styled range input.
/// The value of the range input is emitted to the [handle_oninput] callback as a string.
#[function_component(Range)]
pub fn range(props: &Props) -> Html {
    let handle_oninput = props.handle_oninput.clone();
    let oninput = Callback::from(move |event: InputEvent| {
        let value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_oninput.emit(value);
    });

    html! {
        <div class="col-12 col-xl-6 mb-2">
            <label for={ props.id.clone() } class="form-label">
                { Html::from_html_unchecked(AttrValue::from(format!("<div>{}</div>", props.label.clone()))) }
            </label>
            <input
                type="range"
                class="form-range"
                id={ props.id.clone() }
                min={"0"}
                max={"100"}
                step={"5"}
                oninput={ oninput }/>
        </div>
    }
}
