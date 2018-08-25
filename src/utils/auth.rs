use jwt::{encode, decode, Header, Algorithm, Validation};

pub struct Credentials {
    username: String,
    password: String
}

pub struct Claims {
    uid: String,
    lang: String
}
