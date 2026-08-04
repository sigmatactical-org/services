#![forbid(unsafe_code)]

fn main() -> std::io::Result<()> {
    sigma_theme::warp::run_service("Sigma Services", sigma_services::routes())
}
