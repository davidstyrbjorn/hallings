mod password_strength_input;
mod props;
mod style_util;
mod text;
mod theme;
mod button;

mod prelude {
    pub use crate::password_strength_input::*;
    pub use crate::props::*;
    pub use crate::style_util::*;
    pub use crate::text::*;
    pub use crate::theme::*;
    pub use stylist::css;
    pub use stylist::style;
    pub use stylist::yew::styled_component;
    pub use stylist::yew::use_style;
    pub use stylist::Style;
    pub use yew::prelude::*;
    pub use yew::Properties;
    pub use crate::button::*;
    pub use web_sys::console;
    pub use gloo_events::*;
}

use prelude::*;
use yew::context;
use web_sys::console;

pub fn calculate_strength_level(value: String) -> StrengthLevel {
    if value.contains("secure") {
        return StrengthLevel::HIGH;
    }
    StrengthLevel::LOW
}

pub fn on_level_change(strength_level: StrengthLevel) {
    match strength_level {
        StrengthLevel::LOW => console::log_1(&"LOW".into()),
        StrengthLevel::MEDIUM => console::log_1(&"MEDIUM".into()),
        StrengthLevel::HIGH => console::log_1(&"HIGH".into()),
    }
}

fn strength_level_to_text_and_color(value: StrengthLevel) -> (String, String) {
    match value {
        StrengthLevel::LOW => ("Password not strong enough".into(), "red".into()),
        StrengthLevel::MEDIUM => ("Password weak but passable".into(), "blue".into()),
        StrengthLevel::HIGH => ("Password strong".into(), "green".into()),
    }
}

#[function_component]
fn App() -> Html {
    fn click(s: yew::MouseEvent) {
        console::log_1(&"asss".into())
    }



    let cb = Callback::from(click);
        
    html! {
        <ThemeProvider >
            <Text size={"40px"}>{"hej"}</Text>
            <Button custom={ButtonProps{label: "hello".into(), onclick: cb}}></Button>
            // <Text color="red" size="64px">{"Hallingos"}</Text>
            <PasswordStrengthInput custom={PasswordStrengthInputProps {
                // calculate_strength_level: Some(calculate_strength_level),
                calculate_strength_level: None,
                strength_level_to_text_and_color: Some(strength_level_to_text_and_color),
                // strength_level_to_text_and_color: None,
                strength_callback: on_level_change
            }} />
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
