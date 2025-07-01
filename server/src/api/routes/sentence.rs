// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use poem_openapi::{OpenApi, payload::PlainText};

pub struct Sentence {
    sentence: String,
}

#[OpenApi]
impl Sentence {
    pub fn new(sentence: String) -> Self {
        Self { sentence }
    }

    #[oai(path = "/", method = "get")]
    async fn get_sentence(&self) -> PlainText<String> {
        PlainText(self.sentence.clone())
    }
}
