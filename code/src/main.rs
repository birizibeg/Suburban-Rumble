use bevy::{
	prelude::*,
	window::PresentMode,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Credits,
    Conversation,
    Fight,
}

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct DespawnTimer(Timer);

fn main() {
	App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("Suburban Rumble"),
			width: 1280.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
		.add_state(GameState::Credits)	//start the game in the credits state
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(text_input)
		.add_system_set(
			SystemSet::on_update(GameState::Credits)
				.label("credits")
				.with_system(show_popup)
				.with_system(remove_popup)
		)
		.add_system_set(
			SystemSet::on_enter(GameState::Credits)
				.with_system(setup_credits)
		)
		.add_system_set(
			SystemSet::on_exit(GameState::Credits)
				.with_system(clear_credits)	// remove the popups on screen when exiting the credit state
		)
		.add_system(change_gamestate)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(Camera2dBundle::default());
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
}

fn setup_credits(mut commands: Commands, asset_server: Res<AssetServer>) {
	//commands.spawn_bundle(Camera2dBundle::default());
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
	info!("GameState: Credits");
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

fn clear_credits(
	mut popup: Query<&mut Visibility>
) {
	for mut vis_map in popup.iter_mut() {
		vis_map.is_visible = false;
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

// changes the current gamestate on keypress
fn change_gamestate(
	keys: Res<Input<KeyCode>>,
	mut game_state: ResMut<State<GameState>>
) {
	if keys.pressed(KeyCode::Key1) {	// change GameState to Conversation
		match game_state.set(GameState::Conversation) {
			Ok(_) => info!("GameState: Conversation"),
			Err(_) => (),
		}
	}
	else if keys.pressed(KeyCode::Key2) {
		match game_state.set(GameState::Fight){
			Ok(_) => info!("GameState: Fight"),
			Err(_) => (),
		}
	}
	else if keys.pressed(KeyCode::Key3) {
		match game_state.set(GameState::Credits) {
			Ok(_) => info!("GameState: Credits"),
			Err(_) => (),
		}
	}
}