pub trait Object {

    fn mesh(&self) -> &crate::renderer::Mesh;

    fn position(&self) -> &crate::scene::Position;

    fn update(&mut self) {}

    fn destroy(&mut self) {}
}