use hypertext::prelude::*;
use crate::layouts;

pub struct Props {
    pub path: String,
}

pub fn index(props: Props) -> Rendered<String> {
    let markup = rsx! {
        <h1> "Movies" </h1>
    };

    let layout_props = layouts::default::Props {
        slot: markup,
        path: props.path,
    };
    
    layouts::default(layout_props)
}
