pub fn error_same_name(msg: String) -> tide::Error {
    tide::Error::from_str(
        tide::StatusCode::Conflict,
        format!("{msg} with the same name already exists"),
    )
}

pub fn error_internal_server() -> tide::Error {
    tide::Error::from_str(tide::StatusCode::InternalServerError, "Error")
}

pub fn error_bad_request(message: String) -> tide::Error {
    tide::Error::from_str(tide::StatusCode::BadRequest, message)
}

pub fn error_method_not_allowed(message: String) -> tide::Error {
    tide::Error::from_str(tide::StatusCode::MethodNotAllowed, message)
}

pub fn error_too_early(message: String) -> tide::Error {
    tide::Error::from_str(tide::StatusCode::TooEarly, message)
}
