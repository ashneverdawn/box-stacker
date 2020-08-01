use amethyst::{
    prelude::*,
    utils::application_root_dir,
    GameDataBuilder, Application,
    renderer::{
        plugins::{RenderPbr3D, RenderToWindow}, //RenderSkybox},
        types::DefaultBackend,
        RenderingBundle,
    },
    core::TransformBundle,
    assets::PrefabLoaderSystemDesc,
    controls::FlyControlBundle,
    input::{StringBindings, InputBundle},
    SimpleState, StateData, GameData, input,
};
use amethyst_physics::PhysicsBundle;
use amethyst_nphysics::NPhysicsBackend;

mod systems;
mod utils;

fn main() -> amethyst::Result<()> {

    // TODO: remove warn suppression.
    amethyst::start_logger(amethyst::LoggerConfig{
        log_gfx_backend_level: Some(log::LevelFilter::Error),
        ..Default::default()
    });

    // Set up the assets directory (PathBuf)
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");
    let display_config_path = app_root.join("config/display.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    // Set up the GameDataBuilder
    let game_data = GameDataBuilder::default()
        .with_bundle(PhysicsBundle::<f32, NPhysicsBackend>::new()).unwrap()
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?)?
        .with(systems::controls::ControlsSystem, "controls", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(RenderToWindow::from_config_path(display_config_path)?
                    .with_clear([0.0, 0.0, 0.0, 1.0]), //rgba background
                )
                .with_plugin(RenderPbr3D::default())
                //.with_plugin(RenderSkybox::default()),
        )?;

    // Run the game!
    let mut game = Application::new(assets_dir, GameState, game_data)?;
    game.run();

    Ok(())
}



pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        utils::scene::init(data.world);
    }
    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if input::is_close_requested(&event) || input::is_key_down(&event, input::VirtualKeyCode::Escape) {
                return Trans::Quit
            }
        }
        Trans::None
    }
}