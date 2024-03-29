mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

pub mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
}

use prelude::*;

struct State {
    world: World,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    resources: Resources,
}

impl State {
    fn new() -> Self {
        let (world, resources) = Self::build();

        Self {
            world,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(9, GREEN, BLACK, "Press any key to play again.");
        if let Some(_) = ctx.key {
            // clippy suggest "if Some(ctx.key).is_some()" but it is always true?
            let (world, resources) = Self::build();
            self.world = world;
            self.resources = resources;
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of YALA and feel its power course through your veins.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");
        if let Some(VirtualKeyCode::Key1) = ctx.key {
            // clippy suggest "if Some(ctx.key).is_some()" but it is always true?
            let (world, resources) = Self::build();
            self.world = world;
            self.resources = resources;
        }
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&mut self.world)
            .nth(0)
            .unwrap();
        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);

        <(Entity, &Carried)>::query()
            .iter(&self.world)
            .filter(|(_e, item)| item.0 == player_entity)
            .map(|(e, _)| *e)
            .for_each(|e| {
                entities_to_keep.insert(e);
            });

        let mut cb = CommandBuffer::new(&self.world);
        for e in Entity::query().iter(&self.world) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.world);

        <&mut FieldOfView>::query()
            .iter_mut(&mut self.world)
            .for_each(|fov| fov.is_dirty = true);

        let mut rng = RandomNumberGenerator::new();
        let mut mb = MapBuilder::build(&mut rng);

        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.world)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = mb.player_start.x;
                pos.y = mb.player_start.y;
            });

        if map_level == 2 {
            spawn_amulet(&mut self.world, mb.amulet_start);
        } else {
            let exit_idx = mb.map.point2d_to_index(mb.amulet_start);
            mb.map.tiles[exit_idx] = TileType::Exit;
            mb.monster_spawns
                .iter()
                .for_each(|pos| spawn_entity(&mut self.world, &mut rng, *pos));

            self.resources.insert(mb.map);
            self.resources.insert(Camera::new(mb.player_start));
            self.resources.insert(TurnState::AwaitingInput);
            self.resources.insert(mb.theme);
        }
    }

    fn build() -> (World, Resources) {
        let mut world = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut mb = MapBuilder::build(&mut rng);
        spawn_player(&mut world, mb.player_start);
        // spawn_amulet(&mut world, mb.amulet_start);
        let exit_idx = mb.map.point2d_to_index(mb.amulet_start);
        mb.map.tiles[exit_idx] = TileType::Exit;
        mb.monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut world, &mut rng, *pos));

        resources.insert(mb.map);
        resources.insert(Camera::new(mb.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(mb.theme);
        (world, resources)
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();

        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = *self.resources.get::<TurnState>().unwrap();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.world, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_systems
                .execute(&mut self.world, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.world, &mut self.resources),
            TurnState::NextLevel => {
                self.advance_level();
            }
            TurnState::GameOver => {
                self.game_over(ctx);
            }
            TurnState::Victory => {
                self.victory(ctx);
            }
        }
        render_draw_buffer(ctx).expect("Rendering error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon of Doom")
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH * 2, SCREEN_HEIGHT * 2, "terminal8x8.png")
        .build()?;
    main_loop(context, State::new())
}
