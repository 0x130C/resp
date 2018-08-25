use jwt::{encode, decode, Header, Algorithm, Validation};
use utils::config::JwtConfig;
pub struct JwtService {
    secret: String,
    token_validity_seconds: i64,
    header: Header,
    validation: Validation
}


#[derive(Fail, Debug)]
pub enum JwtError {
    #[fail(display = "InvalidTokenError: [{}]", message)]
    InvalidTokenError { message: String },
    #[fail(display = "ExpiredTokenError: [{}]", message)]
    ExpiredTokenError { message: String },
    #[fail(display = "GenerateTokenError: [{}]", message)]
    GenerateTokenError { message: String },
}


impl JwtService {
    pub fn new(jwt_config: &JwtConfig) -> Self {
        let alg = signature_algorithm_from_str(&jwt_config.signature_algorithm);
        JwtService {
            secret: jwt_config.secret.clone(),
            token_validity_seconds: jwt_config.token_vadidity_seconds,
            header: Header {
                alg,
                ..Header::default()
            },
            validation: Validation::new(alg)
        }
    }
}

pub fn signature_algorithm_from_str(alg: &str) -> Algorithm {
    match alg {
        "HS256" => Algorithm::HS256,
        "HS384" => Algorithm::HS384,
        "HS512" => Algorithm::HS512,
        "RS256" => Algorithm::RS256,
        "RS384" => Algorithm::RS384,
        "RS512" => Algorithm::RS512,
        _ => panic!("Unknown JWT signature algorithm: [{}]", alg),
    }
}