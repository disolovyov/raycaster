use quicksilver::prelude::*;
use specs::prelude::*;

use crate::assets::Assets;
use crate::entities::create_player;
use crate::resources::fps::FPS;
use crate::resources::input::Input;
use crate::resources::renderer::Renderer;
use crate::systems::fps_counter::FpsCounterSystem;
use crate::systems::minimap::MinimapSystem;
use crate::systems::player_fov::PlayerFovSystem;
use crate::systems::player_movement::PlayerInputSystem;

pub struct MainState {
    assets: Assets,
    world: World,
    logic: Dispatcher<'static, 'static>,
    render: Dispatcher<'static, 'static>,
}

impl State for MainState {
    fn new() -> Result<Self> {
        let assets = Assets::new();
        let mut world = World::new();

        let mut logic = DispatcherBuilder::new()
            .with(PlayerInputSystem, "player_input", &[])
            .build();
        logic.setup(&mut world);

        let mut render = DispatcherBuilder::new()
            .with(PlayerFovSystem, "player_fov", &[])
            .with(FpsCounterSystem, "fps_counter", &[])
            .with(MinimapSystem, "minimap", &[])
            .build();
        render.setup(&mut world);

        create_player(&mut world);

        Ok(MainState {
            assets,
            world,
            logic,
            render,
        })
    }

    fn update(&mut self, window: &mut Window) -> Result<()> {
        self.world.write_resource::<Input>().update(window);
        self.world.write_resource::<FPS>().update(window);
        self.logic.dispatch(&self.world);
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.world.write_resource::<Renderer>().clear();
        self.render.dispatch(&self.world);
        self.world
            .write_resource::<Renderer>()
            .render(window, &mut self.assets)?;
        Ok(())
    }
}
