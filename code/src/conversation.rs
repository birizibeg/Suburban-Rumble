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
       //println!("Current level: {}", CHECK_LEVEL); 
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

    //create a random number in case we are in the bonus rounds of the game and need a random tolerance
    let mut rng = rand::thread_rng();
    let mut random_tolerance = rng.gen::<f64>();
    random_tolerance = random_tolerance * 10.;
    //println!("This is the tolerance {}", random_tolerance);
    //let mut random_tolerance = rng.gen_range(0..21.);
    

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
            NICE_REPLIES: ["I 'preciate you hearin' me out, old man.", "Yeah, I don' know - that darn dog gotta mind of its own.", "You are jus' so nice. No'thin like the bull nurses from back home.", "I wish I had someone like ya on the farm, ya so easy-goin'!", "Ya know, I like ya old man. Ya should come over for a base burner some time.", "Thought ya was gon' give me some corral dust, but I 'preciate your response, old man."], 
            MEAN_REPLIES: ["I used to tussle livestock! Ya dont wanna crawl my hump!", "Ya better hold your horses old man!", "Relax old man, I don' wanna have to give ya a lick an' a promise!", "Who do you think ya talkin' to old man?", "Listen here Grandpa, don't go airin' your lungs at ME!", "Shut ya big bazoo, Grandpa."]});
            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Listen here boy, my dog got to runnin' away and I hope you have 'em!", enemy_text_style),
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
            NICE_REPLIES: ["Aww you're just the sweetest boy - I oughta pinch your cheecks!", "You're so nice, I'm gonna make you a fixin' of my famous mac & cheese!", "Thank you for understanding. My eyesight and hearing ain't what it used to be.", "Oh bless your heart - you're just too kind!", "Neighbors like you sure do make life easier for an old lady like me!", "You're sweeter than my award-winning peach cobbler pie!"], 
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
            MEAN_REPLIES: ["Why would you say that to me?", "You can't take me in a fight, so I suggest you calm down!", "I will literally call the police.", "Shut your freaking mouth!", "You're the worst neighbor EVER!", "You don't want to take it there!"]});

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
            NICE_REPLIES: ["I wish you would've been resonable before - we could've avoided all this.", "You're actually nice, you just make dumb decisions.", "I would think you would have learned to be smarter since you're so old, but at least you're kind.", "After this conversation, I don't hate you as much as I did before anymore.", "You're a horrible neighbor, but at least you're a good person.", "You're not as bad as I thought, but we can work on the manners. I'll have my kids teach you."], 
            MEAN_REPLIES: ["You are not a good person.", " My kids are honeslty smarter than you, you idiot!", "I will call the police on you RIGHT NOW!", 
            "As a Mom who deals with toddlers - I can honestly say you're the most immature person I know.", "You need to be put on time-out for this behavior!", "I HATE having you as a neighbor - you need to move!"]});

            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Why are you ALWAYS having people over? Don't you understand that having strangers in a family-friendly neighborhood is unsafe?", enemy_text_style),
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


        //STRETCH GOAL IMPLEMENTATION, USERS COME BACK
        Level::Level6 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("CathyRobinson.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()
            }).insert(Enemy{start_tolerance: random_tolerance, cur_tol: random_tolerance, name: String::from("Catherine Robinson"), age: 27, job: String::from("Teacher"), description: String::from("nice"), 
            NICE_REPLIES: ["You're awesome!", "I can bring you cookies more often if you're going to be this kind!", "I wish the teahcers I worked with were as great as you!", "I'm so glad we became friends!", "You are the best neighbor I've met here so far.", "This was really my pleasure - you're so great!"],
            MEAN_REPLIES: ["Why are you being mean all of a sudden?", "How did your mood change so fast? Let's count down from 10 to cool down.", "You have such a potty mouth!", "Oh my -- I deal with bad kids all day and no one has ever spoken to me like this!", "How about you watch your tone?!?!?!", "I'm going to count to 5, by the time I'm done you better fix your attitude!"]}); //Vec::new()

            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Hi friend, it's Cathy again! I brought you some of the cookies I baked!", enemy_text_style),
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
		
        Level::Level7 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("BillyWickler.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()

            }).insert(Enemy{start_tolerance: random_tolerance, cur_tol: random_tolerance, name: String::from("Billy Wickler"), age: 49, job: String::from("Cowboy Rancher"), description: String::from("brash"), 
            NICE_REPLIES: ["You might be my favorite bull nurse aroun'", "You know I don't let just anyone with my cattle - I guess I really like you.", "If you do a good job, I'll even let you milk ma cows! ", "You know - you're an honest man. I can get ya into the rancher business if ya want!", "I really like ya. We can have a hog-killin' good time together!", "Even though you're a tenderfoot - ya have a good at'tude so I can teach you everythin' I know."], 
            MEAN_REPLIES: ["You better watch that wobblin' jaw before it gets ugly.", "What's you probl'm? Someone must'v put snakes in your chicken coop!", "You no more than a yellow-belly, I suggest you watch your mouth!", "Who are you talk'n to? You beter hang up ya fiddle before I get angry.", "Listen here Grandpa, you're barking up the wrong tree!", "I'll make ya a horse's ******* if ya keep talk'n to me like that!"]});
            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Rancher Billy here. Since we're friends now, I want you to help me feed my cattle!", enemy_text_style),
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
		
        Level::Level8 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("GloriaBrown.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()

            }).insert(Enemy{start_tolerance: random_tolerance, cur_tol: random_tolerance, name: String::from("Gloria Brown"), age: 72, job: String::from("Retired Library Manager"), description: String::from("blunt"), 
            NICE_REPLIES: ["My husband really needs friends, and you're so sweet I think you'd be great for him!", "You have such good manners - your parents sho did do a good job!", "Aren't you just nicer than a cold sweet tea on a hot summer day!", "This is the Southern hospitality I been missing since I moved up here!", "Now you are just the kindest little thing, I'll be coming here more often!", "I'm gonna make you some of my WORLD-famous green-bean casserole to express my gratitude"], 
            MEAN_REPLIES: ["Now you 'bout as dumb as log in the mud.", "You must want a knuckle sandiwch talking to me like that.", "Where I'm from in the South - those are fighting words!", "What happened to respecting your elders? You watch yourself.", "I'm fixin to call the police on you if you keep acting like this!", "You don't deserve to be friends with MY husband, you're a bad person."]});
            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Hello honey bun! I told my husband you were just the nicest guy, and now he wants to meet you.", enemy_text_style),
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
		Level::Level9 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("JeffreyMadden.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()
            }).insert(Enemy{start_tolerance: random_tolerance, cur_tol: random_tolerance, name: String::from("Jeffrey Madden"), age: 34, job: String::from("Stockbroker"), description: String::from("stressed"), 
            NICE_REPLIES: ["Oh...I guess that's fine.","Wow, you're actually really cool.","I don't know why all of our neighbors hate you, you're pretty okay.","I'm glad you're understanding - just don't block my driveway again","Wow as a New Yorker, I'm not used to people being so nice.","Dude, I'm seriously gonna invite you to my next party."], 
            MEAN_REPLIES: ["You can't say that to me.", "Bro, do you even know who my dad is?", "**** you, old man.", "Ok, you watch your mouth now.", "Right...", "I'm actually calling the police this time."]});

            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("You need to mow your lawn. I can see it growing from my house and I don't like how long it is.", enemy_text_style),
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
        Level::Level10 =>{
            commands.spawn_bundle(SpriteBundle {
                texture: asset_server.load("KarenMartinez.png"),
                transform: Transform::from_xyz(0., 0., 1.),
                sprite: Sprite {
                    ..default()
                },
                ..default()

            }).insert(Enemy{start_tolerance: random_tolerance, cur_tol:random_tolerance, name: String::from("Karen Martinez"), age: 42, job: String::from("Mom"), description: String::from("mean"), 
            NICE_REPLIES: ["Well I guess you understand what I'm saying, then.", "I think I just misunderstood you because we are very different people.", "Well you know what they say...even a broken clock is right twice a day.", "You're not as terrible a neighbor as I thought.", "You're not such a bad person.", "My husband wouldn't hate you."], 
            MEAN_REPLIES: ["My husband would hate you.", " My kids have better manners than you, you idiot!", "Don't you dare say that to me!", 
            "I have never in my entire life met someone who is a rude as you are.", "Your mother should have taught you better!", "I am never talking to you again!"]});

            
            commands.spawn_bundle(Text2dBundle {
                text: Text::from_section("Didn't I talk to you about having people over? I don't want my kids playing outside if a bunch of random people will be here.", enemy_text_style),
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
                //stemmer changes words that end with y to end in i instead, the dictionary doesn't have use for those words so
                //we make an exception here
                 if word.to_string().chars().last().unwrap() == 'y'{
                    simple_sentence.push(word.to_string());
                }
                else{
                    simple_sentence.push(finished_word.to_string()); // Then add it to the simplified sentence
                }
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
            else if cur_tol <= (start_tol/2.) || !player_sent{  //if max turns done, and cur_tol is less than half or player said something mean
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
