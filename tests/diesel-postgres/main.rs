#![cfg(all(feature = "diesel", feature = "diesel-postgres"))]

mod schema;

use diesel::prelude::*;
use encrypted_message::{
    EncryptedMessage,
    strategy::{Randomized, Deterministic},
    key_config::Secret,
};

#[derive(Debug, Default)]
struct KeyConfig;
impl encrypted_message::KeyConfig for KeyConfig {
    fn keys(&self) -> Vec<Secret<[u8; 32]>> {
        vec![(*b"uuOxfpWgRgIEo3dIrdo0hnHJHF1hntvW").into()]
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct User {
    id: i32,
    json: Option<EncryptedMessage<String, Randomized, KeyConfig>>,
    jsonb: Option<EncryptedMessage<String, Deterministic, KeyConfig>>,
}

#[derive(Insertable)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserInsertable {
    json: Option<EncryptedMessage<String, Randomized, KeyConfig>>,
    jsonb: Option<EncryptedMessage<String, Deterministic, KeyConfig>>,
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
struct UserChangeset {
    json: Option<Option<EncryptedMessage<String, Randomized, KeyConfig>>>,
    jsonb: Option<Option<EncryptedMessage<String, Deterministic, KeyConfig>>>,
}

#[test]
fn encrypted_message_works() {
    // Attempt to load environment variables from .env.test
    let _ = dotenvy::from_filename(".env.test");

    let database_url = dotenvy::var("POSTGRES_DATABASE_URL").expect("POSTGRES_DATABASE_URL must be set.");
    let mut connection = PgConnection::establish(&database_url).unwrap();

    // Create a new user.
    let user: User = diesel::insert_into(schema::users::table)
        .values(UserInsertable {
            json: Some(EncryptedMessage::encrypt("Very secret.".to_string()).unwrap()),
            jsonb: Some(EncryptedMessage::encrypt("Very secret, also binary.".to_string()).unwrap()),
        })
        .get_result(&mut connection)
        .unwrap();

    // Decrypt the user's secrets.
    assert_eq!(user.json.as_ref().unwrap().decrypt().unwrap(), "Very secret.");
    assert_eq!(user.jsonb.as_ref().unwrap().decrypt().unwrap(), "Very secret, also binary.");
}
