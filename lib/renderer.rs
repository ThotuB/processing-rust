pub trait Renderer {}

#[derive(Debug)]
pub struct P2D;

impl Renderer for P2D {}

#[derive(Debug)]
pub struct P3D;

impl Renderer for P3D {}
