// mochou-p/game/web/backend/src/response.rs

use rspond::*;


pub fn ok(media_type: MediaType, body: Vec<u8>) -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::Successful(Successful::Ok))
        .headers(vec![
            Header::Connection(Connection::Close),
            Header::ContentType(media_type),
            Header::ContentLength(body.len())
        ])
        .body(&body)
        .build()
}

pub fn bad_request() -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::ClientError(ClientError::BadRequest))
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn not_found() -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::ClientError(ClientError::NotFound))
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn not_implemented() -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::ServerError(ServerError::NotImplemented))
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn internal_server_error() -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::ServerError(ServerError::InternalServerError))
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn http_version_not_supported() -> Vec<u8> {
    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::ServerError(ServerError::HttpVersionNotSupported))
        .headers(vec![Header::Connection(Connection::Close)])
        .empty_body()
        .build()
}

pub fn see_other(location: String, appended_headers: Vec<Header>) -> Vec<u8> {
    let mut headers = Vec::with_capacity(2 + appended_headers.len());

    headers.extend([
        Header::Connection(Connection::Close),
        Header::Custom(String::from("Location"), location)
    ]);

    headers.extend(appended_headers);

    ResponseBuilder::new()
        .http_version(HttpVersion::OneOne)
        .status_code(StatusCode::Redirection(Redirection::SeeOther))
        .headers(headers)
        .empty_body()
        .build()
}

