#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{
        generate_route_list, handle_server_fns_with_context, render_app_to_stream_with_context,
        LeptosRoutes,
    };
    use sqlx::SqlitePool;
    use tower_http::services::ServeDir;
    use todomvc_007::app::App;
    use todomvc_007::db::{get_db, run_migrations};

    dotenvy::dotenv().ok();

    // Initialize database pool and run migrations
    let pool = get_db().await.expect("Failed to connect to database");
    run_migrations(&pool).await.expect("Failed to run migrations");

    let conf = get_configuration(None).await.expect("Failed to get configuration");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);
    let site_root = leptos_options.site_root.clone();

    let pool_clone = pool.clone();
    let pool_clone2 = pool.clone();

    let app = Router::new()
        // Server function handler with database context
        .route(
            "/api/*fn_name",
            post({
                let pool = pool_clone.clone();
                move |req| {
                    let pool = pool.clone();
                    handle_server_fns_with_context(
                        move || {
                            provide_context::<SqlitePool>(pool.clone());
                        },
                        req,
                    )
                }
            }),
        )
        // Static file serving for compiled WASM/JS
        .nest_service("/pkg", ServeDir::new(format!("{}/pkg", site_root)))
        // Static assets (CSS, images, etc.)
        .nest_service("/assets", ServeDir::new("public"))
        // Leptos routes with database context
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            {
                let pool = pool_clone2.clone();
                move || {
                    provide_context::<SqlitePool>(pool.clone());
                }
            },
            || view! { <App/> },
        )
        // Fallback SSR renderer
        .fallback(render_app_to_stream_with_context(
            leptos_options.clone(),
            move || {
                provide_context::<SqlitePool>(pool.clone());
            },
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
