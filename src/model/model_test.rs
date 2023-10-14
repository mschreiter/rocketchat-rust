#[cfg(test)]
mod tests {
    use model::attachment::Attachment;
    use model::email::Email;
    use model::user::User;
    use model::user_status::UserStatus::ONLINE;
    use model::user_type::UserType;
    use utils::json::{json_to_object, object_to_json};

    #[test]
    fn test_object_to_json() {
        let mut attachment = Attachment::new();
        attachment.set_color(String::from("color"));
        attachment.set_text(String::from("test"));
        attachment.set_ts(String::from("ts"));
        attachment.set_thumb_url(String::from("thumb_url"));
        attachment.set_message_link(String::from("message_link"));

        let attachment_json = object_to_json(&attachment);
        assert_eq!(attachment_json["color"], "color");
        assert_eq!(attachment_json["text"], "test");
        assert_eq!(attachment_json["ts"], "ts");
        assert_eq!(attachment_json["thumbUrl"], "thumb_url");
        assert_eq!(attachment_json["messageLink"], "message_link");
    }

    #[test]
    fn test_json_to_object() {
        let json = r#"
        {
            "color": "color",
            "text": "test",
            "ts": "ts",
            "thumbUrl": "thumb_url",
            "messageLink": "message_link"
        }
        "#;
        let attachment: Attachment = json_to_object(json);
        assert_eq!(attachment.color(), "color");
        assert_eq!(attachment.text(), "test");
        assert_eq!(attachment.ts(), "ts");
        assert_eq!(attachment.thumb_url(), "thumb_url");
        assert_eq!(attachment.message_link(), "message_link");
    }

    #[test]
    fn test_user_complex_to_json() {
        let mut user = User::new();
        user.set_id(String::from("id"));
        user.set_user_name(String::from("name"));
        user.set_name(String::from("status"));
        user.set_roles(vec![String::from("role1"), String::from("role2")]);
        user.set_created_at(String::from("created"));
        user.set_last_login(String::from("services"));
        user.set_user_type(UserType::USER);
        user.set_user_status(ONLINE);
        user.set_active(true);
        user.set_utc_offset(0);

        let mut mail: Email = Email::new();
        mail.set_address(String::from("test@mail.com"));
        mail.set_verified(true);
        user.set_email(vec![mail]);

        let user_json = object_to_json(&user);
        println!("{}", user_json);
    }

    #[test]
    fn test_json_to_user() {
        let json = r#"
        {"active":true,"createdAt":"created","email":[{"address":"test@mail.com","verified":true}],"id":"5","lastLogin":"services","name":"status","roles":["role1","role2"],"type":"user","userName":"name","userStatus":"online","utcOffset":0}"#;
        let user: User = json_to_object(json);
        assert_eq!(user.id(), "5");
    }
}
