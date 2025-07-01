// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::api::routes::Sentence;
use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use std::{env, io::Error};

mod api;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let port = env::var("PORT").expect("PORT must be set");
    let sentence = env::var("SENTENCE").expect("SENTENCE must be set");

    let api = OpenApiService::new(Sentence::new(sentence), "SayWare Server", "0.1.0")
        .server(format!("https://localhost:{port}"));
    let application = Route::new().nest("/", api);

    Server::new(TcpListener::bind(format!("0.0.0.0:{port}")))
        .run(application)
        .await
}
