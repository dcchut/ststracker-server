use std::sync::Mutex;

use actix_http::Response;
use actix_web::{web, web::Data, Error, HttpRequest, HttpResponse};
use futures::IntoFuture;
use itsdangerous::{default_builder, Signer};

use crate::model::twitch::verify_auth_header;
use crate::model::{ApplicationState, GetAPIKeyResponse, GetRequest, UpdateRequest};

pub fn get(
    state: Data<Mutex<ApplicationState>>,
    request: HttpRequest,
) -> impl IntoFuture<Item = Response, Error = Error> {
    let mut state = state.lock().unwrap();

    match verify_auth_header(&state, &request) {
        Some(payload) => {
            let channel_data = state.get(payload.channel_id());

            if let Ok(channel_data) = channel_data {
                if let Ok(channel_data) = serde_json::from_str::<GetRequest>(&channel_data) {
                    return HttpResponse::Ok().json(channel_data);
                }
            }

            HttpResponse::InternalServerError().finish()
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}

pub fn view_api_key(
    state: Data<Mutex<ApplicationState>>,
    request: HttpRequest,
) -> impl IntoFuture<Item = Response, Error = Error> {
    let state = state.lock().unwrap();

    match verify_auth_header(&state, &request) {
        Some(payload) => {
            if payload.role() != "broadcaster" {
                return HttpResponse::Unauthorized().finish();
            }

            // Sign our key
            let signer = default_builder(state.backend_secret_key()).build();
            let key = signer.sign(payload.channel_id());

            // Send the key along
            HttpResponse::Ok().json(GetAPIKeyResponse::new(key))
        }
        None => HttpResponse::Unauthorized().finish(),
    }
}

pub fn update(
    update: web::Json<UpdateRequest>,
    state: Data<Mutex<ApplicationState>>,
) -> impl IntoFuture<Item = Response, Error = Error> {
    let mut state = state.lock().unwrap();
    let update = update.into_inner();
    let signer = default_builder(state.backend_secret_key()).build();

    // Verify that the key is valid
    if let Ok(channel_id) = signer.unsign(&update.key) {
        // Update the cards / relics for this channel id
        let update = GetRequest::new(update.cards, update.relics);
        let update_response = serde_json::to_string(&update).unwrap();

        if state.set(channel_id, &update_response).is_ok() {
            HttpResponse::Ok().finish()
        } else {
            HttpResponse::InternalServerError().finish()
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
