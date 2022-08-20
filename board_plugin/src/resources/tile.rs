use colored::Colorize;

#[derive(PartialEq)]
pub enum Tile {
    Bomb,
    BombNeighbor(usize),
    Empty,
}

impl Tile {
    pub fn is_bomb(&self) -> bool {
        match self {
            Tile::Bomb => true,
            _ => false,
        }
    }

    pub fn to_char(&self) -> String {
        format!(
            "{}",
            match self {
                Tile::Bomb => "*".bright_red(),
                Tile::BombNeighbor(n) => match n {
                    1 => "1".cyan(),
                    2 => "2".green(),
                    3 => "3".yellow(),
                    _ => n.to_string().red(),
                },
                Tile::Empty => " ".normal(),
            }
        )
    }

    // pub fn to_char(&self) -> String {
    //     format!(
    //         "{}",
    //         match self {
    //             Tile::Bomb => "*".to_string(),
    //             Tile::BombNeighbors(n) => n.to_string(),
    //             Tile::Empty => " ".to_string(),
    //         }
    //     )
    // }
}
