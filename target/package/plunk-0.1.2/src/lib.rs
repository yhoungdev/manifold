
pub mod client;

pub use client::{PlunkClientTrait, PlunkPayloads};
pub use common_manifold::types::plunk_types::PlunkClient;

pub mod prelude {
    pub use super::PlunkClientTrait;
    pub use super::PlunkPayloads;
    pub use super::PlunkClient;
}