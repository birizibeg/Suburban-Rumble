use bevy::{
	prelude::*,
	text::Text2dBounds,
};
use super::ConvInputEvent;
use super::ConvLossEvent;
use super::ConvWinEvent;

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct DialogueBox;

#[derive(Component)]
pub struct UserInput;

#[derive(Component)]
pub struct Button;

enum ConversationState {
    Introduction,
    Conversation,
    GoodEnding,
	BadEnding
}


// Spawn all entities to be used in the conversation part of the game
pub fn setup_conversation(
	mut commands: Commands,
	mut clear_color: ResMut<ClearColor>, 
	asset_server: Res<AssetServer>,
){
    clear_color.0 = Color::DARK_GREEN;
    let user_text_style = TextStyle {
		font: asset_server.load("Fonts/SourceSansPro-Regular.ttf"),
        font_size: 40.0,
        color: Color::WHITE
    };
    let enemy_text_style = TextStyle {
		font: asset_server.load("Fonts/SourceSansPro-Regular.ttf"),
        font_size: 60.0,
        color: Color::BLACK
    };

    commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("hero.png"),
		transform: Transform::from_xyz(-500., -225., 2.),
		sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Hero);

	commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("enemy.png"),
		transform: Transform::from_xyz(500., 200., 2.),
		sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Enemy);

	let box_size = Vec2::new(700.0, 200.0);
    let box_position = Vec2::new(-45.0, -250.0);
    let box_position_two = Vec2::new(45.0, 175.0);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::Rgba{red: 0.0, green: 0.0, blue: 0.0, alpha: 0.5},
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_translation(box_position.extend(0.5)),
        ..default()
    }).insert(DialogueBox);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::Rgba{red: 255.0, green: 255.0, blue: 255.0, alpha: 0.5},
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_translation(box_position_two.extend(0.0)),
        ..default()
    }).insert(DialogueBox);

    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Excuse me neighbor, can I borrow some sugar?", enemy_text_style),
        text_2d_bounds: Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position_two.x - box_size.x / 2.0,
            box_position_two.y + box_size.y / 2.0,
            1.0,
        ),
        ..default()
    }).insert(DialogueBox);
    
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Press \'n\' to say: \"Sure!\" (this option makes your neighbor happy)\nPress \'m\' to say: \"No! You stink!\" (be careful: this option makes your neighbor mad!", user_text_style),
        text_2d_bounds: Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x - box_size.x / 2.0,
            box_position.y + box_size.y / 2.0,
            1.0,
        ),
        ..default()
    }).insert(DialogueBox)
    .insert(UserInput);
	//info!("Setting Up: GameState: Conversation");
}

// Despawns every entity used in the conversation state that is not also in fight or credits
pub fn clear_conversation(
    mut commands: Commands,
    mut hero: Query<Entity, With<Hero>>,
	mut enemy: Query<Entity, With<Enemy>>,
    dialogue: Query<Entity, With<DialogueBox>>,

) {
    for entity in dialogue.iter() {
        commands.entity(entity).despawn();
    }
    let hero_eid = hero.single_mut();
	let enemy_eid = enemy.single_mut();
    commands.entity(hero_eid).despawn();
	commands.entity(enemy_eid).despawn();
}

// This takes the user's input and then prints every character onto the window using a text box
pub fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
	  mut dialogue: Query<&mut Text, With<UserInput>>,
    mut ev_writer: EventWriter<ConvInputEvent>
) {
	let mut dialogue_text = dialogue.single_mut();

	for ev in char_evr.iter() {

		if keys.just_pressed(KeyCode::Return) {
            let entered_string = string.to_string();
            ev_writer.send(ConvInputEvent(entered_string));
			string.clear();	
            dialogue_text.sections[0].value = "".to_string();
		} else
		if keys.just_pressed(KeyCode::Back) {
			string.pop();
			dialogue_text.sections[0].value = string.to_string();
		}
        if keys.just_pressed(KeyCode::N) {
			string.pop();
			dialogue_text.sections[0].value = string.to_string();
		}
        else {
			string.push(ev.char); 
			dialogue_text.sections[0].value = string.to_string();
		}
	}
}

// Processes the input that the user gives
// For now, just a few key phrases are checked to be contained in the user's response
// This will be where the AI part is implemented
pub fn process_input(
    mut ev_reader: EventReader<ConvInputEvent>,
    mut loss_writer: EventWriter<ConvLossEvent>,
    mut win_writer: EventWriter<ConvWinEvent>
) {
    for input in ev_reader.iter() {
        let mut string = input.0.to_string();
        string.make_ascii_lowercase();
        string = string.trim_end().to_string();
        if string.contains("get lost") || string.contains("no") || string.contains("you're stinky") {
            loss_writer.send(ConvLossEvent()); // Trigger loss
        } else if string.contains("sure") || string.contains("absolutely") || string.contains("yes") {
            win_writer.send(ConvWinEvent()); // Trigger win
        }
    }
}
