use crate::db::ApiParam;
use crate::model::user::User;
use std::io;
use tower_web::impl_web;

#[derive(Clone)]
pub struct UsersAPI;

#[derive(Response)]
struct UsersAPIResponse {
    users: Vec<User>,
}

impl_web! {
    impl UsersAPI {


    #[get("/")]
    #[content_type("json")]
    fn user_list(&self, param: ApiParam) -> Result<UsersAPIResponse, io::Error> {
        use crate::schema::user::dsl::*;
        use diesel::RunQueryDsl;

            let results = user.load::<User>(
                &param
                    .connection
                    .get().
                    map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;


        Ok( UsersAPIResponse {users: results} )
    }
}
}
