//! [`ServiceTemplate`].

#[allow(unused_imports)]
use super::*;
use askama::Template;
use sigma_theme::nav::SiteHeader;

#[derive(Template)]
#[template(path = "service.html")]
pub(crate) struct ServiceTemplate {
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) body: String,
    pub(crate) services: Vec<NavEntry>,
    pub(crate) contact_us_url: String,
    pub(crate) site_header: SiteHeader,
    pub(crate) site_nav: String,
    pub(crate) copyright_years: String,
}
