use app_core::{
    features::{auth::models::SessionUser, games},
    layouts,
};
use hypertext::prelude::*;

pub struct Props {
    pub user: SessionUser,
    pub path: String,
}

pub fn index(props: Props) -> Rendered<String> {
    let markup = rsx! {
        <form action="https://steamcommunity.com/openid/login" method="post">
            <input type="hidden" name="openid.identity" value="http://specs.openid.net/auth/2.0/identifier_select">
            <input type="hidden" name="openid.claimed_id" value="http://specs.openid.net/auth/2.0/identifier_select">
            <input type="hidden" name="openid.ns" value="http://specs.openid.net/auth/2.0">
            <input type="hidden" name="openid.mode" value="checkid_setup">
            <input type="hidden" name="openid.return_to" value="http://localhost:3000/steam-login">
            <button type="submit" class="block btn btn-primary btn-md w-fit mx-auto">
                "Sign in through"
                <span class="icon-[mdi--steam] size-8! ml-2"></span>
            </button>
        </form>
    };

    match props.user.steam_id {
        Some(_steam_id) => games::views::index(games::views::Props {
            path: props.path,
            recents: Vec::new(),
            games: Vec::new(),
            search_results: Vec::new(),
        }),
        None => layouts::default(layouts::default::Props {
            slot: markup,
            path: props.path,
        }),
    }
}
