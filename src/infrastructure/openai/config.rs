use gset::Getset;
use serde_derive::Deserialize;

#[derive(Clone, Deserialize, Getset)]
pub struct OpenaiConfig {
    #[getset(get, vis = "pub")]
    api_key: String,
    #[getset(get, vis = "pub")]
    base_url: String,
    #[getset(get, vis = "pub")]
    model_name: String,
}
