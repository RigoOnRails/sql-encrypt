use encrypted_message::{
    EncryptedMessage,
    encryption_type::Randomized,
    key_config::Secret,
    key_generator,
};

#[derive(Debug, Clone)]
struct UserKeyConfig {
    user_key: String,
}

impl encrypted_message::KeyConfig for UserKeyConfig {
    fn keys(&self) -> Vec<Secret<[u8; 32]>> {
        let salt = b"8NdZhr1RcdoaVyHYDrPOWuZu8WlBlTwI";
        vec![key_generator::derive_key_from(self.user_key.as_bytes(), salt, 2_u32.pow(16))]
    }
}

#[test]
fn key_config_with_external_dependency() {
    let key_config = UserKeyConfig {
        user_key: "rigos-weak-key-because-hes-a-human".to_string(),
    };

    // Encrypt a payload.
    let encrypted: EncryptedMessage<String, Randomized, UserKeyConfig> = {
        EncryptedMessage::encrypt_with_key_config("Hi".to_string(), key_config.clone()).unwrap()
    };

    // Decrypt the payload.
    let decrypted = encrypted.decrypt_with_key_config(key_config.clone()).unwrap();
    assert_eq!(decrypted, "Hi");

    // Create a new encrypted message with the same encryption type.
    let encrypted = encrypted.with_new_payload_and_key_config("Bonjour".to_string(), key_config.clone()).unwrap();

    // Decrypt the new payload.
    let decrypted = encrypted.decrypt_with_key_config(key_config).unwrap();
    assert_eq!(decrypted, "Bonjour");
}