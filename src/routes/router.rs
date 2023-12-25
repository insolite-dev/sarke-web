//
// Copyright 2023-present Insolite. All rights reserved.
// Use of this source code is governed by Apache-2.0 license
// that can be found in the LICENSE file.
//

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Home;
use crate::routes::NotFound;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

// Route switcher of application.
// Replaces the [current] route with given [Route]
// appropriate page.
pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home/> }
        }

        Route::NotFound => {
            html! { <NotFound/> }
        }
    }
}
