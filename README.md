# :scissors:+:crab: -> Hallings - components for Yew

## :question: Purpose

### <i>What if there existed a couple of pre-built components for yew like password strength checker or steps-left? Wonder no more.</i>

:::warning
Note: this is not a finished library it is meant as a prototype/test during a master's course at LiU (Sweden), it should not be used for any real world projects :slightly_smiling_face:
:::

## Prerequisites

:::info
Yew: version "0.20"
:::

## :checkered_flag: Getting started

1. [Install Yew](https://yew.rs/docs/getting-started/introduction)
2. Add in `Cargo.toml` file

```toml=
hallings = "0.1"
```

3. Bring in the library by doing <code>use hallings::prelude::\*</code>
   :::success
   As simple as that to get started
   :::

## :pencil: Running tests

All tests are located in <code>test.rs</code>.
To run tests make sure you have installed everything required, then run.

<ul>
    <li><code>wasm-pack test --chrome</code>   </li> 
</ul>

You could replace <code>--chrome</code> with <code>--firefox</code>. Check the [wasm-pack documentation for more info](https://rustwasm.github.io/wasm-pack/book/commands/test.html)

## :ok_hand: Check out our usage examples below...

### Provider & Theme

Halling uses [Context Providers](https://yew.rs/docs/concepts/contexts) to provide components with theme data. Currently you can not pass your own theme struct.

```html
<MaestroProvider> halling components </MaestroProvider>
```

All of the examples have been tested using <code>yew = "0.20"</code> inside [Function Components](https://yew.rs/docs/concepts/function-components)

### Text

```htmlmixed=
<Text
    size={"40px"}
    color={"purple"} // if none is specified theme color is used
    class={classes!("Custom class")}
>
    {"Pretty large text"}
</Text>
```

Or feed text through as prop variable,

```htmlmixed
<Text size={"40px"} custom={TextProps { label: "Test".into() }}/>
```

### Button

```rust=
let counter = use_state(|| 0);

let click = {
    let counter = counter.clone();
    Callback::from(move |_s: yew::MouseEvent| {
        counter.set(*counter + 1);
    })
};

<Button custom={
    ButtonProps { onclick: cb }
}>
    <Text color={"white"}>{"Next"}</Text>
</Button>
```

### Password Strength Checker

This component provides a password input component while also providing feedback dependent on how strong the given password currently is. You can provide your own strength checking function and formatting functions. Right now there is no direct way to extract the password value.

```rust=
<PasswordStrengthInput
	custom = { PasswordStrengthInputProps
	{
        calculate_strength_level: None,
        strength_level_to_text_and_color: None,
        strength_callback: None
	}}
/>
```

<i>Example 1 (default behaviour)</i>

```rust=
pub fn calculate_strength_level(value: String) -> StrengthLevel {
    if value.contains("secure") {
        return StrengthLevel::HIGH;
    }
    StrengthLevel::LOW
}

pub fn on_level_change(strength_level: StrengthLevel) {
    // use strength_level
}

fn strength_level_to_text_and_color(value: StrengthLevel) -> (String, String) {
    match value {
        StrengthLevel::LOW => ("Password not strong enough".into(), "red".into()),
        StrengthLevel::MEDIUM => ("Password weak but passable".into(), "blue".into()),
        StrengthLevel::HIGH => ("Password strong".into(), "green".into()),
    }
}

<PasswordStrengthInput
    custom = { PasswordStrengthInputProps {
        calculate_strength_level: Some(calculate_strength_level),
        strength_level_to_text_and_color: Some(strength_level_to_text_and_color),
        strength_callback: Some(on_level_change)
    }}
/>
```

<i>Example 2 (customized behaviour)</i>

### Steps Left

Steps left draws its UI using <code>svg, circle</code> & <code>lines</code>. The width and height dictates the width and height of the svg image. You can feed however many steps and the component will dynamically position and space the steps.

```rust=
<StepsLeft
    custom = {
        StepsLeftProps {
            width: 800,
            height: 200,
            current_step: (*counter).clone(),
            steps: vec![
                Step {
                    label: "Check out".into()
                },
                Step {
                    label: "Confirm".into()
                },
                Step {
                    label: "Pay".into()
                },
            ]
        }
    }
/>
```
