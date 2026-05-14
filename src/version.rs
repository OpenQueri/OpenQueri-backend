
pub struct VersionServer;

impl VersionServer {
    pub const VERSION_SERVER: &'static str = concat!("v ", env!("CARGO_PKG_VERSION"));
}