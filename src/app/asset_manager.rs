use std::collections::HashMap;

pub struct AssetManager<T> {
    assets: HashMap<String, T>,
}

impl<T> AssetManager<T> {

    pub fn new() -> AssetManager<T> {

        Self {
            assets: HashMap::new(),
        }
    }

    pub fn register(&mut self, entries: &[(String, T)])
    where
        T: Copy
    {

        entries.iter()
            .for_each(|e| { self.assets.insert(e.0.clone(), e.1); });
    }
}

pub trait Asset {}