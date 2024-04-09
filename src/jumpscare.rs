use crate::prelude::*;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct JumpScareProps {}

#[function_component]
pub fn JumpScare(props: &CommonProps<JumpScareProps>) -> Html {
    let theme = use_context::<ThemeContext>();
    let styles = StyleUtil::create_jumpscare_style(props, theme.as_ref().unwrap());

    html! {
        <img
            style={styles}
            src="freddy.png"
            class={classes!("jumpscare")}
        />
    }
}
