use hypertext::prelude::*;

use crate::helper::vite::load_manifest_entry;

pub struct Props<S: Renderable> {
    pub slot: S,
    pub path: String,
}

pub fn default<S: Renderable>(props: Props<S>) -> Rendered<String> {
    let d_notes = if props.path == "" {
        "menu-active text-primary"
    } else {
        ""
    };
    let d_games = if props.path == "/games" {
        "menu-active text-primary"
    } else {
        ""
    };
    let d_movies = if props.path == "/movies" {
        "menu-active text-primary"
    } else {
        ""
    };

    let m_notes = if props.path == "" {
        "basis-1/2 dock-active text-primary"
    } else {
        "basis-1/2"
    };
    let m_games = if props.path == "/games" {
        "basis-1/2 dock-active text-primary"
    } else {
        "basis-1/2"
    };
    let m_movies = if props.path == "/movies" {
        "basis-1/2 dock-active text-primary"
    } else {
        "basis-1/2"
    };

    rsx! {
        <!DOCTYPE html>
        <html lang="it">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"HARM2"</title>

                (load_manifest_entry("frontend/style.css"))
                (load_manifest_entry("frontend/htmx.ts"))
            </head>
            <body>
                <div class="flex flex-col md:flex-row h-screen">
                    <nav id="desktop-nav" class="hidden md:flex flex-col items-center grow-0 shrink-0 bg-base-200 border-r border-stone-800">
                        <div></div>

                        <ul "hx-boost:inherited"="true" class="menu">
                            <li>
                                <a class=(d_notes) href="/">
                                    <span class="icon-[material-symbols--note-stack-outline] my-1.5 inline-block size-5"></span>
                                </a>
                            </li>
                            <li>
                                <a class=(d_games) href="/games">
                                    <span class="icon-[material-symbols--stadia-controller-outline] my-1.5 inline-block size-5"></span>
                                </a>
                            </li>
                            <li>
                                <a class=(d_movies) href="/movies">
                                    <span class="icon-[material-symbols--movie-outline] my-1.5 inline-block size-5"></span>
                                </a>
                            </li>
                        </ul>
                    </nav>

                    <div id="content" class="h-full flex-1 min-h-0">
                        (props.slot)
                    </div>

                    <nav id="mobile-nav" class="md:hidden dock dock-sm static bg-base-200 justify-center border-t border-stone-800">
                        <a class=(m_notes) href="/">
                            <span class="icon-[material-symbols--note-stack-outline] my-1.5 inline-block size-5"></span>
                        </a>
                        <a class=(m_games) href="/games">
                            <span class="icon-[material-symbols--stadia-controller-outline] my-1.5 inline-block size-5"></span>
                        </a>
                        <a class=(m_movies) href="/movies">
                            <span class="icon-[material-symbols--movie-outline] my-1.5 inline-block size-5"></span>
                        </a>
                    </nav>
                </div>

                <dialog id="confirm-modal" class="modal">
                    <div class="modal-box">
                        <h3 class="text-lg font-bold">"Are you sure?"</h3>
                        <p class="py-4">
                            "You are about to delete this folder and all its contents. This action cannot be undone."
                        </p>
                        <div class="modal-action">
                            <form method="dialog">
                                <button type="submit" class="btn btn-ghost">"Cancel"</button>
                            </form>
                            <button class="btn btn-primary">"Confirm"</button>
                        </div>
                    </div>
                </dialog>
            </body>
        </html>
    }.render()
}
