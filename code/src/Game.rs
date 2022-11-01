use bevy::{
	prelude::*,
	window::PresentMode,
};

pub struct Cube {
    pub size: f32,
}

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from(TITLE),
			width: WIN_W,
			height: WIN_H,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
       // .add_system(impl)
		.run();
}

fn setup(mut commands: Commands) {
	commands.spawn_bundle(Camera2dBundle::default());
}

fn make_shape(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>,)
{
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add()
	})
}
impl Default for Cube {
    fn default() -> Self {
        Cube { size: 1.0 }
    }
}
