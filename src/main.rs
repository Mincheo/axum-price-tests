use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router, routing::get,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

type GlobalPrice = Arc<RwLock<Option<u64>>>;

#[tokio::main]
async fn main() {

let global_price = Arc::new(RwLock::new(None));
let app = app(global_price);

let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
println!("Listening on 127.0.0.1:3000");
axum::serve(listener, app).await.unwrap();

}

fn app(state: GlobalPrice) -> Router {
    Router::new()
        .route("/price", get(get_price).patch(upd_price).delete(del_price))
        .with_state(state)
}

#[derive(Debug, Serialize, Deserialize)]
struct PriceStruct {
    price: u64,
}

async fn get_price(
	State(global_price): State<GlobalPrice>,
	) -> Result<impl IntoResponse, StatusCode> {
	let global_price = global_price.read().await;
	if let Some(price) = *global_price{
		Ok(price.to_string())
	}
	else {
		Err(StatusCode::NOT_FOUND)
	}
}


async fn upd_price(
    State(global_price): State<GlobalPrice>,
    Json(input): Json<PriceStruct>,
) -> Result<impl IntoResponse, StatusCode> {
    let price = input.price;
    let mut global_price = global_price.write().await;
    *global_price = Some(price);

    Ok(StatusCode::OK)
}

async fn del_price(
State(global_price): State<GlobalPrice>
)   -> Result<impl IntoResponse, StatusCode> {
let mut global_price = global_price.write().await;
*global_price = None;
Ok(StatusCode::OK)
}



#[cfg(test)]
mod tests{
    use super::*;
    use ::axum_test::TestServer;

    #[tokio::test]
    async fn test_get_price_ok(){
        let state: GlobalPrice = Arc::new(RwLock::new(Some(10)));
        let app = app(state);
        let server = TestServer::new(app).unwrap();
        let response = server.get("/price").await;
        assert_eq!(response.text(), "10");
    }

    #[tokio::test]
    async fn test_get_price_not_found(){
        let state:GlobalPrice = Arc::new(RwLock::new(None));
        let app = app(state);
        let server = TestServer::new(app).unwrap();
        let response = server.get("/price").await;
        assert_eq!(response.status_code(),StatusCode::NOT_FOUND);

    }
    #[tokio::test]
    async fn test_upd_price(){
        let state:GlobalPrice = Arc::new(RwLock::new(None));
        let app = app(state);
        let server = TestServer::new(app).unwrap();
        let _response1 = server.patch("/price").json(&PriceStruct {price:10}).await;
        assert_eq!(_response1.status_code(),StatusCode::OK);
        let _response2 = server.get("/price").await;
        assert_eq!(_response2.text(), "10");
        }
    
    #[tokio::test]
    async fn test_del_price(){
        let state: GlobalPrice = Arc::new(RwLock::new(Some(10)));
        let app = app(state);
        let server = TestServer::new(app).unwrap();
        let response = server.delete("/price").await;
        assert_eq!(response.status_code(),StatusCode::OK);

    }
}