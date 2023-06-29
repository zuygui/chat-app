use actix_web::{error::{Error, JsonPayloadError, InternalError}, HttpRequest, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorJson {
    message: String,
}

pub fn json_error_handler(err: JsonPayloadError, _req: &HttpRequest) -> Error {
    let details = ErrorJson {
        message: err.to_string(),
    };

    log::debug!("Entering json error handler");

    let resp = match &err {
        JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType().json(details),
        JsonPayloadError::Deserialize(json_err) if json_err.is_data() => {
            HttpResponse::UnprocessableEntity().json(details)
        }
        _ => HttpResponse::BadRequest().json(details),
    };

    InternalError::from_response(err, resp).into()
}