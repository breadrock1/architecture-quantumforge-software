use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, Getset)]
pub struct HttpServerConfig {
    #[getset(get, vis = "pub")]
    address: String,
}