use bevy::{
	prelude::*,
	text::Text2dBounds,
};
mod AFFINParser;

use super::ConvInputEvent;
use super::ConvLossEvent;
use super::ConvWinEvent;
extern crate rust_stemmers;
use AFFINParser::SentimentScore; 
use rust_stemmers::{Algorithm, Stemmer};

#[derive(Component)]
pub struct Hero;

#[derive(Component)]
pub struct Background;
//#[derive(Component)]
//pub struct Enemy;

#[derive(Component)]
pub struct DialogueBox;

#[derive(Component)]
pub struct UserInput;

#[derive(Component)]
pub struct EnemyDialogue;

#[derive(Component)]
pub struct Button;

// stats struct to track tolerance for enemies
#[derive(Component)]
pub struct Enemy{
	start_tolerance: i8,
    name: String,
    age: i8,
    job: String,
    description: String,
}

const NICE_RESPONSES: [&'static str;6] = ["Thank you!", "I really appreciate that!",
"You're such a good neighbor!", "You're a life saver", "Thanks! I'll see you later.", "Have a good day!"];

const MEAN_RESPONSES: [&'static str;6] = ["Why would you say that to me?", "You're a crazy person!!",
"I will literally call the police.", "Do you want to fight?!?!???!", "You're the worst neighbor EVER!", "You don't want to take it there!"];

const NICE_GREETINGS: [&'static str;6] = ["Hello!", "How are you?", "I hope your day is going good so far!", 
"How is your day going?", "Long time, no see! How are you?", "How's it going?"];

const MEAN_GREETINGS: [&'static str;6] = ["What is WRONG with you?", "Don't smile at me! You KNOW what you did.", "I can not stand you!", 
"You're actually the worst neighbor ever!", "Why do you act like this?", "You're ruining my day!!"];
struct Word(String, i8);

// 0 - start (enemy prompt, wait for player prompt)
// 1 - after player first response, fetch ai response
// 2 - after player second response, fetch ai response
// etc.. 
// FINAL TURN - after player final response, return fight or not
const MAX_TURNS: i32 = 4;
const START_TURN: i32 = 0;
static mut CUR_TURN: i32 = 0;
static mut WORDS: Vec<Word> = Vec::new();

// Spawn all entities to be used in the conversation part of the game
pub fn setup_conversation(
	mut commands: Commands,
	mut clear_color: ResMut<ClearColor>, 
	asset_server: Res<AssetServer>,
){
    unsafe {
        CUR_TURN = START_TURN;
        println!("Current Turn: {}", CUR_TURN);
        WORDS.push(Word("awesom".to_string(), 10));
        WORDS.push(Word("good".to_string(), 10));
        WORDS.push(Word("pretty".to_string(), 10));
        WORDS.push(Word("yes".to_string(), 10));
        WORDS.push(Word("yeah".to_string(), 10));
        WORDS.push(Word("no".to_string(), -10));
        WORDS.push(Word("stinki".to_string(), -10));
    }

    clear_color.0 = Color::NONE;
    let user_text_style = TextStyle {
		font: asset_server.load("Fonts/Minecraft.ttf"),
        font_size: 40.0,
        color: Color::WHITE
    };
    let enemy_text_style = TextStyle {
		font: asset_server.load("Fonts/Minecraft.ttf"),
        font_size: 40.0,
        color: Color::BLACK
    };

    commands .spawn_bundle(SpriteBundle { 
        texture: asset_server.load("conversationscreen.png"), 
        ..default() 
    }).insert(Background);
    
    commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("hero.png"),
		transform: Transform::from_xyz(0., 0., 1.),
		sprite: Sprite {
            //color: Color::WHITE,
            //custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Hero);

	commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("CathyRobinson.png"),
		transform: Transform::from_xyz(0., 0., 1.),
		sprite: Sprite {
            //color: Color::WHITE,
            //custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Enemy{start_tolerance: 100, name: String::from("Catherine Robinson"), age: 27, job: String::from("Teacher"), description: String::from("nice")});

    /*commands.spawn_bundle(SpriteBundle {
		texture: asset_server.load("Billy Wickler.png"),
		transform: Transform::from_xyz(0., 0., 2.),
		sprite: Sprite {
            //color: Color::WHITE,
            //custom_size: Some(Vec2::new(200., 200.)),
            ..default()
        },
		..default()
	}).insert(Enemy{start_tolerance: 50, name: String::from("Billy Wickler"), age: 49, job: String::from("Teacher"), description: String::from("nice")});*/

	let box_size = Vec2::new(700.0, 200.0);
    let box_position = Vec2::new(-45.0, -250.0);
    let box_position_two = Vec2::new(45.0, 175.0);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::Rgba{red: 0.0, green: 0.0, blue: 0.0, alpha:0.75},
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 2.).with_translation(box_position.extend(0.5)).with_scale(Vec3::splat(1.1)),
        ..default()
    }).insert(DialogueBox);

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::Rgba{red: 255.0, green: 255.0, blue: 255.0, alpha: 0.5},
            custom_size: Some(Vec2::new(box_size.x, box_size.y)),
            ..default()
        },
        transform: Transform::from_translation(box_position_two.extend(0.5)).with_scale(Vec3::splat(1.25)),
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
    }).insert(DialogueBox)
    .insert(EnemyDialogue);
    
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("Type your response", user_text_style),
        text_2d_bounds: Text2dBounds {
            size: box_size,
        },
        transform: Transform::from_xyz(
            box_position.x - box_size.x / 2.0,
            box_position.y + box_size.y / 2.0,
            2.0,
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
    mut background: Query<Entity, With<Background>>,
    dialogue: Query<Entity, With<DialogueBox>>,

) {
    for entity in dialogue.iter() {
        commands.entity(entity).despawn();
    }
    let hero_eid = hero.single_mut();
	let enemy_eid = enemy.single_mut();
    let background_eid = background.single_mut();
    commands.entity(hero_eid).despawn();
	commands.entity(enemy_eid).despawn();
    commands.entity(background_eid).despawn();
    unsafe{
        WORDS = Vec::new();
    }
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
		} else {
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
    mut win_writer: EventWriter<ConvWinEvent>,
    mut enemy_dialogue: Query<&mut Text, With<EnemyDialogue>>,
    mut enemy: Query<&mut Enemy>,
    //mut tolerances: Query<Enemy>>,
) {
    let mut score = 0;
    let mut multiplier = 1;
    let mut enemy = enemy.single_mut();
    let stemmer = Stemmer::create(Algorithm::English);
    let mut simple_sentence: Vec<String> = Vec::new();
    let mut enem_dlg = enemy_dialogue.single_mut();
    let mut player_sent = true;

    for input in ev_reader.iter() {
        // Get the input and do some string manipulation to make it easier to parse
        let mut string = input.0.to_string();
        string.make_ascii_lowercase();
        string = string.trim_end().to_string();
        unsafe {
            // Create a simple sentence by iterating through the words and pushing them to a vec
            for words in string.split_whitespace(){
                // If the word is not an article,
                let word = words.trim_end_matches(","); // Trim off any potential commas
                if word.to_string() != "a" && word.to_string() != "an" && word.to_string() != "the" {
                    let finished_word = &stemmer.stem(word).into_owned(); // Find the stem
                    simple_sentence.push(finished_word.to_string()); // Then add it to the simplified sentence
                }
            }
            // Once the sentence is simplified, search for the words
            for word in &simple_sentence {
                if word.to_string() == "not" {
                    multiplier = multiplier * -1;
                } else  if word.to_string() == "veri" || word.to_string() == "pretti" {
                    multiplier = multiplier * 2;
                } else {
                    println!("Checking dictionary for {}", word);
                    // Iterate through our dictionary and add the score if the word is found
                    for check in WORDS.iter() {
                        if &check.0 == word {
                            score = score + &check.1 * multiplier;
                            println!("Final score of sentence: {}", score);
                            multiplier = 1;
                        }
                    }
                }

                
            } 
            let sentiment_score = AFFINParser::generate_affin_scores(&simple_sentence);
            println!("Sentiment Score: {}", sentiment_score.net_score);

        }
        
        unsafe {
           enemy.start_tolerance = enemy.start_tolerance + score;
            if enemy.start_tolerance <= 0 {
                loss_writer.send(ConvLossEvent());
            } else if score < 0 {
                player_sent = false;
            } else {
                player_sent = true;
                win_writer.send(ConvWinEvent());
            } 
            
            if CUR_TURN <= MAX_TURNS {
                CUR_TURN = CUR_TURN + 1;
                //println!("Current Turn: {}", CUR_TURN);
            }
            else{ // TODO: CASE REACHED FINAL TURN -- NEEDS TO BE HANDLED
                //println!("OUT OF RESPONSES: CONV PHASE OVER");
                loss_writer.send(ConvLossEvent());
            }
            let enemy_resp: &str;
            //println!("player sentiment good: {}", PLAYER_SENT);
            if player_sent {
                enemy_resp = NICE_RESPONSES[CUR_TURN as usize];
            } else {
                enemy_resp = MEAN_RESPONSES[CUR_TURN as usize];
            }
            //println!("Current Turn: {}", CUR_TURN);
            enem_dlg.sections[0].value = enemy_resp.to_string();
        }
    }
}
