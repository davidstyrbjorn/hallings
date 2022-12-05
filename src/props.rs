use crate::prelude::*;

/*
    Common props i expect all components to use
*/

#[derive(PartialEq, Properties)]
pub struct CommonProps<T: PartialEq> {
    #[prop_or_default]
    pub children: Children,
    pub size: Option<String>,
    pub class: Option<Classes>,
    pub custom: Option<T>,
}
