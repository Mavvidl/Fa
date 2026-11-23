
pub mod block_alloc;
pub mod hilbert;
pub mod sierpinski;

/// Coordonnée bidimensionnelle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2D {
    pub x: u32,
    pub y: u32,
}

/// Interface commune des structures fractales.
pub trait Fractal {
    type Output;

    fn generate(&self) -> Self::Output;

    fn depth(&self) -> u32;
}

