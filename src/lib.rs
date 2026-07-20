//! Sigma Services: professional offerings (maintenance, consulting, R&D).

#![forbid(unsafe_code)]

mod config;
mod content;
mod templates;

use std::convert::Infallible;
use std::sync::OnceLock;

use sigma_theme::warp::TemplateError;
use warp::{Filter, Rejection, Reply};

fn index_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    static INDEX_HTML: OnceLock<String> = OnceLock::new();
    sigma_theme::warp::cached_page(&INDEX_HTML, templates::render_index_html)
}

fn service_page() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("service" / String)
        .and(warp::get())
        .and_then(|slug: String| async move {
            let Some(service) = content::get(&slug) else {
                return Err(warp::reject::not_found());
            };
            templates::render_service_html(service)
                .map(warp::reply::html)
                .map_err(|_| warp::reject::custom(TemplateError))
        })
}

/// Identity BFF origin for CSP `connect-src`, resolved once per process
/// (the theme's `security_headers` requires a `'static` borrow).
fn identity_origin() -> &'static str {
    static ORIGIN: OnceLock<String> = OnceLock::new();
    ORIGIN.get_or_init(config::identity_public_origin)
}

/// Site routes: index, `/service/{slug}`, `/up`, sigma-pg health routes,
/// theme static assets, and themed error recovery — wrapped in the shared
/// security header set.
pub fn routes()
-> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone + Send + Sync + 'static {
    sigma_theme::warp::security_headers(
        sigma_theme::warp::site_routes(
            index_page().or(service_page()),
            sigma_pg::health::warp::health_routes("services", None),
        ),
        identity_origin(),
    )
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
        assert!(body.contains("aria-label=\"Cart\""));
        // The body "Contact us" link's return_url must be an absolute URL matching
        // what the allowlist expects, not a bare path (regression: bare "/" 400s).
        assert!(body.contains(">Contact us</a>"));
        assert!(body.contains("/contact?return_url=http"));
        assert!(!body.contains("?return_url=/\""));
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
        assert!(body.contains("/contact?return_url=http"));
        assert!(!body.contains("?return_url=/service/consulting\""));
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
