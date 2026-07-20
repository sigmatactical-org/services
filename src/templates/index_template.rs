//! [`IndexTemplate`].

use askama::Template;
use sigma_theme::nav::SiteHeader;

use super::ServiceCard;

#[derive(Template)]
#[template(path = "index.html")]
pub(crate) struct IndexTemplate {
    pub(crate) services: &'static [ServiceCard],
    pub(crate) contact_us_url: String,
    pub(crate) site_header: SiteHeader,
    pub(crate) site_nav: String,
    pub(crate) copyright_years: String,
}
