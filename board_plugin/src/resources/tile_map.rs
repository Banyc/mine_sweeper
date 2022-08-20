use rand::Rng;

use crate::{components::coordinates::Coordinates, resources::tile::Tile};

pub struct TileMap {
    bomb_count: usize,
    height: usize,
    width: usize,
    tiles: Vec<Tile>,
}

impl TileMap {
    pub fn check_rep(&self) {
        // bomb count
        {
            let mut bomb_count = 0;
            for tile in &self.tiles {
                if tile.is_bomb() {
                    bomb_count += 1;
                }
            }
            assert_eq!(bomb_count, self.bomb_count);
        }

        // bomb neighbors
        for x in 0..self.width {
            for y in 0..self.height {
                let coordinates = Coordinates { x, y };
                let tile = self.tile(&coordinates);
                if tile.is_bomb() {
                    continue;
                }
                let bomb_count_around = self.bomb_count_around(&coordinates);
                match tile {
                    Tile::Empty => assert_eq!(bomb_count_around, 0),
                    Tile::BombNeighbor(n) => {
                        assert!(*n > 0);
                        assert_eq!(*n, bomb_count_around);
                    }
                    _ => panic!(),
                }
            }
        }
    }

    #[must_use]
    pub fn empty(width: usize, height: usize) -> Self {
        let mut tiles = Vec::with_capacity(width * height);
        for _ in 0..width * height {
            tiles.push(Tile::Empty);
        }
        let this = Self {
            bomb_count: 0,
            height,
            width,
            tiles,
        };
        this.check_rep();
        this
    }

    pub fn set_bombs(&mut self, bomb_count: usize) {
        let mut rng = rand::thread_rng();
        let mut bomb_count_left = bomb_count;
        while bomb_count_left > 0 {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            let coordinates = Coordinates { x, y };
            if !self.tile(&coordinates).is_bomb() {
                self.set_tile(&coordinates, Tile::Bomb);
                bomb_count_left -= 1;
            }
        }
        self.bomb_count += bomb_count;

        // update bomb neighbors
        for x in 0..self.width {
            for y in 0..self.height {
                let coordinates = Coordinates { x, y };
                if self.tile(&coordinates).is_bomb() {
                    continue;
                }
                let bomb_count_around = self.bomb_count_around(&coordinates);
                if bomb_count_around > 0 {
                    self.set_tile(&coordinates, Tile::BombNeighbor(bomb_count_around));
                }
            }
        }

        self.check_rep();
    }

    #[must_use]
    pub fn tile(&self, coordinates: &Coordinates) -> &Tile {
        &self.tiles[coordinates.y * self.width + coordinates.x]
    }

    fn set_tile(&mut self, coordinates: &Coordinates, tile: Tile) {
        self.tiles[coordinates.y * self.width + coordinates.x] = tile;
    }

    #[must_use]
    pub fn tiles_to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&self.tile(&Coordinates { x, y }).to_char());
            }
            s.push_str("\n");
        }
        s
    }

    #[must_use]
    pub fn width(&self) -> usize {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> usize {
        self.height
    }

    #[must_use]
    pub fn bomb_count(&self) -> usize {
        self.bomb_count
    }

    #[must_use]
    pub fn safe_tiles_around(&self, coordinates: &Coordinates) -> Vec<Coordinates> {
        let mut safe_tiles = Vec::new();
        for y in (coordinates.y - 1)..=(coordinates.y + 1) {
            for x in (coordinates.x - 1)..=(coordinates.x + 1) {
                if y < self.height && x < self.width {
                    if !self.tile(&Coordinates { x, y }).is_bomb() {
                        safe_tiles.push(Coordinates { x, y });
                    }
                }
            }
        }
        safe_tiles
    }

    #[must_use]
    pub fn bomb_count_around(&self, coordinates: &Coordinates) -> usize {
        let mut bomb_count = 0;
        for y in (coordinates.y as i32 - 1)..=(coordinates.y as i32 + 1) {
            for x in (coordinates.x as i32 - 1)..=(coordinates.x as i32 + 1) {
                if y < self.height as i32 && x < self.width as i32 && y >= 0 && x >= 0 {
                    let x = x as usize;
                    let y = y as usize;
                    if self.tile(&Coordinates { x, y }).is_bomb() {
                        bomb_count += 1;
                    }
                }
            }
        }
        bomb_count
    }
}
