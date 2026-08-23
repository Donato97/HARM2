use super::models::SteamSearchGames;
use hypertext::prelude::*;

use crate::layouts;

pub struct Props {
    pub path: String,
    pub search_results: SteamSearchGames,
}

#[component]
pub fn search_results_list<'a>(games: &'a SteamSearchGames) -> impl Renderable {
    rsx! {
        <ul id="searchResults" class="list">
            @if !games.is_empty() {
                @for game in games {
                    <li>
                        <button class="flex gap-2 items-center p-4 focus:bg-base-300 w-full">
                            <img class="w-[115px] h-[43px] shrink-0" alt=(&game.name) src=(&game.tiny_image) />
                            <div class="flex flex-col sm:flex-row justify-between min-w-0 w-full gap-x-8">
                                <span class="truncate">(&game.name)</span>
                                <span class="opacity-80">(&game.id)</span>
                            </div>
                         </button>
                    </li>
                 }
             } @else {
                <li class="py-12 text-center text-sm text-neutral-content">
                    <p>"No results"</p>
                    <p>"Type at least 3 characters"</p>
                </li>
             }
        </ul>
    }
}

pub fn index(props: Props) -> Rendered<String> {
    let markup = rsx! {
        <h1> "Games" </h1>


        <button class="btn" onclick="searchModal.showModal()">open modal</button>
        <dialog id="searchModal" class="modal">
            <div class="modal-box border border-neutral p-0 max-w-xl"
                x-data
                "@keydown.down"="$focus.next()"
                 "@keydown.up"="$focus.previous()">

               <label class="input input-ghost w-full outline-none bg-base-200">
                    <span class="text-primary">"/"</span>
                    <input
                        hx-trigger="keyup[key=='Enter'&&target.value.length>2]"
                        hx-get="/games/search"
                        hx-target="#searchResults"
                        hx-swap="innerHTML"
                        name="query"
                        type="text"
                        class="grow" />
                </label>

                <div class="border-y border-neutral overflow-y-auto w-full">
                    <SearchResultsList games=(&props.search_results) />
                </div>

                <div class="text-xs opacity-75 p-2">
                    <kbd class="kbd kbd-xs">"⇅"</kbd> " Move • "
                    <kbd class="kbd kbd-xs">"↲"</kbd> " Add • "
                    <kbd class="kbd kbd-xs">"esc"</kbd> " Close"
                </div>
            </div>
            <form method="dialog" class="modal-backdrop">
                <button>close</button>
            </form>
        </dialog>
    };

    let layout_props = layouts::default::Props {
        path: props.path,
        slot: markup,
    };

    layouts::default(layout_props)
}
