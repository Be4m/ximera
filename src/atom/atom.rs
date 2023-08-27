#[derive(Clone, Copy)]
pub struct Atom {
    pub atomic_radius: f32,
    pub atomic_mass: f32,
    pub num_protons: u32,
    pub num_neutrons: u32,
     
    pub name: Option<&'static str>,
    pub symbol: Option<&'static str>,
    
    //mesh: crate::scene::Mesh,
}