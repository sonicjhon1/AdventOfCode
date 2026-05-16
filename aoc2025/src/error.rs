use tracing::warn;

pub type Result<T = (), E = Box<dyn std::error::Error>> = core::result::Result<T, E>;

#[allow(clippy::borrowed_box)]
pub fn warn_handler(error: &Box<dyn std::error::Error>) { warn!("{error}") }
