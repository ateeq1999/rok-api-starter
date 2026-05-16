use tower_http::cors::CorsLayer;

pub fn layer() -> CorsLayer {
    CorsLayer::permissive()
}
