use crate::app::{AssetLoaderModule, AssetLoaderError};
use crate::app::AssetManager;

use crate::atom::Atom;

pub struct PeriodicTableAssetLoaderModule {}

impl PeriodicTableAssetLoaderModule {

    pub fn new(asset_manager: &mut AssetManager::<Atom>) -> Self {

        Self {}
    }
}

impl AssetLoaderModule for PeriodicTableAssetLoaderModule {
    type Item = Atom;

    fn load_all(&self) -> Result<(), AssetLoaderError> {
        
        todo!()
    }

    fn retrieve_asset_manager(&self) -> Result<AssetManager::<Self::Item>, AssetLoaderError> {
        
        todo!()
    }
}