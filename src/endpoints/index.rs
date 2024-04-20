#[get("/")]
pub async fn index() -> String {
    "The donkeys have waited long enough. It is time to strike!".to_string()
}