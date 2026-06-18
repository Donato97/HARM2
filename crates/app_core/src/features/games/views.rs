use maud::{html, Markup};

use crate::layouts;

pub struct Props {
    pub path: String,
}

pub fn index(props: Props) -> Markup {
    let markup = html! {
        h1 { "Games" }
    };

    let layout_props = layouts::default::Props {
        path: props.path,
        slot: markup,
    };

    layouts::default(layout_props)
}
