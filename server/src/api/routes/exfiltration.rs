// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::State;
use poem::web::Data;
use poem_openapi::OpenApi;

pub struct Exfiltration;

#[OpenApi]
impl Exfiltration {
    #[oai(path = "/", method = "post")]
    async fn send_exfiltrated_data(&self, state: Data<&State>) {}
}
