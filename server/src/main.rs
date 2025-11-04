// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::api::routes::{Exfiltration, Sentence};
use anyhow::Result;
use config::{Config, Environment};
use poem::{EndpointExt, Route, Server, listener::TcpListener, middleware::SizeLimit};
use poem_openapi::OpenApiService;
use reqwest::{Client, Url};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use std::iter::once;
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

#[derive(Clone)]
pub struct State {
    proxy: Client,
    proxy_url: Url,
    instance_identifier: Uuid,
}

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = Config::builder()
        .add_source(Environment::default())
        .build()?
        .try_deserialize::<Configuration>()?;

    let ingrest = Client::builder()
        .default_headers(
            once((
                "X-Api-Key".parse()?,
                format!(
                    "{}.{}",
                    configuration.instance_identifier,
                    configuration.instance_secret.expose_secret()
                )
                .parse()?,
            ))
            .collect(),
        )
        .build()?;

    let api = OpenApiService::new(
        (Sentence::new(configuration.sentence), Exfiltration),
        "SayWare Server",
        "0.1.0",
    )
    .server(format!("https://localhost:{}", configuration.port));
    let application = Route::new()
        .nest(format!("/{}", configuration.url_prefix), api)
        .with(SizeLimit::new(2048))
        .data(State {
            proxy: ingrest,
            proxy_url: configuration.proxy_url.parse()?,
            instance_identifier: configuration.instance_identifier,
        });

    Ok(
        Server::new(TcpListener::bind(format!("0.0.0.0:{}", configuration.port)))
            .run(application)
            .await?,
    )
}
