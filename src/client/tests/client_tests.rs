#[cfg(test)]
mod tests {
    use crate::client::rocket_chat_client_call_builder::RocketChatClientCallBuilder;
    use crate::client::rocket_chat_rest_api_v1::RocketChatRestApiV1::ChannelsAddAll;

    #[test]
    fn test() {
        let mut client = RocketChatClientCallBuilder::new(
            String::from("http://localhost:3000"),
            String::from("admin"),
            String::from("admin"),
        )
        .unwrap();

        client.build_call_login(ChannelsAddAll).unwrap();
    }
}
