use actix_web::Error;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

#[derive(Serialize)]
pub struct FilteredResponse {
    missing_entries: Vec<String>,
    matched_entries: Vec<String>,
}

impl FilteredResponse {
    pub fn new() -> Self {
        FilteredResponse {
            missing_entries: Vec::new(),
            matched_entries: Vec::new(),
        }
    }

    pub fn add_missing(&mut self, item: &str) {
        self.missing_entries.push(item.to_owned());
    }

    pub fn add_matched(&mut self, item: &str) {
        self.matched_entries.push(item.to_owned());
    }
}

impl Responder for FilteredResponse {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S: 'static>(self, _request: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

#[derive(Deserialize)]
pub struct KeysRequest {
    pub keys: Vec<String>,
}
