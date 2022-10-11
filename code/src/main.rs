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
		.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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

	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("Makayla_Miles.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(0.,false)))
		.insert(DespawnTimer(Timer::from_seconds(3.,false)));
	
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

		// This is the text that the user inputs
		commands.spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("Fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(1.0, 0.5, 0.5),
                        },
                    },
                ],
                ..Default::default()
            },
            style: Style {
                position_type: PositionType::Absolute,
				position: UiRect {
                    top: Val::Px(500.0),
                    left: Val::Px(5.0),
                    ..Default::default()
                },
                ..Default::default()
            },
			transform: Transform::from_xyz(50., 20., 3.),
            ..Default::default()
        });
		
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

/// prints every char coming in; press enter to reset the string
fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
	mut query: Query<&mut Text>,
) {
	for mut text in query.iter_mut() {
		for ev in char_evr.iter() {
			if keys.just_pressed(KeyCode::Return) {
				text.sections[0].value = "".to_string();
				string.clear();	
			} else
			if keys.just_pressed(KeyCode::Back) {
				string.pop();
				text.sections[0].value = string.to_string();
			} else {
				string.push(ev.char); 
				text.sections[0].value = string.to_string();
			}
		}
	}
	
}

