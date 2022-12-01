use bevy::{
	prelude::*,
	text::Text2dBounds,
};
mod AFFINParser;

use super::ConvInputEvent;
use super::ConvLossEvent;
use super::ConvWinEvent;
use super::Level;
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
	start_tolerance: f64,
    name: String,
    age: i8,
    job: String,
    description: String,
}

const NICE_RESPONSES: [&'static str;6] = ["Thank you!", "I really appreciate that!",
"You're such a good neighbor!", "You're a life saver", "Thanks! I'll see you later.", "Have a good day!"];

const MEAN_RESPONSES: [&'static str;6] = ["Why would you say that to me?", "Why would you say that to me?",
"I will literally call the police.", "Do you want to fight?!?!???!", "You're the worst neighbor EVER!", "You don't want to take it there!"];

const NICE_GREETINGS: [&'static str;6] = ["Hello!", "How are you?", "I hope your day is going good so far!", 
"How is your day going?", "Long time, no see! How are you?", "How's it going?"];

const MEAN_GREETINGS: [&'static str;6] = ["What is WRONG with you?", "Don't smile at me! You KNOW what you did.", "I can not stand you!", 
"You're actually the worst neighbor ever!", "Why do you act like this?", "You're ruining my day!!"];

const NEGATOR_WORDS: [&'static str;8] = ["not", "don't", "dont", "neither","never","seldom", "nevermore","little"];
const EMPHASIZING_WORDS: [&'static str;17] = ["veri", "pretti", "extrem", "vast", "huge", "especi", "over", 
"exceed", "extra", "immens", "tremend", "excess", "great", "genuin", "realli", "super", "truli"];

// 0 - start (enemy prompt, wait for player prompt)
// 1 - after player first response, fetch ai response
// 2 - after player second response, fetch ai response
// etc.. 
// FINAL TURN - after player final response, return fight or not
const MAX_TURNS: i32 = 4;
const START_TURN: i32 = 0;
static mut CUR_TURN: i32 = 0;
static mut CHECK_LEVEL: i32 = 1;

// Spawn all entities to be used in the conversation part of the game
pub fn setup_conversation(
	mut commands: Commands,
	mut clear_color: ResMut<ClearColor>, 
	asset_server: Res<AssetServer>,
    mut level: ResMut<State<Level>>,

){
    unsafe {
       println!("Current level: {}", CHECK_LEVEL); 
       CUR_TURN = 0; //reinitialize current # of turns
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

    // SPAWN THE BACKGROUND SCREEN

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

	let box_size = Vec2::new(700.0, 200.0);
    let box_position = Vec2::new(-45.0, -250.0);
    let box_position_two = Vec2::new(45.0, 175.0);


    //SPAWN THE DIALOGUE BOXES

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
    
    match level.current(){
        Level::Level1 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("CathyRobinson.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    //color: Color::WHITE,
                    //custom_size: Some(Vec2::new(200., 200.)),
                    ..default()
                },
                ..default()
            }).insert(Enemy{start_tolerance: 100., name: String::from("Catherine Robinson"), age: 27, job: String::from("Teacher"), description: String::from("nice")});
            
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
		}
		
        Level::Level2 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("BillyWickler.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()
            }).insert(Enemy{start_tolerance: 50., name: String::from("Billy Wickler"), age: 49, job: String::from("Cowboy Rancher"), description: String::from("brash")});
            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Listen here, my dog ran away earlier and I know you have him.", enemy_text_style),
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
        }
		
        Level::Level3 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("GloriaBrown.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()
            }).insert(Enemy{start_tolerance: 70., name: String::from("Gloria Brown"), age: 72, job: String::from("Retired Library Manager"), description: String::from("blunt")});
            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("I need someone to read to me", enemy_text_style),
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
        }
		Level::Level4 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("JeffreyMadden.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()
            }).insert(Enemy{start_tolerance: 30., name: String::from("Jeffrey Madden"), age: 34, job: String::from("Stockbroker"), description: String::from("stressed")});
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("You need to move your car NOW, I'm having a party and it's blocking the driveway", enemy_text_style),
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
        }
        Level::Level5 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("KarenMartinez.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()
            }).insert(Enemy{start_tolerance: 10., name: String::from("Katie Martinez"), age: 42, job: String::from("Mom"), description: String::from("mean")});
            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Why are you ALWAYS having people over? Is it safe to have all these strangers in a family-friendly neighborhood?", enemy_text_style),
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
        }
    }

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
) {
    let mut multiplier: f64;
    let mut enemy = enemy.single_mut();
    let mut startTol = enemy.start_tolerance;
    let mut curTol = startTol;
    let stemmer = Stemmer::create(Algorithm::English);
    let mut simple_sentence: Vec<String> = Vec::new();
    let mut enem_dlg = enemy_dialogue.single_mut();
    let mut player_sent = true;

    for input in ev_reader.iter() {
        multiplier = 1.0;
        // Get the input and do some string manipulation to make it easier to parse
        let mut string = input.0.to_string();
        string.make_ascii_lowercase();
        string = string.trim_end().to_string();
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
            let mut word_was_neg = false;
            //check if the word is present in our NEGATOR array
            for negativeWord in NEGATOR_WORDS{
                if(word.to_string() == negativeWord){
                    multiplier = multiplier * -1.0;
                    word_was_neg = true;
                    println!("word was negative");
                }
            }
            if(!word_was_neg){ //the word was not negative so check it for emphasis
                println!("This is the word we're on {}", word.to_string());
                for emphasis in EMPHASIZING_WORDS{
                    if(word.to_string() == emphasis){
                        multiplier = multiplier * 2.0;
                        println!("emphasizer used");
                    }
                }

            }
        } 
        let mut sentiment_score = AFFINParser::generate_affin_scores(&simple_sentence);
        if sentiment_score.net_score == 0.0 {
            sentiment_score.net_score = multiplier;
        } else {
            sentiment_score.net_score = sentiment_score.net_score * multiplier;
        }
        println!("Sentiment Score: {}", sentiment_score.net_score);
        //enemy.start_tolerance = enemy.start_tolerance + sentiment_score.net_score;
        curTol = curTol + sentiment_score.net_score;
        println!("This is current tol {}", curTol);
        
        
        //if the enemy has no more tolerance
        if curTol <= 0.0 {
            loss_writer.send(ConvLossEvent());
         }else if curTol >= startTol*2.0 {  //the enemy is so satisfied, the level was won
            //let enemy_resp = "You know what? I love you! Have a great day.";
            //enem_dlg.sections[0].value = enemy_resp.to_string();
            win_writer.send(ConvWinEvent());
         }else if sentiment_score.net_score <= 0.0 {
            player_sent = false;
        } else {
            player_sent = true;
        }
        
        unsafe {    
            //IF WE ARE NOT OUT OF TURNS, INCREMENT TURNS
            if CUR_TURN <= MAX_TURNS {
                CUR_TURN = CUR_TURN + 1;
                //println!("Current Turn: {}", CUR_TURN);
            }
            //CASE REACHED FINAL TURN AND PLAYER DIDN'T TRIGGER FIGHT,BUT ENEMY TOLERANCE LESS THAN HALF OF ORIGINAL
            else if (curTol <= (startTol/2.)) { 
                loss_writer.send(ConvLossEvent());
            }
            //MAX TURNS REACHED AND ENEMY IS MORE THAN HALF CONTENT, LEVEL WON
            else{
                win_writer.send(ConvWinEvent());
            }
            let enemy_resp: &str;
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
