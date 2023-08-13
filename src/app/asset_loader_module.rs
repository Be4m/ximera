use std::fmt;

use super::AssetManager;

// TODO: Implement Error
#[derive(Debug, Clone)]
pub struct AssetLoaderError;

impl fmt::Display for AssetLoaderError {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "couldn't not finnish loading assets")
    }
}

pub trait AssetLoaderModule {
    type Item;

    fn load_all(&self) -> Result<(), AssetLoaderError>;

    fn retrieve_asset_manager(&self) -> Result<AssetManager::<Self::Item>, AssetLoaderError>;
}