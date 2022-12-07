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
use rand::Rng;

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
    cur_tol: f64,
    name: String,
    age: i8,
    job: String,
    description: String,
    NICE_REPLIES: [&'static str;6], 
    MEAN_REPLIES: [&'static str;6],
}

const NICE_RESPONSES: [&'static str;6] = ["Thank you!", "I really appreciate that!",
"You're such a good neighbor!", "You're a life saver", "Thanks! I'll see you later.", "Have a good day!"];

const MEAN_RESPONSES: [&'static str;6] = ["Why would you say that to me?", "Why would you say that to me?",
"I will literally call the police.", "Do you want to fight?!?!???!", "You're the worst neighbor EVER!", "You don't want to take it there!"];

const NEGATOR_WORDS: [&'static str;8] = ["not", "don't", "dont", "neither","never","seldom", "nevermore","little"];
const EMPHASIZING_WORDS: [&'static str;17] = ["veri", "pretti", "extrem", "vast", "huge", "especi", "over", 
"exceed", "extra", "immens", "tremend", "excess", "great", "genuin", "realli", "super", "truli"];

// 0 - start (enemy prompt, wait for player prompt)
// 1 - after player first response, fetch ai response
// 2 - after player second response, fetch ai response
// etc.. 
// FINAL TURN - after player final response, return fight or not
const MAX_TURNS: i32 = 4;
static mut CUR_TURN: i32 = 0;
static mut CHECK_LEVEL: i32 = 1;
static mut check_dups: Vec<i32> = Vec::new();

// Spawn all entities to be used in the conversation part of the game
// Spawn all entities to be used in the conversation part of the game
pub fn setup_conversation(
	mut commands: Commands,
	mut clear_color: ResMut<ClearColor>, 
	asset_server: Res<AssetServer>,
    level: ResMut<State<Level>>,

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
    

    //BEGINNING OF MATCH STATEMENT TO SPAWN VARIOUS ENEMIES (SPRITE & CHAT) 
    //BASED ON WHICH LEVEL IT IS AFTER THE CHECK OF LEVELS
    match level.current(){
        Level::Level1 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("CathyRobinson.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()
            }).insert(Enemy{start_tolerance: 50., cur_tol:50., name: String::from("Catherine Robinson"), age: 27, job: String::from("Teacher"), description: String::from("nice"), 
            NICE_REPLIES: ["You are just the best!", "You're an absolute life-saver!", "I came over for sugar, but I feel like I'm leaving with a friend!", "You have no idea how much this means to me!", "You are so amazing!", "Wow, I spend so much time talking to kids - I forgot how nice adults could be!"],
            MEAN_REPLIES: ["I've seen this kind of bad behavior before. Is everything okay at home?", "You're obviously in a bad mood. Let's count down from 5 to cool down.", "I have students who act like this all the time. Let's just breathe...Ooohh, Ahhh, Ooohh, Ahhh, Oooh, Ahhh", "Let's try using our kind words, sweetie.", "Those are bad words and you know that.", "Why are you acting like this? Talk to me."]}); //Vec::new()

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

            }).insert(Enemy{start_tolerance: 26., cur_tol:26., name: String::from("Billy Wickler"), age: 49, job: String::from("Cowboy Rancher"), description: String::from("brash"), 
            NICE_REPLIES: ["I 'preciate you hearin' me out, boy.", "Yeah, I don' know - that darn dog gotta mind of its own.", "You are jus' so nice. No'thin like the bull nurses from back home.", "I wish I had someone like you on the farm, you so easy-goin'!", "Ya know, I like ya boy. You should come over for a base burner some time.", "Thought you was gon; give me some corral dust, but I 'preciate your response, boy."], 
            MEAN_REPLIES: ["I used to tussle livestock! You dont wanna crawl my hump!", "You better hold your horses boy!", "Relax boy, I don' wanna have to give ya a lick an' a promise!", "Who do you think you talkin' to boy?", "Listen here Grandpa, don't go airin' your lungs at ME!", "Shut your big bazoo, Grandpa."]});
            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Listen here boy, my dog got to runnin' away and I think you took em!", enemy_text_style),
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

            }).insert(Enemy{start_tolerance: 36., cur_tol:36., name: String::from("Gloria Brown"), age: 72, job: String::from("Retired Library Manager"), description: String::from("blunt"), 
            NICE_REPLIES: ["Aww you're just the sweetest boy - I oughta pinch your cheecks!", "You're so nice, I'm gonna mke you a fixin' of my famous mac & cheese!", "Thank you for understanding. My eyesight and hearing ain't what it used to be.", "Oh bless your heart - you're just too kind!", "Neighbors like you sure do make life easier.", "Thank you! You're sweeter than my award-winning peach cobbler pie!"], 
            MEAN_REPLIES: ["Who taught a young boy like you to talk like that?!", "You talk to me that way, we be fighting 'till the cows come home!", "You're getting too big for your britches talk'n like that!", "I outghta make you wash your mouth out with soap!", "Oh, I'll knock you into the middle of next week!", "You wouldn't know manners if it slapped you in the face!"]});
            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Hi honey. I need someone to read to me...", enemy_text_style),
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
            }).insert(Enemy{start_tolerance: 12., cur_tol:12., name: String::from("Jeffrey Madden"), age: 34, job: String::from("Stockbroker"), description: String::from("stressed"), 
            NICE_REPLIES: ["I guess you're not as dumb as I thought.","If I knew you were so easygoing, I would've invited you to my party.","Why doesn't anyone like you? You're not that bad.","I'm glad you're understanding - just don't block my driveway again","Wow as a New Yorker, I'm not used to people being so nice.","Thanks for being such a chill guy."], 
            MEAN_REPLIES: ["Why would you say that to me?", "You can't take me in a fight, so I suggest you calm down!", "I will literally call the police.", "Shut the **** up!", "You're the worst neighbor EVER!", "You don't want to take it there!"]});

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

            }).insert(Enemy{start_tolerance: 6., cur_tol:6., name: String::from("Karen Martinez"), age: 42, job: String::from("Mom"), description: String::from("mean"), 
            NICE_REPLIES: ["I wish you would've been resonable before - we could've avoided all this.", "You're actually nice, you just make dumb decisions.", "I would think you would have learned to be smarter since you're so old, but at least you're kind.", "I guess you're not as bad as I thought.", "You're a horrible neighbor, but at least you're a pretty good person.", "You're not as bad as I thought, but we can work on the manners. I'll have my kids teach you."], 
            MEAN_REPLIES: ["You are not a good person.", " My kids are honeslty smarter than you, you idiot!", "I will call the police on you RIGHT NOW!", 
            "As a Mom who deals with toddlers - I can honestly say you're the most immature person I know.", "You need to be put on time-out for this behavior!", "I HATE having you as a neighbor - you need to move!"]});

            
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
    //reinit vector array
    unsafe{
        check_dups = Vec::new();
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
) {
    let mut multiplier: f64;
    let mut enemy = enemy.single_mut();
    let mut cur_tol = enemy.cur_tol;
    let start_tol = enemy.start_tolerance;
    let stemmer = Stemmer::create(Algorithm::English);
    let mut simple_sentence: Vec<String> = Vec::new();
    let mut enem_dlg = enemy_dialogue.single_mut();
    let mut player_sent = true;
    let mut rng = rand::thread_rng();
    let mut random;

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
            for negative_word in NEGATOR_WORDS{
                if word.to_string() == negative_word {
                    multiplier = multiplier * -1.0;
                    word_was_neg = true;
                }
            }
            if !word_was_neg{ //the word was not negative so check it for emphasis
                for emphasis in EMPHASIZING_WORDS{
                    if word.to_string() == emphasis {
                        multiplier = multiplier * 2.0;
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
        //enemy.start_tolerance = enemy.start_tolerance + sentiment_score.net_score;
        cur_tol = cur_tol + sentiment_score.net_score;
        enemy.cur_tol = cur_tol;        
        
        //if the enemy has no more tolerance
        if cur_tol <= 0.0 {
            loss_writer.send(ConvLossEvent());
            // TODO: Fix this so that it checks correctly
         }else if cur_tol >= start_tol*2.0 {  //the enemy is so satisfied, the level was won
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
            // TODO: Fix this so it checks correctly
            else if cur_tol <= (start_tol/2.) || sentiment_score.net_score < -1.0 { 
                loss_writer.send(ConvLossEvent());
            }
            //MAX TURNS REACHED AND ENEMY IS MORE THAN HALF CONTENT, LEVEL WON
            else{
                win_writer.send(ConvWinEvent());
            }
            let enemy_resp: &str;
            random = rng.gen_range(0..6);
            //println!("This is the number selected {}", random);
            //check to make sure you won't get a response that's already been used
            while(check_dups.contains(&random)){
                //println!("It was a dup {}", random);
                random = rng.gen_range(0..6);
                //println!("this is the new one {}", random);
            }
            check_dups.push(random);
            if player_sent {
                enemy_resp = enemy.NICE_REPLIES[random as usize];
            } else {
                enemy_resp = enemy.MEAN_REPLIES[random as usize];
            }
            //println!("Current Turn: {}", CUR_TURN);
            enem_dlg.sections[0].value = enemy_resp.to_string();
        }
    }
}
