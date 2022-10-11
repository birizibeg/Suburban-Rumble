use bevy::{
	prelude::*,
	text::Text2dBounds,
};

#[derive(Component)]
pub struct Hero;
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct DialogueBox;
enum ConversationState {
    Introduction,
    Conversation,
    GoodEnding,
	BadEnding
}


pub fn setup_conversation(
	mut commands: Commands,
	mut clear_color: ResMut<ClearColor>, 
	asset_server: Res<AssetServer>
){
    clear_color.0 = Color::DARK_GREEN;
	let font = asset_server.load("SourceSansPro-Regular.ttf");
	let text_style = TextStyle {
		font,
        font_size: 60.0,
        color: Color::WHITE
    };

    commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("hero.png"),
		transform: Transform::from_xyz(-200., 0., 2.),
		sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Hero);
	commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("enemy.png"),
		transform: Transform::from_xyz(200., 100., 2.),
		sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Enemy);
	let box_size = Vec2::new(300.0, 200.0);
    let box_position = Vec2::new(0.0, -250.0);
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::DARK_GRAY,
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_translation(box_position.extend(0.0)),
        ..default()
    });
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Dialogue Box", text_style),
        text_2d_bounds: Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x - box_size.x / 2.0,
            box_position.y + box_size.y / 2.0,
            1.0,
        ),
        ..default()
    }).insert(DialogueBox);
	info!("Setting Up: GameState: Conversation");
}
pub fn clear_conversation(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut hero: Query<Entity, With<Hero>>,
	mut enemy: Query<Entity, With<Enemy>>,

) {
    clear_color.0 = Color::BLACK;

    let hero_eid = hero.single_mut();
	let enemy_eid = enemy.single_mut();
    commands.entity(hero_eid).despawn();
	commands.entity(enemy_eid).despawn();

}

/// prints every char coming in; press enter to echo the full string
pub fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
	mut dialogue: Query<(&mut Text), With<DialogueBox>>
) {
	let mut dialogue_text = dialogue.single_mut();

	for ev in char_evr.iter() {

		if keys.just_pressed(KeyCode::Return) {
			println!("Text input: {}", *string);
			dialogue_text.sections[0].value = string.to_string();
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
