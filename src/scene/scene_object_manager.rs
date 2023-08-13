use super::Object;


pub struct SceneObjectManager<T: Object> {
    objects_vec: Vec::<T>,
}

impl<T: Object> SceneObjectManager<T> {

    pub fn new() -> SceneObjectManager<T> {

        Self {
            objects_vec: vec![],
        }
    }

    pub fn add_object(&mut self, object: T) {
        self.objects_vec.push(object);
    }
}