// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::api::routes::{Exfiltration, Sentence};
use anyhow::Result;
use config::{Config, Environment};
use poem::{Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use secrecy::SecretString;
use serde::Deserialize;
use uuid::Uuid;

mod api;

#[derive(Debug, Default, Deserialize)]
struct Configuration {
    port: u16,
    proxy_url: String,
    instance_identifier: Uuid,
    instance_secret: SecretString,
    url_prefix: String,
    sentence: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Config::builder()
        .add_source(Environment::default())
        .build()?
        .try_deserialize::<Configuration>()?;

    let api = OpenApiService::new(
        (Sentence::new(configuration.sentence), Exfiltration),
        "SayWare Server",
        "0.1.0",
    )
    .server(format!("https://localhost:{}", configuration.port));
    let application = Route::new().nest(format!("/{}", configuration.url_prefix), api);

    Ok(
        Server::new(TcpListener::bind(format!("0.0.0.0:{}", configuration.port)))
            .run(application)
            .await?,
    )
}
