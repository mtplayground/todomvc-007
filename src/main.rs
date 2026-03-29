#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::routing::post;
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, handle_server_fns, render_app_to_stream, LeptosRoutes};
    use tower_http::services::ServeDir;
    use todomvc_007::app::App;
    use todomvc_007::db::{get_db, run_migrations};

    dotenvy::dotenv().ok();

    // Initialize database pool
    let pool = get_db().await.expect("Failed to connect to database");
    run_migrations(&pool).await.expect("Failed to run migrations");

    let conf = get_configuration(None).await.expect("Failed to get configuration");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let site_root = leptos_options.site_root.clone();

    let app = Router::new()
        .route("/api/*fn_name", post(handle_server_fns))
        .nest_service("/pkg", ServeDir::new(format!("{}/pkg", site_root)))
        .nest_service("/assets", ServeDir::new("public"))
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool.clone();
                move || {
                    leptos::provide_context(pool.clone());
                }
            },
            || view! { <App/> },
        )
        .fallback(render_app_to_stream(
            leptos_options.clone(),
            || view! { <App/> },
        ))
        .layer(axum::Extension(pool))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    logging::log!("Listening on http://{}", addr);
    axum::serve(listener, app).await.expect("Failed to start server");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
