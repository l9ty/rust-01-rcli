use std::{fmt::Debug, io::Read};

use anyhow::Ok;

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub aud: String,
    pub exp: u64,
}

pub fn process_jwt_sign(key: &str, claims: &Claims) -> anyhow::Result<String> {
    let header = Header::new(Algorithm::HS256);
    let token = encode(&header, claims, &EncodingKey::from_secret(key.as_bytes()))?;
    Ok(token)
}

pub fn process_jwt_verify(token_reader: &mut dyn Read, key: &str) -> anyhow::Result<()> {
    let mut token = String::with_capacity(128);
    token_reader.read_to_string(&mut token)?;

    let key = DecodingKey::from_secret(key.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_aud = false;
    validation.leeway = 0;

    let _ =
        decode::<Claims>(&token, &key, &validation).map_err(|err| anyhow::anyhow!("{}", err))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::thread::sleep;

    use super::*;
    use anyhow::Result;

    #[test]
    fn t_jwt_sign_verify() -> Result<()> {
        let key = "rcli-test-key";
        let cliams = Claims {
            sub: "homework".to_string(),
            aud: "audience".to_string(),
            exp: jsonwebtoken::get_current_timestamp() + 3600,
        };
        let sig = process_jwt_sign(key, &cliams)?;
        process_jwt_verify(&mut sig.as_bytes(), key)
    }

    #[test]
    fn t_jwt_sign_verify_expired() -> Result<()> {
        let key = "rcli-test-key";
        let cliams = Claims {
            sub: "homework".to_string(),
            aud: "audience".to_string(),
            exp: jsonwebtoken::get_current_timestamp() + 1,
        };
        let sig = process_jwt_sign(key, &cliams)?;
        sleep(std::time::Duration::from_secs(2));
        let result = process_jwt_verify(&mut sig.as_bytes(), key);
        assert!(result.is_err());
        Ok(())
    }
}
