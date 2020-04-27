
pub struct Position { x: usize, y: usize }

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn empty() -> Self { Position::new(0, 0) }
    pub fn x(&self) -> usize { self.x }
    pub fn y(&self) -> usize { self.y }
}
