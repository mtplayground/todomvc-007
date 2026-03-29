#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::post;
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, handle_server_fns, render_app_to_stream, LeptosRoutes};
    use tower_http::services::ServeDir;
    use todomvc_007::app::App;

    dotenvy::dotenv().ok();

    let conf = get_configuration(None).await.expect("Failed to get configuration");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let site_root = leptos_options.site_root.clone();

    let app = Router::new()
        .route("/api/*fn_name", post(handle_server_fns))
        .nest_service("/pkg", ServeDir::new(format!("{}/pkg", site_root)))
        .nest_service("/assets", ServeDir::new("public"))
        .leptos_routes(&leptos_options, routes, || view! { <App/> })
        .fallback(render_app_to_stream(
            leptos_options.clone(),
            || view! { <App/> },
        ))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    logging::log!("Listening on http://{}", addr);
    axum::serve(listener, app).await.expect("Failed to start server");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
