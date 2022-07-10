use raylib::prelude::*;

struct Config {
    window_x: i32,
    window_y: i32,
    tile_size: i32,
    tiles_x: usize,
    tiles_y: usize,
    x_offset: i32,
    y_offset: i32,
    font_path: String,
}

struct GameData {
    player: Player,
    default_tile: char,
    grid: Vec<Vec<char>>,
}

struct Player {
    symbol: char,
    pos: Vector2,
}

impl Config {
    fn new(window_x: i32, window_y: i32, tile_size: i32) -> Config {
        Config {
            window_x: window_x,
            window_y: window_y,
            tile_size: tile_size,
            tiles_x: (window_x / tile_size) as usize,
            tiles_y: (window_y / tile_size) as usize,
            x_offset: window_x % tile_size,
            y_offset: window_y % tile_size,
            font_path: String::from("res/Greybeard-16pxB.ttf")
        }
    }
}

impl GameData {
    fn new(default_tile: char, config: &Config) -> GameData {
        GameData {
            player: Player::new(config),
            default_tile: default_tile,
            grid:
            vec![vec![ default_tile; config.tiles_x]; config.tiles_y],
        }
    }
}

impl Player {
    fn new(config: &Config) -> Player {
        Player {
            symbol: '@',
            pos: Vector2::new(
                (config.tiles_x / 2) as f32,
                (config.tiles_y / 2) as f32)
        }
    }
}

fn main() {
    let config = Config::new(1366, 768, 16);
    let mut gd = GameData::new('.', &config);

    let (mut rl, thread) = raylib::init()
        .size(config.window_x, config.window_y)
        .title("Generic Roguelike")
        .build();
    rl.set_target_fps(60);
    let rl_font = rl
        .load_font(&thread, &config.font_path)
        .expect("Couldn't load font");

    while !rl.window_should_close() {
        update_game(&mut gd, &rl);
        draw_game(&gd, &config, &mut rl, &thread, &rl_font);
    }
}

fn draw_game (gd: &GameData, config: &Config, rl: &mut RaylibHandle,
              thread: &RaylibThread, rl_font: &Font) {
    
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    draw_tiles(&mut d, &config, &gd.grid, &rl_font);
}

fn draw_tiles (d: &mut RaylibDrawHandle, config: &Config,
               tiles: &Vec<Vec<char>>, font: &Font) {
    
    let mut cursor: Vector2 =
        Vector2::new((0 + config.x_offset) as f32,
                     (0 + config.y_offset) as f32);
    
    let mut first_row = true;
    for row in tiles.iter() {
        if first_row == true { first_row = false; }
        else {
            cursor = cursor + Vector2::new(0.0, config.tile_size as f32); 
            cursor.x = (0 + config.x_offset) as f32;
        }
        
        for tile in row.iter() {
            d.draw_text_ex(font, &tile.to_string(),
                           cursor,
                           config.tile_size as f32,
                           0.0,
                           Color::ORANGE);
            cursor = cursor + Vector2::new(config.tile_size as f32, 0.0);
        }
    }
}

fn update_game (gd: &mut GameData, rl: &RaylibHandle) {
    clear_grid(&mut gd.grid);
    
    player_input(gd, &rl);

    gd.grid[gd.player.pos.y as usize][gd.player.pos.x as usize]
        = gd.player.symbol;
}

fn player_input (gd: &mut GameData, rl: &RaylibHandle) {
    use raylib::consts::KeyboardKey::*;
    let lastkey: KeyboardKey;
    
    if rl.is_key_down(KEY_UP) || rl.is_key_down(KEY_K) {
        try_move_player(0, -1, &mut gd.player.pos);
    } else if rl.is_key_down(KEY_RIGHT) || rl.is_key_down(KEY_L) {
        try_move_player(1, 0, &mut gd.player.pos);
    } else if rl.is_key_down(KEY_DOWN) || rl.is_key_down(KEY_J) {
        try_move_player(0, 1, &mut gd.player.pos);
    } else if rl.is_key_down(KEY_LEFT) || rl.is_key_down(KEY_H) {
        try_move_player(-1, 0, &mut gd.player.pos);
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, pos: &mut Vector2) {
    pos.x = pos.x + delta_x as f32;
    pos.y = pos.y + delta_y as f32;
}

fn clear_grid(grid: &mut Vec<Vec<char>>) {
    for row in grid.iter_mut() {
        for tile in row.iter_mut() {
            *tile = '.';
        }
    }
}
