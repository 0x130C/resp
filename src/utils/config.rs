use std::fs;

pub struct JwtConfig {
    secret: String,
    signature_algorithm: String,
    token_vadidity_seconds: i64
}

pub fn load_config() -> Result<Config, Box<Error>> {
    let config_toml = fs::read_to_string("Config.toml")?;
    Ok(toml::from_str(&config_toml)?)
}

#[cfg(test)]
mod test {
    #[test]
    fn load_config_success()
}