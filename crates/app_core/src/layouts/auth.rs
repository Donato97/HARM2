use crate::helper::vite::load_manifest_entry;
use hypertext::prelude::*;

pub fn auth<S: Renderable>(slot: S) -> Rendered<String> {
    rsx! {
        <!DOCTYPE html>
        <html lang="it">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"HARM2"</title>

                (load_manifest_entry("frontend/style.css"))
            </head>
            <body>(slot)</body>
        </html>
    }
    .render()
}
