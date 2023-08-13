mod app;
#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run().await
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
