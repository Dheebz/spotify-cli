/// Summary of cached items for `spotify-cli cache status`.
#[derive(Debug, Clone)]
pub struct CacheStatus {
    pub root: String,
    pub device_count: usize,
    pub playlist_count: usize,
}
