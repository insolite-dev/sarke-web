//
// Copyright 2023-present Insolite. All rights reserved.
// Use of this source code is governed by Apache-2.0 license
// that can be found in the LICENSE file.
//

use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
       <div class="overlay">
          <div class="center-div">
             <div class="text-container">
                <div class="title">
                  <a href="/" alt="404 | Not found">{"404 | Not found"}</a>
                </div>
             </div>
          </div>
       </div>
    }
}
