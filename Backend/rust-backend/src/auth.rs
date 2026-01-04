//The handler probably 

use chrono::{Duration,Utc};
use jsonwebtoken::{decode,DecodingKey,encode, EncodingKey,Header,TokenData,Validation};
use serde::{Deserialize,Serialize};

#[derive(Debug Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,
    pub exp: i64
}

//Sample function member_id, secret
pub fn generate_jwt(member_id:i32,secret:&str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(3600)) //This can be adjusted here. Token validity is 1h.
        .expect("Timestamp invalid..!")
        .timestamp();

    let claims = Claims {
        sub: member_id.to_owned(),
        exp: expiration,
    };

    //Write an if else condition to check if else
    encode(&Header::default(),&claims, &EncodingKey::from_secret(secret.as_ref()))   
}

pub fn decode_token(token:&str,secret:&str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error>
{
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default()
    )
}

pub fn is_valid(claims:&Claims) -> bool {
    claim.exp > Utc::now.timestamp()
}