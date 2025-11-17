use actix_web::{error, Error, web};
use serde::Deserialize;
use futures_util::StreamExt;

const MAX_SIZE: usize = 262_144; // max payload size is 256k

pub async fn handle_post<T: for<'a> Deserialize<'a>>(mut payload: web::Payload) -> Result<T, Error> {
    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }

    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<T>(&body)?;
    Ok(obj) // <- send response
}