// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use poem_openapi::Object;

#[derive(Object)]
pub struct ExfiltratedData {
    pub operating_system_version: String,
    pub mac_address: String,
    pub hostname: String,
    pub username: String,
}
