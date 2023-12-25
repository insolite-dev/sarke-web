//
// Copyright 2023-present Insolite. All rights reserved.
// Use of this source code is governed by Apache-2.0 license
// that can be found in the LICENSE file.
//

use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
       <div class="overlay">
          <div class="center-div">
             <img src="./public/fav.png" class="logo"/>
             <div class="gap"/>
             <div class="title">{"Sarke"}</div>
             <div class="divider"/>
             <div class="slog">{"... Comming Soon ..."}</div>
          </div>
       </div>
    }
}
