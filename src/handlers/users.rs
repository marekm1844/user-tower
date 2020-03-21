use crate::db::MysqlPool;
use crate::model::user::User;
use serde_json::json;
use std::io;
use std::io::Error;
use tower_web::impl_web;

#[derive(Clone)]
pub struct UsersAPI {
    conn: MysqlPool,
}

#[derive(Response)]
struct UsersAPIResponse {
    users: Vec<User>,
}

// NOTE: test API without db conneciton;
fn users_mock() -> Result<User, Error> {
    let _users_json = r#"{"id":"89251ab3-1cdc-4629-9086-ce022cf6669e","first_name":"Marek","last_name":"Majdak","email":"info@sufrago.com","name":"sufrago","create_at":"2019-12-17T17:58:07.533406","avatar_id":"1cb15088-afd4-4d00-a7fc-d95eae1abefb","phone_no":"+48666666666","company_name":"Sufrago sp z o.o.","vat_id":"PL 9512468001"}"#;

    let user: User = serde_json::from_str(_users_json)?;

    Ok(user)
}
impl_web! {
    impl UsersAPI {

        pub fn new(conn: MysqlPool ) -> Self {
            Self { conn }
        }

    #[get("/")]
    #[content_type("json")]
    fn user_list(&self) -> Result<UsersAPIResponse, io::Error> {
        use crate::schema::user::dsl::*;
        use diesel::RunQueryDsl;

            let results = user.load::<User>(
                &self
                    .conn
                    .get().
                    map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        Ok( UsersAPIResponse {users: results} )
    }

    // #[get("/")]
    // #[content_type("json")]
    // fn user_list(&self) -> Result<UsersAPIResponse, io::Error> {

    //         let results = vec![users_mock().unwrap()];


    //     Ok( UsersAPIResponse {users: results} )
    // }

}
}
