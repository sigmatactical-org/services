#![forbid(unsafe_code)]

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    sigma_services::config::validate()?;
    sigma_theme::warp::run_service("Sigma Services", sigma_services::routes())?;
    Ok(())
}
