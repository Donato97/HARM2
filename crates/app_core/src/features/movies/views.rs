use maud::{html, Markup};

use crate::layouts;

pub struct Props {
    pub path: String,
}

pub fn index(props: Props) -> Markup {
    let markup = html! {
        h1 { "Movies" }
    };

    let layout_props = layouts::default::Props {
        slot: markup,
        path: props.path,
    };
    layouts::default(layout_props)
}
