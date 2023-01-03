use crate::prelude::*;

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

#[function_component]
pub fn StepsLeft(props: &CommonPropsNoOption<StepsLeftProps>) -> Html {
    let size: (usize, usize) = (props.custom.width, props.custom.height);

    // Calculate step size as width of canvas / numbe of steps
    let step_size = props.custom.width / props.custom.steps.len();
    let center = props.custom.height / 2;

    let steps = props.custom.steps.clone();
    // Generate the steps i.e circles
    let steps_display = steps
        .into_iter()
        .enumerate()
        .map(|(i, step)| {
            let x = (i * step_size) + (step_size / 2);
            let mut color = "rgb(100,100,100)";
            if props.custom.current_step > i {
                color = "rgb(0,0,0)";
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
                        fill={color}
                        cx={x.to_string()}
                        cy={center.to_string()}
                        r="15"
                    />
                </>
            }
        })
        .collect::<Html>();

    let steps = props.custom.steps.clone();
    let lines_display = steps
        .into_iter()
        .enumerate()
        .map(|(i, _step)| {
            let x = (i * step_size) + (step_size / 2);
            let mut color = "rgb(100,100,100)";
            if props.custom.current_step > i {
                color = "rgb(0,0,0)";
            }
            html! {
                if i != 0 {
                    <line
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
