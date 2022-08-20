#[derive(Clone)]
pub struct BoardOptions {
    pub width: usize,
    pub height: usize,
    pub bomb_count: usize,
}

impl Default for BoardOptions {
    fn default() -> Self {
        Self {
            width: 15,
            height: 15,
            bomb_count: 30,
        }
    }
}
