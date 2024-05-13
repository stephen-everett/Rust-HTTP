/*
    This validator validates a JWT given by the user
 */

use actix_web::{dev::ServiceRequest, error::Error,HttpMessage};
use actix_web_httpauth::extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use jwt::VerifyWithKey;

use crate::structs::app_state::TokenClaims;

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // create key using JWT_SECRET environment variable
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    
    // grab token from credentials passed from request
    let token_string = credentials.token();

    // check to see if token  is valid
    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid Token");

    // check claims. If the token is valid, pass it on to the route. If not return error
    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req.app_data::<bearer::Config>().cloned().unwrap_or_default().scope("");
            Err((AuthenticationError::from(config).into(), req))
        }

    }
}