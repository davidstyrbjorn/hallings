use crate::prelude::*;
use std::collections::HashMap;

pub struct StyleUtil;

impl StyleUtil {
    pub fn create_text_style(props: &CommonProps<TextProps>, theme: &Theme) -> String {
        let mut style_entries = HashMap::new();
        style_entries.insert("color", &theme.text_color);

        // Optional match the props against style entries
        if let Some(color) = &props.color {
            style_entries.insert("color", color);
        }
        if let Some(font_size) = &props.size {
            style_entries.insert("font-size", font_size);
        }

        let mut style = String::new();
        style_entries.keys().for_each(|key| {
            style += &format!("{}: {}; ", key, style_entries[key]);
        });

        style
    }

    pub fn create_button_style(props: &CommonPropsNoOption<ButtonProps>, theme: &Theme) -> String {
        let mut style_entries = HashMap::new();
        style_entries.insert("background-color", &theme.main);

        //special for button
        let border = String::from("none");
        style_entries.insert("border", &border);

        //border-radius style
        let border_radius = String::from("5px");
        style_entries.insert("border-radius", &border_radius);

        let padding = String::from("0px 10px");
        style_entries.insert("padding", &padding);

        let min_width = String::from("70px");
        style_entries.insert("min-width", &min_width);

        // Optional match the props against style entries
        if let Some(color) = &props.color {
            style_entries.insert("background-color", color);
        }
        if let Some(font_size) = &props.size {
            style_entries.insert("font-size", font_size);
        }

        let mut style = String::new();
        style_entries.keys().for_each(|key| {
            style += &format!("{}: {}; ", key, style_entries[key]);
        });

        style
    }

    pub fn create_jumpscare_style(props: &CommonProps<JumpScareProps>, theme: &Theme) -> String {
        let mut style_entries = HashMap::new();
        style_entries.insert("opacity", "100%");

        let mut style = String::new();
        style_entries.keys().for_each(|key| {
            style += &format!("{}: {}; ", key, style_entries[key]);
        });
        style
    }
}
