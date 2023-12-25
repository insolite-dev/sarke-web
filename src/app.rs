//
// Copyright 2023-present Insolite. All rights reserved.
// Use of this source code is governed by Apache-2.0 license
// that can be found in the LICENSE file.
//

use stylist::{yew::styled_component, Style};
use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::*;

const STYLECSS: &str = include_str!("styles/main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let global_style = match Style::new(STYLECSS) {
        // TODO: Implement pretty error.
        Err(e) => return html! { format!("Something went wrong: {}", e) },
        Ok(s) => s,
    };

    html! {
      <div class={global_style}>
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
      </div>
    }
}
