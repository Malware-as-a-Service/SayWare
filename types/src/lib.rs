// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExfiltratedData {
    pub operating_system: String,
    pub mac_address: String,
    pub hostname: String,
    pub username: String,
}
