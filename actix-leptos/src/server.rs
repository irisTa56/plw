use crate::app::App;
use actix_files::Files;
use actix_web::{middleware::Compress, App, HttpServer};
use leptos::{leptos_config::ConfFile, view};
use leptos_actix::LeptosRoutes;

pub async fn run() -> std::io::Result<()> {
    let ConfFile { leptos_options } = leptos::get_configuration(None)
        .await
        .expect("Failed to get configuration");
    let addr = leptos_options.site_addr;

    println!("Serving at {}", addr);

    let routes = leptos_actix::generate_route_list(|cx| view! { cx, <App/> });

    HttpServer::new(move || {
        App::new()
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .service(Files::new("/", leptos_options.site_root.to_owned()))
            .wrap(Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}
