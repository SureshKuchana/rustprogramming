#![allow(dead_code, unused_variables)]

mod database {
    pub enum Status {
        Connected,
        Interrupted,
    }

    pub fn connect_to_database() -> Status {
        Status::Connected
    }

    pub fn get_user() {}
}

mod auth_utils {

    pub fn login(creds: models::Credentials) {
        // authenticate
        crate::database::get_user();
    }

    fn logout() {}

    // submodule
    pub mod models {
        pub struct Credentials {
            username: String,
            password: String,
        }
    }
}

use auth_utils::models::Credentials;
use database::Status;

pub fn authenicate(creds: Credentials) {
    if let Status::Connected = database::connect_to_database() {
        auth_utils::login(creds);
    }
}
