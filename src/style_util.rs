use std::collections::HashMap;

use crate::prelude::*;

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

        // Special for text

        return style.to_owned();
    }
}
