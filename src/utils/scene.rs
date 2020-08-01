use amethyst::{
    prelude::*,
    assets::AssetLoaderSystemData,
    core::Transform,
    renderer::rendy::mesh::{Position, Normal, Tangent, TexCoord},
    renderer::{shape::Shape, Camera, Mesh, Material, MaterialDefaults},
    renderer::light::{Light, PointLight},
    renderer::palette::rgb::Rgb,
    core::math::{Vector3, UnitQuaternion},
};

pub fn init(world: &mut World) {
    initialize_camera(world);
    initialize_floor(world);
    initialize_light(world);
}

fn initialize_camera(world: &mut World) {

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 15.0, 20.0);
    transform.set_rotation_euler(-0.5, 0.0, 0.0);

    world.create_entity()
        .with(Camera::standard_3d(1024.0, 768.0))   //window dimensions
        .with(transform)
        .build();
}

fn initialize_floor(world: &mut World) {
    let mesh = world.exec(|loader: AssetLoaderSystemData<'_, Mesh>| {
        loader.load_from_data(
            Shape::Cube
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    });

    let material_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let material = world.exec(|loader: AssetLoaderSystemData<'_, Material>| {
        loader.load_from_data(
                Material {
                    ..material_defaults
                },
                (),
            )
        },
    );

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.0);
    transform.set_scale(Vector3::new(10.0, 1.0, 10.0));

    world.create_entity()
        .with(mesh)
        .with(material)
        .with(transform)
        .build();
}

fn initialize_light(world: &mut World) {
    let light: Light = PointLight {
        intensity: 10.0,
        color: Rgb::new(1.0, 1.0, 1.0),
        ..PointLight::default()
    }.into();

    let mut transform = Transform::default();
    transform.set_translation_xyz(5.0, 5.0, 20.0);

    world.create_entity()
        .with(light)
        .with(transform)
        .build();
}