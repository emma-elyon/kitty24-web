use bevy::{prelude::*, render::render_resource::{Extent3d, TextureDimension, TextureFormat}};

use assembler::Assembler;
use virtual_machine::VirtualMachine;

#[derive(Resource)]
struct Emulator {
    vm: VirtualMachine,
}

#[derive(Resource)]
struct Video(Handle<Image>);

fn main() {
    let assembly = include_str!("boot.kittyasm");

    let vm = match Assembler::assemble(assembly) {
        Ok(rom) => VirtualMachine::new(rom),
        Err(error) => panic!("{}", error),
    };

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, frame_update)
        .add_systems(FixedUpdate, fixed_update)
        .insert_resource(Time::<Fixed>::from_seconds(1.0 / 60.0))
        .insert_resource(Emulator { vm })
        .run();
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    eprintln!("SETUP");
    let image = Image::new(
        Extent3d { width: 320, height: 180, depth_or_array_layers: 1 },
        TextureDimension::D2,
        vec![0; 320 * 180 * 4],
        TextureFormat::Rgba8Unorm,
    );

    let image_handle = images.add(image);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: image_handle.clone(),
        ..default()
    });
    commands.insert_resource(Video(image_handle));
}

fn frame_update(
    _emulator: Res<Emulator>
) {
}

fn fixed_update(
    mut images: ResMut<Assets<Image>>,
    video: Res<Video>,
    mut emulator: ResMut<Emulator>
) {
    emulator.vm.run();
    if let Some(image) = images.get_mut(video.0.clone()) {
        image.data = emulator.vm.video.clone();
    }
}
