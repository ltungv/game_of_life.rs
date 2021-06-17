#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CellPosition(pub usize, pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellState {
    Alive,
    Dead,
}
