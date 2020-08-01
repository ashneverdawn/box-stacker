
use amethyst::{
    assets::{AssetStorage, Handle, Loader, Progress, ProgressCounter},
    core::{
        geometry::Plane,
        math::{Point2, Vector2, Vector3},
        transform::{Transform, TransformBundle},
        Named, WithNamed,
    },
    derive::SystemDesc,
    ecs::{
        prelude::Entity, Entities, Join, Read, ReadExpect, ReadStorage, System, SystemData,
        WriteStorage,
    },
    input::{InputBundle, InputHandler, StringBindings},
    prelude::{Builder, World, WorldExt},
    renderer::{
        camera::{ActiveCamera, Camera},
        plugins::{RenderFlat2D, RenderToWindow},
        sprite::{SpriteRender, SpriteSheet, SpriteSheetFormat},
        types::DefaultBackend,
        ImageFormat, RenderingBundle, Texture,
    },
    ui::{RenderUi, UiBundle, UiCreator, UiFinder, UiText},
    utils::application_root_dir,
    window::ScreenDimensions,
    Application, GameData, GameDataBuilder, SimpleState, SimpleTrans, StateData, Trans,
};

#[derive(SystemDesc)]
pub struct ControlsSystem;

impl<'s> System<'s> for ControlsSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Camera>,
        ReadStorage<'s, SpriteRender>,
        ReadStorage<'s, Named>,
        WriteStorage<'s, UiText>,
        Read<'s, AssetStorage<SpriteSheet>>,
        ReadExpect<'s, ScreenDimensions>,
        Read<'s, ActiveCamera>,
        Read<'s, InputHandler<StringBindings>>,
        UiFinder<'s>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            cameras,
            sprites,
            names,
            mut ui_texts,
            sprite_sheets,
            screen_dimensions,
            active_camera,
            input,
            ui_finder,
        ): Self::SystemData,
    ) {
        // Get the mouse position if its available
        if let Some(mouse_position) = input.mouse_position() {
            // Get the active camera if it is spawned and ready
            let mut camera_join = (&cameras, &transforms).join();
            if let Some((camera, camera_transform)) = active_camera
                .entity
                .and_then(|a| camera_join.get(a, &entities))
                .or_else(|| camera_join.next())
            {
                // Project a ray from the camera to the 0z axis
                let ray = camera.projection().screen_ray(
                    Point2::new(mouse_position.0, mouse_position.1),
                    Vector2::new(screen_dimensions.width(), screen_dimensions.height()),
                    camera_transform,
                );
                let distance = ray.intersect_plane(&Plane::with_z(0.0)).unwrap();
                let mouse_world_position = ray.at_distance(distance);

                if let Some(t) = ui_finder
                    .find("mouse_position")
                    .and_then(|e| ui_texts.get_mut(e))
                {
                    t.text = format!(
                        "({:.0}, {:.0})",
                        mouse_world_position.x, mouse_world_position.y
                    );
                }

                // Find any sprites which the mouse is currently inside
                let mut found_name = None;
                for (sprite, transform, name) in (&sprites, &transforms, &names).join() {
                    let sprite_sheet = sprite_sheets.get(&sprite.sprite_sheet).unwrap();
                    let sprite = &sprite_sheet.sprites[sprite.sprite_number];
                    let (min_x, max_x, min_y, max_y) = {
                        // Sprites are centered on a coordinate, so we build out a bbox for the sprite coordinate
                        // and dimensions
                        // Notice we ignore z-axis for this example.
                        (
                            transform.translation().x - (sprite.width * 0.5),
                            transform.translation().x + (sprite.width * 0.5),
                            transform.translation().y - (sprite.height * 0.5),
                            transform.translation().y + (sprite.height * 0.5),
                        )
                    };
                    if mouse_world_position.x > min_x
                        && mouse_world_position.x < max_x
                        && mouse_world_position.y > min_y
                        && mouse_world_position.y < max_y
                    {
                        found_name = Some(&name.name);
                    }
                }

                if let Some(t) = ui_finder
                    .find("under_mouse")
                    .and_then(|e| ui_texts.get_mut(e))
                {
                    if let Some(name) = found_name {
                        t.text = format!("{}", name);
                    } else {
                        t.text = "".to_string();
                    }
                }
            }
        }
    }
}
