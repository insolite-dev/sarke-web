//
// Copyright 2023-present Insolite. All rights reserved.
// Use of this source code is governed by Apache-2.0 license
// that can be found in the LICENSE file.
//

pub mod app;
pub mod routes;
use app::App;

fn main() {
    yew::start_app::<App>();
}
