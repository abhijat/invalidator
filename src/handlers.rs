use actix_web::*;

use crate::api::FilteredResponse;
use crate::api::KeysRequest;
use crate::bloom_filter::AppState;

pub fn push_keys((request, payload): (HttpRequest<AppState>, Json<KeysRequest>)) -> Result<HttpResponse> {
    let items = &payload.keys;

    let mut filter = request.state().filter.lock().unwrap();
    items.iter().for_each(|item| filter.put(item));

    Ok(HttpResponse::Ok()
        .json(
            format!("added {} objects to filter", items.len())
        ))
}

pub fn check_keys((request, payload): (HttpRequest<AppState>, Json<KeysRequest>)) -> impl Responder {
    let keys = &payload.keys;
    let filter = request.state().filter.lock().unwrap();
    let mut response = FilteredResponse::new();
    keys.iter().for_each(|key| {
        if filter.get(key) {
            response.add_matched(key);
        } else {
            response.add_missing(key);
        }
    });
    response
}
