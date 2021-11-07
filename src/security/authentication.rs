use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};

pub async fn ok_validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    if credentials.token() == "test" {
        Ok(req)
    } else {
        let config = req.app_data::<Config>()
            .map(|data| data.clone())
            .unwrap_or_else(Default::default)
            .scope("urn:example:channel=HBO&urn:example:rating=G,PG-13");

        Err(AuthenticationError::from(config).into())
    }
}
