//! [`ServiceCard`].

/// Landing-page card for one service, borrowing from the static registry.
pub(crate) struct ServiceCard {
    pub(crate) slug: &'static str,
    pub(crate) title: &'static str,
    pub(crate) summary: &'static str,
}
