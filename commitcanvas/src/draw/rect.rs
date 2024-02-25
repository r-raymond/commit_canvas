use super::Line;

#[derive(Debug, Clone)]
pub struct Rect {
    pub top: Line,
    pub left: Line,
    pub right: Line,
    pub bottom: Line,
}
