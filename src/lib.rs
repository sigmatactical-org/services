//! Sigma Services: professional offerings (maintenance, consulting, R&D).

mod config;
mod content;
mod templates;

use std::convert::Infallible;

use warp::Filter;
use warp::{Rejection, Reply};

pub use content::{ServiceEntry, get, sorted_entries};
pub use sigma_theme::copyright_years;

/// Resolve listen address from **`PORT`** (default **8080**).
#[must_use]
pub fn listen_socket_addr_from_env() -> std::net::SocketAddr {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port)
}

fn index_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path::end().and(warp::get()).and_then(|| async {
        templates::render_index_html()
            .map(warp::reply::html)
            .map_err(|_| warp::reject::not_found())
    })
}

fn service_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("service" / String)
        .and(warp::get())
        .and_then(|slug: String| async move {
            let Some(service) = get(&slug) else {
                return Err(warp::reject::not_found());
            };
            templates::render_service_html(service)
                .map(warp::reply::html)
                .map_err(|_| warp::reject::not_found())
        })
}

/// Site routes: index, `/service/{slug}`, `/up`, theme static assets, error recovery.
pub fn routes() -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone + Send + 'static
{
    use warp::reply::with::header;

    warp::path("up")
        .and(warp::get())
        .map(|| warp::reply::with_status("up", warp::http::StatusCode::OK))
        .or(sigma_pg::health::warp::health_routes("services", None))
        .or(index_page())
        .or(service_page())
        .or(sigma_theme::warp::static_files())
        .or(sigma_theme::warp::favicon())
        .recover(sigma_theme::warp::handle_rejection)
        .with(header(
            "content-security-policy",
            "default-src 'self'; base-uri 'self'; object-src 'none'; frame-ancestors 'none'; \
             img-src 'self' data:; style-src 'self' 'unsafe-inline'; script-src 'self'; \
             font-src 'self'; connect-src 'self'; form-action 'self'",
        ))
        .with(header("x-content-type-options", "nosniff"))
        .with(header("x-frame-options", "DENY"))
        .with(header("referrer-policy", "strict-origin-when-cross-origin"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::http::StatusCode;

    #[tokio::test]
    async fn index_lists_services() {
        let res = warp::test::request()
            .method("GET")
            .path("/")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 200);
        let body = std::str::from_utf8(res.body()).unwrap();
        assert!(body.contains("Vehicle maintenance"));
        assert!(body.contains("Consulting"));
    }

    #[tokio::test]
    async fn service_page_renders() {
        let res = warp::test::request()
            .method("GET")
            .path("/service/consulting")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), 200);
        let body = std::str::from_utf8(res.body()).unwrap();
        assert!(body.contains("Consulting"));
    }

    #[tokio::test]
    async fn unknown_service_is_404() {
        let res = warp::test::request()
            .method("GET")
            .path("/service/no-such-offering")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn up_returns_ok() {
        let res = warp::test::request()
            .method("GET")
            .path("/up")
            .reply(&routes())
            .await;
        assert_eq!(res.status(), StatusCode::OK);
    }
}
