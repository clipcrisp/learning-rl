use raylib::prelude::*;

struct Game {
    config: Config,
    gd: GameData,
    rl: RaylibHandle,
    thread: RaylibThread,
    res: Res,
}

struct Config {
    window_x: i32,
    window_y: i32,
    tile_size: i32,
    tiles_x: usize,
    tiles_y: usize,
    x_offset: i32,
    y_offset: i32,
}

struct GameData {
    entity: Entity,
    default_tile: Tile,
    grid: Vec<Vec<Tile>>,
}

struct Res {
    rl_font: Font,
}
struct Entity { 
    tile: Tile,
    pos: Vector2,
}

//TODO, make two sets of tiles, one passable, one not.
#[derive(Copy, Clone)]
struct Tile {
    symbol: char,
    color: Color,
    passable: bool,
}

impl Game {
    fn new(window_x: i32, window_y: i32, tile_size: i32) -> Game {
        let config = Config::new(window_x, window_y, tile_size); 
        
        let gd = GameData::new(
                Tile {symbol: '.', color: Color::ORANGE, passable: true },
                &config);

        let (mut rl, thread) = raylib::init()
            .size(config.window_x, config.window_y)
            .title("Generic Roguelike")
            .build();
        rl.set_target_fps(60);
       
        let res = Res::new(&mut rl, &thread);

        Game {
            config: config,
            gd: gd,
            rl: rl,
            thread: thread,
            res: res 
        }
    }
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
            y_offset: window_y % tile_size
        }
    }
}

impl GameData {
    fn new(default_tile: Tile, config: &Config) -> GameData {
        GameData {
            entity: Entity::new(config),
            default_tile: default_tile,
            grid:
            vec![vec![ default_tile; config.tiles_x]; config.tiles_y],
        }
    }
}

impl Res {
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Res {
        let font_path = String::from("res/Greybeard-16pxB.ttf");
        let rl_font = rl.load_font(&thread, &font_path)
            .expect("Couldn't load font");

        Res {
            rl_font: rl_font 
        }
    }
}

impl Entity {
    fn new(config: &Config) -> Entity {
        Entity {
            tile: Tile {
                symbol: '@',
                color: Color::GREEN,
                passable: false
            },
            pos: Vector2::new(
                (config.tiles_x / 2) as f32,
                (config.tiles_y / 2) as f32)
        }
    }
}

fn main() {
    let mut game = Game::new(1366, 768, 32);
    
    while !game.rl.window_should_close() {  
        update_game(&mut game.gd, &game.rl);
        draw_game(&game.gd, &game.config, &mut game.rl,
                  &game.thread, &game.res.rl_font);
    }
}


fn draw_game (gd: &GameData, config: &Config, rl: &mut RaylibHandle,
              thread: &RaylibThread, rl_font: &Font) {
    
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);

    draw_tiles(&mut d, &config, &gd.grid, &rl_font);
}

fn draw_tiles (d: &mut RaylibDrawHandle, config: &Config,
               tiles: &Vec<Vec<Tile>>, font: &Font) {
    
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
            d.draw_text_ex(font, &tile.symbol.to_string(),
                           cursor,
                           config.tile_size as f32,
                           0.0,
                           &tile.color);
            cursor = cursor + Vector2::new(config.tile_size as f32, 0.0);
        }
    }
}

fn update_game (gd: &mut GameData, rl: &RaylibHandle) {
    clear_grid(&mut gd.grid);
    
    player_input(gd, &rl);

    gd.grid[gd.entity.pos.y as usize][gd.entity.pos.x as usize]
        = gd.entity.tile;
}

fn player_input (gd: &mut GameData, rl: &RaylibHandle) {
    use raylib::consts::KeyboardKey::*;
    let lastkey: KeyboardKey;
    
    if rl.is_key_pressed(KEY_UP) || rl.is_key_pressed(KEY_K) {
        try_move_entity(0, -1, &mut gd.entity.pos);
    } else if rl.is_key_pressed(KEY_RIGHT) || rl.is_key_pressed(KEY_L) {
        try_move_entity(1, 0, &mut gd.entity.pos);
    } else if rl.is_key_pressed(KEY_DOWN) || rl.is_key_pressed(KEY_J) {
        try_move_entity(0, 1, &mut gd.entity.pos);
    } else if rl.is_key_pressed(KEY_LEFT) || rl.is_key_pressed(KEY_H) {
        try_move_entity(-1, 0, &mut gd.entity.pos);
    }
}

fn try_move_entity(delta_x: i32, delta_y: i32, pos: &mut Vector2) {
    pos.x = pos.x + delta_x as f32;
    pos.y = pos.y + delta_y as f32;
}

fn clear_grid(grid: &mut Vec<Vec<Tile>>) {
    for row in grid.iter_mut() {
        for tile in row.iter_mut() {
            tile.symbol = '.';
            tile.color = Color::ORANGE;
        }
    }
}
