use super::{Input, InputType};

pub trait InputHandlerModule {

    fn accepted_input_types(&self) -> Vec::<InputType>;

    fn process_input(&mut self, input: Input);
}