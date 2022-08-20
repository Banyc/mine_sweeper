use bevy::log;
use bevy::prelude::*;

use crate::components::coordinates::Coordinates;
use crate::resources::board_assets::BoardAssets;
use crate::resources::board_options::BoardOptions;
use crate::resources::tile::Tile;
use crate::resources::tile_map::TileMap;

const TILE_SIZE: f32 = 32.0;

pub struct BoardPlugin {
    pub font_path: String,
    pub board_options: BoardOptions,
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.board_options.clone());
        app.insert_resource(FontPath(self.font_path.clone()));
        app.add_startup_system(setup_assets.before(create_board));
        app.add_startup_system(create_board);
    }
}

fn setup_assets(mut commands: Commands, asset_server: Res<AssetServer>, font_path: Res<FontPath>) {
    println!("setup board");
    commands.insert_resource(BoardAssets {
        font: asset_server.load(&font_path.0),
    });
    println!("setup board done");
}

fn create_board(
    mut commands: Commands,
    board_options: Res<BoardOptions>,
    board_assets: Res<BoardAssets>,
    _window: Option<Res<WindowDescriptor>>,
) {
    // tile map
    let tile_map = {
        let mut tile_map = TileMap::empty(board_options.width, board_options.height);
        tile_map.set_bombs(board_options.bomb_count);
        tile_map
    };
    log::info!("\n{}", tile_map.tiles_to_string());

    // board
    {
        let mut board = commands.spawn();
        board.insert(Name::new("Board"));
        board.insert(Transform::default());
        board.insert(GlobalTransform::default());
        board.insert(Visibility::default());
        board.insert(ComputedVisibility::default());
        board.add_children(|parent| {
            // background
            {
                let mut background = parent.spawn();
                background.insert(Name::new("Background"));
                background.insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::BLACK,
                        custom_size: Some(Vec2::splat(1000000.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..Default::default()
                });
            }

            // tiles
            {
                for y in 0..tile_map.height() {
                    for x in 0..tile_map.width() {
                        let coordinates = Coordinates { x, y };

                        let mut tile_entity = parent.spawn();
                        let custom_size = Vec2::splat(TILE_SIZE);
                        {
                            let transform = Transform::from_xyz(
                                x as f32 * TILE_SIZE,
                                y as f32 * TILE_SIZE,
                                1.0,
                            );
                            tile_entity.insert(Name::new(format!("Tile ({})", coordinates)));
                            tile_entity.insert_bundle(SpriteBundle {
                                sprite: Sprite {
                                    color: Color::GRAY,
                                    custom_size: Some(custom_size),
                                    ..Default::default()
                                },
                                transform,
                                ..Default::default()
                            });
                        }

                        match tile_map.tile(&coordinates) {
                            Tile::Bomb => {
                                tile_entity.add_children(|parent| {
                                    let mut bomb_entity = parent.spawn();
                                    bomb_entity.insert_bundle(SpriteBundle {
                                        sprite: Sprite {
                                            color: Color::RED,
                                            custom_size: Some(custom_size),
                                            ..Default::default()
                                        },
                                        transform: Transform::from_xyz(0., 0., 1.),
                                        ..Default::default()
                                    });
                                });
                            }
                            Tile::BombNeighbor(n) => {
                                tile_entity.add_children(|parent| {
                                    let mut bomb_neighbor_entity = parent.spawn();
                                    bomb_neighbor_entity.insert_bundle(Text2dBundle {
                                        text: Text {
                                            sections: vec![TextSection {
                                                value: n.to_string(),
                                                style: TextStyle {
                                                    font: board_assets.font.clone(),
                                                    font_size: 16.,
                                                    color: Color::BLACK,
                                                    ..Default::default()
                                                },
                                            }],
                                            alignment: TextAlignment {
                                                vertical: VerticalAlign::Center,
                                                horizontal: HorizontalAlign::Center,
                                            },
                                        },
                                        transform: Transform::from_xyz(0., 0., 1.),
                                        ..Default::default()
                                    });
                                });
                            }
                            Tile::Empty => {}
                        }
                        tile_entity.insert(coordinates);
                    }
                }
            }
        });
    }
    commands.insert_resource(tile_map);
}

struct FontPath(String);
