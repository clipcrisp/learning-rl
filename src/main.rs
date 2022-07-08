use bracket_lib::prelude::*;
use std::cmp::{max, min};

struct Player {
    name: String,
    pos: Point,
    render: Renderable,
}

struct Point {
    x: i32,
    y: i32,
}

struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    player: Player,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(&mut self.player.pos, ctx);
        ctx.print(1, 1, "Hellow Bracket World");
        ctx.set(
            self.player.pos.x,
            self.player.pos.y,
            self.player.render.fg,
            self.player.render.bg,
            self.player.render.glyph,
        );
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, pos: &mut Point) {
    pos.x = min(79, max(0, pos.x + delta_x));
    pos.y = min(49, max(0, pos.y + delta_y));
}

fn player_input(point: &mut Point, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, point),
            VirtualKeyCode::Right => try_move_player(1, 0, point),
            VirtualKeyCode::Up => try_move_player(0, -1, point),
            VirtualKeyCode::Down => try_move_player(0, 1, point),
            _ => {}
        },
    }
}

                
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Generic Fantasy RL")
        .build()?;

    let gs: State = State {
        player: Player {
            name: "Protag Anist".to_string(),
            pos: Point { x: 40, y: 25 },
            render: Renderable {
                glyph: to_cp437('@'),
                fg: RGB::named(YELLOW),
                bg: RGB::named(BLACK),
            },
        },
    };
    main_loop(context, gs)
}
