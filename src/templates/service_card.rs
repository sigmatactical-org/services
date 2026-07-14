//! [`ServiceCard`].

#[allow(unused_imports)]
use super::*;

#[derive(Clone)]
pub(crate) struct ServiceCard {
    pub(crate) slug: String,
    pub(crate) title: String,
    pub(crate) summary: String,
}
