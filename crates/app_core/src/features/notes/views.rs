use maud::{html, Markup};

use crate::layouts;

pub fn index() -> Markup {
    let markup = html! {
        div class="h-full" id="root" {}
    };

    let layout_props = layouts::default::Props {
        slot: markup,
        path: "".to_string(),
    };

    layouts::default(layout_props)
}
