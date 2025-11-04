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

#[cfg(test)]
mod tests {
    use crate::api::routes::sentence::Sentence;
    use poem::{Route, test::TestClient};
    use poem_openapi::OpenApiService;

    #[tokio::test]
    async fn test_get_sentence() {
        let sentence = "Hello, World!";
        let api = OpenApiService::new(Sentence::new(sentence.to_string()), "Sayware", "0.1.0");
        let application = Route::new().nest("/", api);
        let client = TestClient::new(application);

        client.get("/").send().await.assert_text(sentence).await;
    }
}
