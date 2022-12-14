use crate::prelude::{CommonPropsNoOption, ThemeContext, ToSVGHex};
use std::fmt;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Step {
    pub label: String,
}

#[derive(PartialEq, Properties, Clone)]
pub struct StepsLeftProps {
    pub width: usize,
    pub height: usize,
    pub current_step: usize,
    pub steps: Vec<Step>,
}

impl fmt::Debug for StepsLeftProps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StepsLeftProps")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("current_step", &self.current_step)
            .finish()
    }
}

#[function_component]
pub fn StepsLeft(props: &CommonPropsNoOption<StepsLeftProps>) -> Html {
    let size: (usize, usize) = (props.custom.width, props.custom.height);
    let theme = use_context::<ThemeContext>();

    // Calculate step size as width of canvas / numbe of steps
    let step_size = props.custom.width / props.custom.steps.len();
    let center = props.custom.height / 2;

    let steps = props.custom.steps.clone();
    // Generate the steps i.e circles
    let steps_display: Html = steps
        .into_iter()
        .enumerate()
        .map(|(i, step)| {
            let x = (i * step_size) + (step_size / 2);
            let mut color = theme.as_ref().unwrap().foreground.convert_to_svg();
            if props.custom.current_step > i {
                color = theme.as_ref().unwrap().main.convert_to_svg();
            }
            html! {
                <>
                    <text
                        text-anchor="middle"
                        x={x.to_string()}
                        y={(center-30).to_string()}
                    >
                        {step.label}
                    </text>
                    <circle
                        class={classes!("hallings-circle")}
                        fill={color}
                        cx={x.to_string()}
                        cy={center.to_string()}
                        style="fill 1s;"
                        r="15"
                    />
                </>
            }
        })
        .collect();

    let steps = props.custom.steps.clone();

    let lines_display = steps
        .into_iter()
        .enumerate()
        .map(|(i, _step)| {
            let x = (i * step_size) + (step_size / 2);
            let mut color = theme.as_ref().unwrap().foreground.convert_to_svg();
            if props.custom.current_step > i {
                color = theme.as_ref().unwrap().main.convert_to_svg();
            }
            html! {
                if i != 0 {
                    <line
                        class={classes!("hallings-line")}
                        x1={x.to_string()} x2={(x-step_size).to_string()}
                        y1={center.to_string()} y2={center.to_string()}
                        style={format!("stroke:{};stroke-width:5", color)}
                    />
                }
            }
        })
        .collect::<Html>();

    html! {
        <svg width={size.0.to_string()} height={size.1.to_string()}>
            {
                lines_display
            }
            {
                steps_display
            }
        </svg>
    }
}
