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
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
