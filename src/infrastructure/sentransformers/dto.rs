use derive_builder::Builder;
use serde_derive::Serialize;

#[derive(Builder, Debug, Serialize)]
pub struct Request {
    inputs: Vec<String>
}
