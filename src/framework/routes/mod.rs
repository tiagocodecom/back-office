//! Utility functions for registering the application's entry points.
mod api;
mod web;

pub use api::api_routes;
pub use web::web_routes;
