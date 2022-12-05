mod props;
mod text;
mod theme;

mod prelude {
    pub use crate::props::*;
    pub use crate::text::*;
    pub use crate::theme::*;
    pub use stylist::yew::styled_component;
    pub use stylist::yew::use_style;
    pub use stylist::Style;
    pub use yew::prelude::*;
}

// Provider for theming
// Button
//
//
// Password strength meter
// Steps left

use prelude::*;

// #[function_component]
#[function_component]
fn App() -> Html {
    html! {
        <ThemeProvider>
            <Text class={Some("custom".into())}>
                {"Snakes with human traits"}
            </Text>
            <Text>
                {"Boobs on the ground gameplay"}
            </Text>
            <Text custom={TextProps {label: "LOL".into()}} />
        </ThemeProvider>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
