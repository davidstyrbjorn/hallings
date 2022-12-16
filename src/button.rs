use crate::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ButtonProps {
    pub label: String,
    pub onClick: Callback,
}

pub fn Button(props: &CommonProps<ButtonProps>) -> Html {
    html! {
        <button>
            {props.custom.}
        </button>
    }
}
