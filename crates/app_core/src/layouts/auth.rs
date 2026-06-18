use crate::helper::vite::load_manifest_entry;
use maud::{html, Markup, DOCTYPE};

pub fn auth(slot: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="it" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "HARM2" }

                (load_manifest_entry("frontend/style.css"))
            }
            body { (slot) }
        }
    }
}
