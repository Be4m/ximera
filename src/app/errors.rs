use super::asset_loader_module::AssetLoaderError;

#[derive(Debug)]
pub struct AppInitError;

impl From<AssetLoaderError> for AppInitError {

    // TODO
    fn from(value: AssetLoaderError) -> Self {
        
        Self {}
    }
}