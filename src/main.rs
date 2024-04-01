use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router, routing::get,
};
use serde::Deserialize;
use tokio::sync::RwLock;
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

#[derive(Debug, Deserialize)]
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

type GlobalPrice = Arc<RwLock<Option<u64>>>;

