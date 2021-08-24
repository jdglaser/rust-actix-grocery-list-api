use actix_web::{get, post, Responder, HttpResponse, web, http};

use crate::state;

use crate::item::{Item, ItemTemplate};
use crate::auth::AuthorizationService;
use crate::util::errors::CustomError;


#[get("/{id}")]
async fn get_item(_: AuthorizationService, state: web::Data<state::AppState>, id: web::Path<i32>) -> impl Responder {
    let result = Item::get_item(&state.database_pool, *id).await;

    match result {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(_) => Err(CustomError(
            format!("Item {} not found", id),
            http::StatusCode::NOT_FOUND
        ))
    }
}

#[get("")]
async fn get_items(_: AuthorizationService, state: web::Data<state::AppState>) -> impl Responder {
    let result = Item::get_items(&state.database_pool).await;

    match result {
        Ok(items) => Ok(HttpResponse::Ok().json(items)),
        Err(error) => {
            error!("Error: {}", error.to_string());
            Err(CustomError("Problem getting items from database", http::StatusCode::BAD_REQUEST))
        }
    }
}

#[post("")]
async fn create_item(_: AuthorizationService,
                     state: web::Data<state::AppState>, 
                     new_item: web::Json<ItemTemplate>) -> impl Responder {
    let created_item = Item::create_item(&state.database_pool, new_item.into_inner()).await;

    match created_item {
        Ok(item) => Ok(HttpResponse::Ok().json(item)),
        Err(err) => Err(CustomError(format!("Error creating item: {}", err), http::StatusCode::BAD_REQUEST))
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/item")
            .service(get_item)
            .service(get_items)
            .service(create_item)
    );
}

#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, body::Body, dev};
    use crate::item::{Item, ItemTemplate};
    use crate::util::test_util;
    use serde_json::{json, Value};

    #[actix_rt::test]
    async fn test_create_item() {
        let db = test_util::setup_test_db().await;

        let new_item = ItemTemplate {
            name: "foo".to_string(),
            category: "bar".to_string()
        };

        let mut req = test::TestRequest::post()
            .uri("/item")
            .set_json(&new_item);
        req = test_util::login_test_user(req, &db).await;

        let mut resp = test_util::make_request(req, &db).await;
        println!("{:?}", resp);
        //assert!(resp.status().is_success());

        let body = resp.take_body();
        let body = body.as_ref().unwrap();
        if let dev::Body::Bytes(bytes) = body {
            let string = std::str::from_utf8(&bytes.as_ref()).unwrap();
            let body: Value = serde_json::from_str(string).unwrap();
            println!("{:?}", body);
            println!("{}", body["itemId"])
        }
        //assert!(resp.status().is_success());
        /*assert_eq!(
            &Body::from(json!({"name":"Test"})), // or serde.....
            body
        );*/

        assert_eq!(1,2);
    }

}

