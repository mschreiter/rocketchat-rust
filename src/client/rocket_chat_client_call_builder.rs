use std::any::Any;
use std::error;
use std::io::{Error, ErrorKind};

use reqwest::blocking::Client;
use reqwest::StatusCode;
use serde_json::Value;

use crate::client::rocket_chat_query_params::RocketChatQueryParams;
use crate::client::rocket_chat_rest_api_v1::RocketChatRestApiV1;
use crate::utils::http_methods::HttpMethods;

const CALL_METHOD_NAME_ARGUMENTS_KEY: &str = "methodNameArgs";

#[derive(Default)]
pub struct RocketChatClientCallBuilder {
    server_url: String,
    user: String,
    password: String,
    sha256password: String,
    auth_token: String,
    user_id: String,
}

impl RocketChatClientCallBuilder {
    pub fn new(
        server_url: String,
        user: String,
        password: String,
    ) -> Result<Self, Box<dyn error::Error>> {
        let server_url = if server_url.ends_with('/') {
            format!(
                "{}{}",
                server_url,
                if server_url.ends_with("api/") {
                    ""
                } else {
                    "api/"
                }
            )
        } else {
            format!(
                "{}{}",
                server_url,
                if server_url.ends_with("api") {
                    "/"
                } else {
                    "/api/"
                }
            )
        };

        Ok(RocketChatClientCallBuilder {
            server_url,
            user,
            password,
            sha256password: String::new(),
            auth_token: String::new(),
            user_id: String::new(),
        })
    }

    pub fn build_call_login(
        &mut self,
        call: RocketChatRestApiV1,
    ) -> Result<(), Box<dyn error::Error>> {
        self.build_call(call, None, None)
    }

    pub fn build_call_without_body(
        &mut self,
        call: RocketChatRestApiV1,
        query_params: RocketChatQueryParams,
    ) -> Result<(), Box<dyn error::Error>> {
        self.build_call(call, Some(query_params), None)
    }

    pub fn build_call(
        &mut self,
        call: RocketChatRestApiV1,
        query_params: Option<RocketChatQueryParams>,
        body: Option<Box<dyn Any>>,
    ) -> Result<(), Box<dyn error::Error>> {
        if call.requires_auth() && (self.auth_token.is_empty() || self.user_id.is_empty()) {
            self.login();
        }

        match call.http_method() {
            HttpMethods::Get => self.build_get_call(call, query_params),
            HttpMethods::Post => self.build_post_call(call, query_params, body),
            _ => {
                return Err(
                    Error::new(ErrorKind::Other, "The HTTP method is not supported.").into(),
                )
            }
        }
    }
    fn login(&mut self) -> Result<(), Box<dyn error::Error>> {
        let client = Client::new();
        let url = format!("{}v1/login", self.server_url);

        let response = client
            .post(&url)
            .form(&[("username", &self.user), ("password", &self.password)])
            .send()?;

        if response.status() == StatusCode::UNAUTHORIZED {
            return Err(Error::new(
                ErrorKind::Other,
                "The username and password provided are incorrect.",
            )
            .into());
        }

        let data = response.json::<Value>()?;
        let data = data
            .get("data")
            .ok_or_else(|| Error::new(ErrorKind::Other, "Data not found in login response"))?;
        self.auth_token = data["authToken"]
            .as_str()
            .ok_or_else(|| Error::new(ErrorKind::Other, "authToken not found"))?
            .to_string();
        self.user_id = data["userId"]
            .as_str()
            .ok_or_else(|| Error::new(ErrorKind::Other, "userId not found"))?
            .to_string();

        Ok(())
    }

    fn build_get_call(
        &self,
        call: RocketChatRestApiV1,
        query_params: Option<RocketChatQueryParams>,
    ) -> Result<(), Box<dyn error::Error>> {
        let method_name = self.prepare_call_method_name(&call, &query_params);
        let url = format!("{}{}", self.server_url, method_name);
        let client = Client::new();

        Ok({
            // let mut response = client.get(&url).send()?;
            // response.json::<RocketChatClientResponse>()
        })
    }

    fn build_post_call(
        &self,
        call: RocketChatRestApiV1,
        query_params: Option<RocketChatQueryParams>,
        body: Option<Box<dyn Any>>,
    ) -> Result<(), Box<dyn error::Error>> {
        let method_name = self.prepare_call_method_name(&call, &query_params);
        let url = format!("{}{}", self.server_url, method_name);
        let client = Client::new();
        Ok({
            // let mut response = client.post(&url).json(&body).send()?;
            // response.json::<RocketChatClientResponse>()?
        })
    }

    fn prepare_call_method_name(
        &self,
        call: &RocketChatRestApiV1,
        query_params: &Option<RocketChatQueryParams>,
    ) -> String {
        // Implement the prepare_call_method_name method here
        // You may need to adapt this based on the actual implementation
        String::from("") // Replace with the actual method
    }
}
