#[derive(Clone)]
pub struct Atom {
    atomic_radius: f32,
    atomic_mass: f32,
    num_protons: u32,
    num_neutrons: u32,
    
    name: Option<&'static str>,
    symbol: Option<&'static str>,
    
    //mesh: crate::scene::Mesh,
}