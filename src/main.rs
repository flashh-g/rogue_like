use crate::components::*;
use crate::map::*;
use rltk::{GameState, Rltk, RGB};
use specs::{Join, World, WorldExt};
mod components;
mod map;

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

fn main() {
    setup();
}
fn setup() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Map gen test")
        .build()?;
    let mut gs = State { ecs: World::new() };
    let (rooms, map) = new_map();
    gs.ecs.insert(map);
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    rltk::main_loop(context, gs)
}
