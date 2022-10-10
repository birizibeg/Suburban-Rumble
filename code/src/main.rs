use bevy::{
	prelude::*,
	window::PresentMode,
};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct DespawnTimer(Timer);

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("The End!"),
			width: 1280.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(ClearColor(Color::rgb(255.0, 255.0, 255.0)))
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(text_input)
		.add_system(show_popup)
		.add_system(remove_popup)
		//.add_system(trans_sprite)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(Camera2dBundle::default());
	//commands
	//	.spawn_bundle(SpriteBundle {
	//		texture: asset_server.load("hello_world_win.png"),
	//		..default()
	//	});
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Makayla_Miles.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		//.insert(PopupTimer(Timer::from_seconds(5.0, false)));
		.insert(PopupTimer(Timer::from_seconds(0.,false)))
		.insert(DespawnTimer(Timer::from_seconds(3.,false)));
		//.insert(Timer::new(5., false));
			
		//commands.entity(texture).despawn();
//fn setup2(mut commands: Commands, asset_server: Res<AssetServer>) {
	//commands.spawn_bundle(Camera2dBundle::default()); 	
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("adamsheelar.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(3., false)))
		.insert(DespawnTimer(Timer::from_seconds(6.,false)));

	
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("colinferlan.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(6., false)))
		.insert(DespawnTimer(Timer::from_seconds(9.,false)));
	
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("BoazJoseph.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(9., false)))
		.insert(DespawnTimer(Timer::from_seconds(12.,false)));
	
	
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("AlexChlpka.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(12., false)))
		.insert(DespawnTimer(Timer::from_seconds(15.,false)));

		commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Birizibe Gnassingbe.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(15., false)))
		.insert(DespawnTimer(Timer::from_seconds(18.,false)));

		commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("emilykyle.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(18., false)))
		.insert(DespawnTimer(Timer::from_seconds(21.,false)));

		commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("VibhuCreditsF.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(21., false)))
		.insert(DespawnTimer(Timer::from_seconds(24.,false)));
		
	info!("The End!");
}



fn show_popup(
	time: Res<Time>,
	mut popup: Query<(&mut PopupTimer, &mut Transform)>
) {
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			transform.translation.z = 2.;		
		}
	}
}

fn remove_popup(
	time: Res<Time>,
	mut rmpopup: Query<(&mut DespawnTimer, &mut Visibility)>
) {
	for (mut timer, mut vis_map) in rmpopup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			vis_map.is_visible = false;
		}
	}
}

/// prints every char coming in; press enter to echo the full string
fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
) {
	for ev in char_evr.iter() {
		if keys.just_pressed(KeyCode::Return) {
			println!("Text input: {}", *string);
			string.clear();	
		} else
		if keys.just_pressed(KeyCode::Back) {
			string.pop();
			println!("Text input: {}", *string);
		} else {
			string.push(ev.char); 
			println!("Text input: '{}'", *string);
		}
	}
}

