use hypertext::prelude::*;

use crate::layouts;

pub struct Props {
    pub path: String,
}

pub fn index(props: Props) -> Rendered<String> {
    let markup = rsx! {
        <h1> "Games" </h1>
    };

    let layout_props = layouts::default::Props {
        path: props.path,
        slot: markup,
    };

    layouts::default(layout_props)
}
