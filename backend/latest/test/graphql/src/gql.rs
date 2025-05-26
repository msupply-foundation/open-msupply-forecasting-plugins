use reqwest::{RequestBuilder, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use ApiError as Error;

const AUTH_QUERY: &str = r#"
query AuthToken($username: String!, $password: String) {
  root: authToken(password: $password, username: $username) {
    ... on AuthToken {
      __typename
      token
    }
    ... on AuthTokenError {
      __typename
      error {
        description
      }
    }
  }
}
"#;

#[derive(Debug)]
pub struct Api {
    url: Url,
    token: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct GraphQlResponse {
    data: Root,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Serialize)]
struct Root {
    root: serde_json::Value,
}

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error("Error while sending request to {1}")]
    SendingRequest(#[source] reqwest::Error, Url),
    #[error("Error while getting text, status {1:?}")]
    GettingText(#[source] reqwest::Error, reqwest::StatusCode),
    #[error("Error parsing gql response: {1}")]
    ParsingJson(#[source] serde_json::Error, String),
    #[error("Error validating typename, expected typename {expected_typename}, result: {json}")]
    ValidatingTypename {
        expected_typename: String,
        json: String,
    },
}

impl Api {
    pub async fn new_with_token(url: Url, username: &str, password: &str) -> Result<Self, Error> {
        let result = _gql(
            &url.join("graphql").unwrap(),
            AUTH_QUERY,
            serde_json::json! ({
              "username": username,
              "password": password,
            }),
            None,
            Some("AuthToken"),
        )
        .await?;

        let token = result["token"].as_str().unwrap().to_string();

        Ok(Api { url, token })
    }

    pub async fn gql(
        &self,
        query: &str,    
        variables: serde_json::Value,
        expected_typename: Option<&str>,
    ) -> Result<serde_json::Value, Error> {
        _gql(
            &self.url.join("graphql").unwrap(),
            query,
            variables,
            Some(&self.token),
            expected_typename,
        )
        .await
    }
}

async fn _gql(
    url: &Url,
    query: &str,
    variables: serde_json::Value,
    token: Option<&str>,
    expected_typename: Option<&str>,
) -> Result<serde_json::Value, Error> {
    let body = serde_json::json!({
        "query": query,
        "variables": variables
    });

    let mut client = reqwest::Client::new().post(url.clone());

    if let Some(token) = token {
        client = client.bearer_auth(token)
    };

    let built_request = client.json(&body);

    let json_result: GraphQlResponse = send_and_parse(built_request, url.clone()).await?;

    let result = json_result.data.root;

    let Some(expected_typename) = expected_typename else {
        return Ok(result);
    };

    if result["__typename"] != expected_typename {
        return Err(Error::ValidatingTypename {
            expected_typename: expected_typename.to_string(),
            json: serde_json::to_string(&result).unwrap(),
        });
    }

    Ok(result)
}

async fn send_and_parse<T: DeserializeOwned>(
    built_request: RequestBuilder,
    url: Url,
) -> Result<T, Error> {
    let response = built_request
        .send()
        .await
        .map_err(|e| Error::SendingRequest(e, url))?;

    let status = response.status();
    let text_result = response
        .text()
        .await
        .map_err(|e| Error::GettingText(e, status))?;

    Ok(serde_json::from_str(&text_result).map_err(|e| Error::ParsingJson(e, text_result))?)
}
