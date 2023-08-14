use super::InputType;


pub trait InputHandlerModuleClone {
    fn clone_box(&self) -> Box<dyn InputHandlerModule>;
}

impl<T> InputHandlerModuleClone for T
where
    T: InputHandlerModule + Clone + 'static
{
    fn clone_box(&self) -> Box<dyn InputHandlerModule> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn InputHandlerModule> {

    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait InputHandlerModule: InputHandlerModuleClone {

    fn accepted_input_types(&self) -> Vec::<InputType>;

    fn process_input(&mut self, input_type: InputType);
}