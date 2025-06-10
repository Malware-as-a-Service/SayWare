// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use poem::{Route, Server, listener::TcpListener};
use poem_openapi::{OpenApi, OpenApiService, payload::PlainText};
use std::{env, io::Error};

struct Api {
    sentence: String,
}

#[OpenApi]
impl Api {
    fn new(sentence: String) -> Self {
        Self { sentence }
    }

    #[oai(path = "/", method = "get")]
    async fn index(&self) -> PlainText<String> {
        PlainText(self.sentence.clone())
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let port = env::var("PORT").expect("PORT must be set");
    let sentence = env::var("SENTENCE").expect("SENTENCE must be set");

    let api = OpenApiService::new(Api::new(sentence), "SayWare Server", "1.0.0")
        .server(format!("http://localhost:{port}"));
    let application = Route::new().nest("/", api);

    Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
        .run(application)
        .await
}
