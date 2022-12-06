use rand::Rng;
use rand::seq::IteratorRandom;
use bevy::{
    prelude::*
};
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use super::CollideEvent;
use super::FightWinEvent;
use super::FightLossEvent;
use super::Level;

const PLAYER_W: f32 = 64.;
const PLAYER_H: f32 = 128.;
const FLOOR_HEIGHT: f32 = -crate::WIN_H/4.;
const PLAYER_SPEED: f32 = 500.; // play around with these values to make movement feel right for a fighting game
const ACCEL_RATE: f32 = 5000.;  
const GRAVITY: f32 = 2000.;
const HEALTHBAR_X: f32 = 5.*100.;
const HEALTHBAR_Y: f32 = 32.;
const PUNCHATTACK: f32 =10.;
const KICKATTACK: f32 =20.;

//===============COMPONENTS================

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct DespawnTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct PlayerName(String);

#[derive(Component)]
pub struct EnemyName(String);

#[derive(Component)]
pub struct PlayerAttack;	// used to identify attacks done by the player

#[derive(Component)]
pub struct EnemyAttack;

// use a velocity component to track the player and enemy velocity
#[derive(Component)]
pub struct Velocity {
	velocity: Vec2,
}
impl Velocity {
	fn new() -> Self {
		Self { velocity: Vec2::splat(0.) }
	}
}

// stats struct to track relevant statistics (just health for now) for entities
#[derive(Component)]
pub struct Stats {
	health: f32,
}
impl Stats {
	fn new() -> Self {
		Self { health: 100. }	// start every entity at 100 health
	}
}

#[derive(Component)]
pub struct Actions {	// actions struct to help regulate actions
	attacking: bool,
	blocking: bool,
}
impl Actions {
	fn new() -> Self {
		Self { 
			attacking: false,
			blocking: false,
		}
	}
}

#[derive(Component)]
pub struct ActionTimer(Timer);

#[derive(Component)]
pub struct HealthBarTop;
#[derive(Component)]
pub struct HealthBarBottom;

//=======================ENEMY STATE MACHINE===========================

pub struct StateMachine<S>{
	state: S,
}

pub struct Stand {
	stand: std::time::Duration,
}

pub struct Move{
	x: f32,
	y: f32,
}

pub struct Attack{
	attack: f32
}

impl StateMachine<Stand> {
	fn new() -> Self {
		StateMachine { 
			state: Stand{
				stand: std::time::Duration::new(0 , 0), 
			}
		}
	}
}

impl From<StateMachine<Stand>> for StateMachine<Move> {
	fn from(val: StateMachine<Stand>) -> StateMachine<Move> {
		let mut enemy: Query<(&mut Transform, &mut Velocity), With<Enemy>>; 
		//logic before transition
		//let (mut enemy_transform, mut enemy_velocity) = enemy.single_mut();
		let mut deltav = Vec2::splat(0.);

		deltav.x += 1.;
		deltav.y -= 1.;
		let new_state = StateMachine {
         	state: Move{
				x : deltav.x,
				y : deltav.y,
			}	
		}; 
		return new_state;
	}
}

/*impl From<StateMachine<Move>> for StateMachine<Attack> {
	fn from(val: StateMachine<Move>) -> StateMachine<Attack> { 
		//logic before transition
		StateMachine  {
			state: Attack{
					attack: 1.,
		}
	} 
	}
}

impl From<StateMachine<Attack>> for StateMachine<Move> {
	fn from(val: StateMachine<Attack>) -> StateMachine<Move> {
		//logic before transition
		StateMachine  {
			state: Move{
					moving: 1.,
			}
		}
	}
}*/

impl From<StateMachine<Move>> for StateMachine<Stand> {
	fn from(val: StateMachine<Move>) -> StateMachine<Stand> {
		//logic before transition
		StateMachine  {
			state: Stand{
				stand: std::time::Duration::new(0 , 0),
			}

		}

	}
}

//====================FIGHT SETUP/OVERHEAD FUNCTIONS=======================

pub fn setup_fight(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    level: ResMut<State<Level>>,
) {
    let texture_handle = asset_server.load("start_sprite_screen.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(320., 180.), 46, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(4.)),
        ..default()
    })
    .insert(AnimationTimer(Timer::from_seconds(0.125,  true)));

    // spawn the player sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-crate::WIN_W/4., 0., 1.),
            ..default()
        },
        ..default()
    })
    .insert(Velocity::new())
    .insert(Stats::new())
    .insert(Actions::new())
    .insert(Player);


    match level.current(){
        Level::Level1 => {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::ORANGE_RED,
                    custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(crate::WIN_W/4., 0., 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity::new())
            .insert(Stats::new())
            .insert(Actions::new())
            .insert(ActionTimer(Timer::from_seconds(2., false)))    // enemy can perform one attack or block every 2 secs
            .insert(Enemy);
        } Level::Level2 => {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::TOMATO,
                    custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(crate::WIN_W/4., 0., 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity::new())
            .insert(Stats::new())
            .insert(Actions::new())
            .insert(ActionTimer(Timer::from_seconds(2., false)))    // enemy can perform one attack or block every 2 secs
            .insert(Enemy);
        } Level::Level3 => {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::ANTIQUE_WHITE,
                    custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(crate::WIN_W/4., 0., 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity::new())
            .insert(Stats::new())
            .insert(Actions::new())
            .insert(ActionTimer(Timer::from_seconds(2., false)))    // enemy can perform one attack or block every 2 secs
            .insert(Enemy);
        } Level::Level4 => {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::SALMON,
                    custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(crate::WIN_W/4., 0., 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity::new())
            .insert(Stats::new())
            .insert(Actions::new())
            .insert(ActionTimer(Timer::from_seconds(2., false)))    // enemy can perform one attack or block every 2 secs
            .insert(Enemy);
        } Level::Level5 => {
            commands.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::PINK,
                    custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(crate::WIN_W/4., 0., 1.),
                    ..default()
                },
                ..default()
            })
            .insert(Velocity::new())
            .insert(Stats::new())
            .insert(Actions::new())
            .insert(ActionTimer(Timer::from_seconds(2., false)))    // enemy can perform one attack or block every 2 secs
            .insert(Enemy);
        }
    }
    // spawn a dummy enemy sprite
    /* 
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::ORANGE_RED,
            custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(crate::WIN_W/4., 0., 1.),
            ..default()
        },
        ..default()
    })
    .insert(Velocity::new())
    .insert(Stats::new())
    .insert(Actions::new())
    .insert(ActionTimer(Timer::from_seconds(2., false)))    // enemy can perform one attack or block every 2 secs
    .insert(Enemy);
    */

    // spawn player health bar
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::LIME_GREEN,
            custom_size: Some(Vec2::new(HEALTHBAR_X, HEALTHBAR_Y)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new( (-crate::WIN_W/2. + HEALTHBAR_X/2.)+16., (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
            ..default()
        },
        ..default()
    })
    .insert(HealthBarTop)
    .insert(PlayerName(String::from("Player")));
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(HEALTHBAR_X, HEALTHBAR_Y)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new( (-crate::WIN_W/2. + HEALTHBAR_X/2.)+16., (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 1.),
            ..default()
        },
        ..default()
    })
    .insert(HealthBarBottom)
    .insert(PlayerName(String::from("Player")));

    // spawn enemy health bar
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::LIME_GREEN,
            custom_size: Some(Vec2::new(HEALTHBAR_X, HEALTHBAR_Y)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new( (crate::WIN_W/2. - HEALTHBAR_X/2.)-16., (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
            ..default()
        },
        ..default()
    })
    .insert(HealthBarTop)
    .insert(EnemyName(String::from("dummy")));
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::RED,
            custom_size: Some(Vec2::new(HEALTHBAR_X, HEALTHBAR_Y)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new( (crate::WIN_W/2. - HEALTHBAR_X/2.)-16., (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 1.),
            ..default()
        },
        ..default()
    })
    .insert(HealthBarBottom)
    .insert(EnemyName(String::from("dummy")));
    
    
}


//animates the background image (just copied over from the start screen code in main.rs)
pub fn animate_background(
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<(
		&mut AnimationTimer, 
		&mut TextureAtlasSprite, 
		&Handle<TextureAtlas>
	)>,
){
	for(mut timer, mut sprite, _texture_atlas_handle) in &mut query{
		timer.tick(time.delta());
		if timer.just_finished(){
			let texture_atlas = texture_atlases.get(_texture_atlas_handle).unwrap();
			sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
		}
	}
}

//despawns player, healthbar, attack entities
pub fn clear_fight(
    mut commands: Commands,
    mut player: Query<Entity, With<Player>>,
	mut enemy: Query<Entity, With<Enemy>>,
	healthbar_tops: Query<Entity, With<HealthBarTop>>,
	healthbar_bottoms: Query<Entity, With<HealthBarBottom>>,
	player_attack: Query<Entity, With<PlayerAttack>>,
	enemy_attack: Query<Entity, With<EnemyAttack>>,
) {
    let player_eid = player.single_mut();
    commands.entity(player_eid).despawn();
	let enemy_eid = enemy.single_mut();
	commands.entity(enemy_eid).despawn();
	for eid in healthbar_tops.iter() {
		commands.entity(eid).despawn();
	}
	for eid in healthbar_bottoms.iter() {
		commands.entity(eid).despawn();
	}
	for eid in player_attack.iter(){
		commands.entity(eid).despawn();
	}
	for eid in enemy_attack.iter(){
		commands.entity(eid).despawn();
	}
}

fn check_collision(
	apos: Vec3,
	asize: Vec2,
	bpos: Vec3, 
	bsize: Vec2,
)->bool{
	let collision = collide(
		apos,
		asize,
		bpos,
		bsize
	);
	if collision.is_some(){
		return true;
	}
	return false;
}

/*pub fn remove_popup(
	time: Res<Time>,
	mut rmpopup: Query<(&mut DespawnTimer, &mut Visibility)>
) {
	for (mut timer, mut vis_map) in rmpopup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			vis_map.is_visible = false;
		}
	}
}*/

//========================PLAYER FUNCTIONS============================

pub fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
	mut player_send: EventWriter<CollideEvent>,
    mut player: Query<(&mut Transform, &mut Velocity, &mut Actions), Without<Enemy>>,
	mut enemy: Query<&Transform, With<Enemy>>
) {
    let (mut player_transform, mut player_velocity, player_actions) = player.single_mut();
	let enemy_transform = enemy.single_mut();

	let mut deltav = Vec2::splat(0.);

	if input.pressed(KeyCode::A) && !(player_actions.blocking) && !(player_actions.attacking) {
		deltav.x -= 1.;
	}

	if input.pressed(KeyCode::D) && !(player_actions.blocking) && !(player_actions.attacking) {
		deltav.x += 1.;
	}

	// player needs to be on the floor to jump, hence the floor height check
	if input.pressed(KeyCode::W) && player_transform.translation.y <= (FLOOR_HEIGHT + PLAYER_H)
		&& !(player_actions.blocking) && !(player_actions.attacking) {
		deltav.y += 1.;
	}
	else {
		deltav.y -= 1.;
	}


	// calculating by deltat instead of just relying on frames *should* normalize for different framerates
	let deltat = time.delta_seconds();
	let acc = ACCEL_RATE * deltat;
	let grav = GRAVITY * deltat;

	// calculate the velocity vector by multiplying it with the acceleration constant
	let new_vel_x = if deltav.x != 0. {
		(player_velocity.velocity.x + (deltav.normalize_or_zero().x * acc)).clamp(-PLAYER_SPEED, PLAYER_SPEED)
	} else if player_velocity.velocity.x > acc {	// if I try to be clever and do both in one conditional it doesn't work right
		player_velocity.velocity.x - acc
	} else if player_velocity.velocity.x < -acc {
		player_velocity.velocity.x + acc
	} else {
		0.
	};

	let new_vel_y = if deltav.y > 0. {
		//player has jumped
		deltav.normalize_or_zero().y * (grav * 25.)
	} else if deltav.y < 0.
		&& player_transform.translation.y != enemy_transform.translation.y+PLAYER_H {
		//player is falling/not jumping
		player_velocity.velocity.y + (deltav.normalize_or_zero().y * grav)
	} else if player_transform.translation.y == enemy_transform.translation.y+PLAYER_H 
		&& (player_transform.translation.x+PLAYER_W/2.0 <= enemy_transform.translation.x-PLAYER_W/2.0
		|| player_transform.translation.x-PLAYER_W/2.0 >= enemy_transform.translation.x+PLAYER_W/2.0) {
		player_velocity.velocity.y + (deltav.normalize_or_zero().y * grav)
	} else {
		0.
	};

	player_velocity.velocity = Vec2::new(
		new_vel_x,
		new_vel_y,
	);
	let change = player_velocity.velocity * deltat;

	let new_pos = player_transform.translation + Vec3::new(
		change.x,
		0.,
		0.,
	);
	//calls collision function to see if a collision happened  
	let collide = check_collision(
		//apos
		new_pos,
		//asize
		Vec2::new(PLAYER_H/2., PLAYER_W/2.),
		//bpos
		enemy_transform.translation,
		//bsize
		Vec2::new(PLAYER_H/2.,PLAYER_W/2.)
	);

	// if a collision does happen then an event is sent to the collision_handle system 
	// A string is also sent that indicates on what side the collision occurred relative to the player
	if collide {
		if new_pos.x < enemy_transform.translation.x {
			player_send.send(CollideEvent(true,String::from("rightside")));
		}
		if new_pos.x > enemy_transform.translation.x {
			player_send.send(CollideEvent(true,String::from("leftside")));
		}
	}
	// if the new pos does not collide with the enemy and is inside the window then the player pos is set to the new pos
	if !collide 
	  && new_pos.x >= -(crate::WIN_W/2.) + PLAYER_W/2. 
	  && new_pos.x <= crate::WIN_W/2. - PLAYER_W/2.
	{	
		player_transform.translation = new_pos;
		player_send.send(CollideEvent(false,String::from("nocollision")));
	}
	 

	let new_pos = player_transform.translation + Vec3::new(
		//changes the new position to FLOOR_HEIGHT + PLAYER_H/2 if it becomes less than that
		0.,
		if change.y + player_transform.translation.y < FLOOR_HEIGHT + PLAYER_H/2.{
			 -1.*player_transform.translation.y + FLOOR_HEIGHT + PLAYER_H/2.
		} else {
			change.y
		},
		0.,
	);
	
	//The code below is similar to the one above except it deals with the y position
	let collide=check_collision(
		//apos
		new_pos,
		//asize
		Vec2::new(PLAYER_H/2., PLAYER_W/2.),
		//bpos
		Vec3::new(enemy_transform.translation.x,enemy_transform.translation.y+PLAYER_H,enemy_transform.translation.z),
		//bsize
		Vec2::new(PLAYER_H/2.,PLAYER_W/2.)
	);

	if collide {
		if new_pos.y < enemy_transform.translation.y+PLAYER_H {
			player_send.send(CollideEvent(true,String::from("topside")));
		}
		if new_pos.y >= enemy_transform.translation.y+PLAYER_H {
			player_send.send(CollideEvent(true,String::from("bottomside")));
		}
	}
	if !collide
	   && new_pos.y >= FLOOR_HEIGHT + PLAYER_H/2. 
	   && new_pos.y <= crate::WIN_H/2. - PLAYER_H/2.	
	{
		player_transform.translation = new_pos;
		player_send.send(CollideEvent(false,String::from("nocollision")));
	}
}

// collision_handle system deals with all the collisions and what to do depending on the kind of collision
pub fn collision_handle(
	mut commands: Commands,
	enemy_healthbar_en: Query<Entity, (With<EnemyName>,With<HealthBarTop>)>,
	player_healthbar_en: Query<Entity, (With<PlayerName>,With<HealthBarTop>)>,
	mut event_receive: EventReader<CollideEvent>,
	mut win_state: EventWriter<FightWinEvent>,
	mut loss_state: EventWriter<FightLossEvent>,
	mut player: Query<(&mut Transform, &mut Velocity, &mut Stats, &mut Actions), With<Player>>,
	mut enemy: Query<(&mut Transform, &mut Velocity, &mut Stats, &mut Actions), (With<Enemy>, Without<Player>)>
){
	let (mut player_transform, mut player_velocity, mut player_stats, player_actions) = player.single_mut();
	let (enemy_transform, mut enemy_velocity, mut enemy_stats, enemy_actions) = enemy.single_mut();
	for p in event_receive.iter(){
		if p.0 == true {
			if p.1 == "rightside" {
				// if the collision is on the right side of the player then just adjust the player x pos so it can't pass through the enemy
				player_transform.translation = player_transform.translation + Vec3::new(
					enemy_transform.translation.x-player_transform.translation.x-PLAYER_W,
					0.,
					0.,
				);
			} else if p.1 == "leftside" {
				// if the collision is on the left side of the player then just adjust the player x pos so it can't pass through the enemy
				player_transform.translation = player_transform.translation - Vec3::new(
					player_transform.translation.x-enemy_transform.translation.x-PLAYER_W,
					0.,
					0.,
				);
			} else if p.1 == "bottomside" {
				// This is supposed to handle collisions for the bottom side of the player
				// for example if the player jumps on the enemy then there would be a collision on the bottom side of the player
				// It is currently not working because I think the gravity being applied needs to be taken into consideration
				player_transform.translation = Vec3::new(
					player_transform.translation.x,
					enemy_transform.translation.y + PLAYER_H,
					player_transform.translation.z,
				);
			} else if p.1 == "punchleft" {
				// this handles punch collisions 
				// The current healthbar entity is despawned and a new entity with updated health, size and pos is spawned 
				if !enemy_actions.blocking {
					enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
						700.,
						0.,
					);
					if enemy_stats.health-PUNCHATTACK > 0.{ 
						enemy_stats.health = enemy_stats.health-PUNCHATTACK;
					} else {
						enemy_stats.health = 0.;
					}
				} else {
					enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
						350.,
						0.,
					);
					if enemy_stats.health-(PUNCHATTACK/4.) > 0. {
						enemy_stats.health = enemy_stats.health-(PUNCHATTACK/4.);
					} else {
						enemy_stats.health = 0.;
					}
				}
				let enemy_healthbar_eid = enemy_healthbar_en.single();
				let x_size = 5.*enemy_stats.health;
				let x_pos = (crate::WIN_W/2. - 5.*enemy_stats.health/2.)-16.;
				commands.entity(enemy_healthbar_eid).despawn();
				commands.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::LIME_GREEN,
						custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
						..default()
					},
					transform: Transform {
						translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
						..default()
					},
					..default()
				})
				.insert(HealthBarTop)
				.insert(EnemyName(String::from("dummy")));
				if enemy_stats.health == 0.{
					win_state.send(FightWinEvent());	// enemy health has reached zero, player has won the fight
				}
			} else if p.1 == "punchright" {
				// this handles punch collisions 
				// The current healthbar entity is despawned and a new entity with updated health, size and pos is spawned 
				if !enemy_actions.blocking {
					enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
						-700.,
						0.,
					);
					if enemy_stats.health-PUNCHATTACK > 0.{ 
						enemy_stats.health = enemy_stats.health-PUNCHATTACK;
					} else {
						enemy_stats.health = 0.;
					}
				} else {
					enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
						-350.,
						0.,
					);
					if enemy_stats.health-(PUNCHATTACK/4.) > 0.{ 
						enemy_stats.health = enemy_stats.health-(PUNCHATTACK/4.);
					} else {
						enemy_stats.health = 0.;
					}
				}
				let enemy_healthbar_eid = enemy_healthbar_en.single();
				let x_size = 5.*enemy_stats.health;
				let x_pos = (crate::WIN_W/2. - 5.*enemy_stats.health/2.)-16.;
				commands.entity(enemy_healthbar_eid).despawn();
				commands.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::LIME_GREEN,
						custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
						..default()
					},
					transform: Transform {
						translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
						..default()
					},
					..default()
				})
				.insert(HealthBarTop)
				.insert(EnemyName(String::from("dummy")));
				if enemy_stats.health == 0.{
					win_state.send(FightWinEvent());	// enemy health has reached zero, player has won the fight
				}
			} else if p.1 == "kickleft" {
				if !enemy_actions.blocking {
					enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
						1000.,
						0.,
					);
					if enemy_stats.health - KICKATTACK > 0. {
						enemy_stats.health = enemy_stats.health-KICKATTACK;
					} else {
						enemy_stats.health = 0.;
					}
				} else {
					enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
						500.,
						0.,
					);
					if enemy_stats.health - (KICKATTACK/4.) > 0. {
						enemy_stats.health = enemy_stats.health-(KICKATTACK/4.);
					} else {
						enemy_stats.health = 0.;
					}
				}
				println!("Enemy health is {}",enemy_stats.health);
				let enemy_healthbar_eid = enemy_healthbar_en.single();
				let x_size = 5.*enemy_stats.health;
				let x_pos = (crate::WIN_W/2. - 5.*enemy_stats.health/2.)-16.;
				commands.entity(enemy_healthbar_eid).despawn();
				commands.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::LIME_GREEN,
						custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
						..default()
					},
					transform: Transform {
						translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
						..default()
					},
					..default()
				})
				.insert(HealthBarTop)
				.insert(EnemyName(String::from("dummy")));
				if enemy_stats.health == 0.{
					win_state.send(FightWinEvent());	// enemy health has reached zero, player has won the fight
				}
			} else if p.1 == "kickright" {
				if !enemy_actions.blocking {
					enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
						-1000.,
						0.,
					);
					if enemy_stats.health - KICKATTACK > 0. {
						enemy_stats.health = enemy_stats.health-KICKATTACK;
					} else {
						enemy_stats.health = 0.;
					}
				} else {
					enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
						-500.,
						0.,
					);
					if enemy_stats.health - (KICKATTACK/4.) > 0. {
						enemy_stats.health = enemy_stats.health-(KICKATTACK/4.);
					} else {
						enemy_stats.health = 0.;
					}
				}
				println!("Enemy health is {}",enemy_stats.health);
				let enemy_healthbar_eid = enemy_healthbar_en.single();
				let x_size = 5.*enemy_stats.health;
				let x_pos = (crate::WIN_W/2. - 5.*enemy_stats.health/2.)-16.;
				commands.entity(enemy_healthbar_eid).despawn();
				commands.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::LIME_GREEN,
						custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
						..default()
					},
					transform: Transform {
						translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
						..default()
					},
					..default()
				})
				.insert(HealthBarTop)
				.insert(EnemyName(String::from("dummy")));
				if enemy_stats.health == 0.{
					win_state.send(FightWinEvent());	// enemy health has reached zero, player has won the fight
				}
			} else if p.1 == "enemy_punchleft" {
				// this handles punch collisions 
				// The current healthbar entity is despawned and a new entity with updated health, size and pos is spawned 
				if !player_actions.blocking {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						700.,
						0.,
					);
					if player_stats.health-PUNCHATTACK > 0.{ 
						player_stats.health = player_stats.health-PUNCHATTACK;
					} else {
						player_stats.health = 0.;
					}
				} else {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						350.,
						0.,
					);
					if player_stats.health-(PUNCHATTACK/4.) > 0.{ 
						player_stats.health = player_stats.health-(PUNCHATTACK/4.);
					} else {
						player_stats.health = 0.;
					}
				}
				let player_healthbar_eid = player_healthbar_en.single();
				let x_size = 5.*player_stats.health;
				let x_pos = (-crate::WIN_W/2. + 5.*player_stats.health/2.)+16.;
				commands.entity(player_healthbar_eid).despawn();
				commands.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::LIME_GREEN,
						custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
						..default()
					},
					transform: Transform {
						translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
						..default()
					},
					..default()
				})
				.insert(HealthBarTop)
				.insert(PlayerName(String::from("Player")));
				if player_stats.health == 0.{
					loss_state.send(FightLossEvent());	// player health has reached zero, player has lost the fight
				}
			} else if p.1 == "enemy_punchright" {
				// this handles punch collisions 
				// The current healthbar entity is despawned and a new entity with updated health, size and pos is spawned 
				if !player_actions.blocking {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						-700.,
						0.,
					);
					if player_stats.health-PUNCHATTACK > 0.{ 
						player_stats.health = player_stats.health-PUNCHATTACK;
					} else {
						player_stats.health = 0.;
					}
				} else {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						-350.,
						0.,
					);
					if player_stats.health-(PUNCHATTACK/4.) > 0.{ 
						player_stats.health = player_stats.health-(PUNCHATTACK/4.);
					} else {
						player_stats.health = 0.;
					}
				}
				let player_healthbar_eid = player_healthbar_en.single();
				let x_size = 5.*player_stats.health;
				let x_pos = (-crate::WIN_W/2. + 5.*player_stats.health/2.)+16.;
				commands.entity(player_healthbar_eid).despawn();
				commands.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::LIME_GREEN,
						custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
						..default()
					},
					transform: Transform {
						translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
						..default()
					},
					..default()
				})
				.insert(HealthBarTop)
				.insert(PlayerName(String::from("Player")));
				if player_stats.health == 0.{
					loss_state.send(FightLossEvent());	// player health has reached zero, player has lost the fight
				}
			} else if p.1 == "enemy_kickleft" {
				if !player_actions.blocking {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						1000.,
						0.,
					);
					if player_stats.health - KICKATTACK > 0. {
						player_stats.health = player_stats.health-20.;
					} else {
						player_stats.health = 0.;
					}
				} else {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						500.,
						0.,
					);
					if player_stats.health - (KICKATTACK/4.) > 0. {
						player_stats.health = player_stats.health-(KICKATTACK/4.);
					} else {
						player_stats.health = 0.;
					}
				}
				println!("Player health is {}",player_stats.health);
				let player_healthbar_eid = player_healthbar_en.single();
				let x_size = 5.*player_stats.health;
				let x_pos = (-crate::WIN_W/2. + 5.*player_stats.health/2.)+16.;
				commands.entity(player_healthbar_eid).despawn();
				commands.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::LIME_GREEN,
						custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
						..default()
					},
					transform: Transform {
						translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
						..default()
					},
					..default()
				})
				.insert(HealthBarTop)
				.insert(PlayerName(String::from("Player")));
				if player_stats.health == 0.{
					loss_state.send(FightLossEvent());	// player health has reached zero, player has lost the fight
				}
			} else if p.1 == "enemy_kickright" {
				if !player_actions.blocking {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						-1000.,
						0.,
					);
					if player_stats.health - KICKATTACK > 0. {
						player_stats.health = player_stats.health-20.;
					} else {
						player_stats.health = 0.;
					}
				} else {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						-500.,
						0.,
					);
					if player_stats.health - (KICKATTACK/4.) > 0. {
						player_stats.health = player_stats.health-(KICKATTACK/4.);
					} else {
						player_stats.health = 0.;
					}
				}
				println!("Player health is {}",player_stats.health);
				let player_healthbar_eid = player_healthbar_en.single();
				let x_size = 5.*player_stats.health;
				let x_pos = (-crate::WIN_W/2. + 5.*player_stats.health/2.)+16.;
				commands.entity(player_healthbar_eid).despawn();
				commands.spawn_bundle(SpriteBundle {
					sprite: Sprite {
						color: Color::LIME_GREEN,
						custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
						..default()
					},
					transform: Transform {
						translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 2.),
						..default()
					},
					..default()
				})
				.insert(HealthBarTop)
				.insert(PlayerName(String::from("Player")));
				if player_stats.health == 0.{
					loss_state.send(FightLossEvent());	// player health has reached zero, player has lost the fight
				}
			}
		}
	}
}

pub fn attack(
	input: Res<Input<KeyCode>>, 
	mut player_send: EventWriter<CollideEvent>,
	mut player: Query<(&mut Transform, &mut Actions), With<Player>>,
	mut commands: Commands, 
	mut enemy: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
){
    let (player_transform, mut player_actions) = player.single_mut();
	let enemy_transform = enemy.single_mut();
	let mut attack_xpos = 60.;
	if player_transform.translation.x > enemy_transform.translation.x {
		 attack_xpos = -60.;
	}
	
	if input.just_pressed(KeyCode::P)	// punch
		&& !(player_actions.blocking)
		&& !(player_actions.attacking)
	{
		player_actions.attacking = true;

        commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::GREEN,
				custom_size: Some(Vec2::new(80.,32.)),
				..default()
			},
            transform: Transform {
            translation: Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y+32., 2.),
            ..default()
        },
			..default()
		})
        .insert(DespawnTimer(Timer::from_seconds(0.2,false)))
		.insert(PlayerAttack);
		// The collision function is called to see if a collision occurred
		// if there was a collision a signal is sent to the collision_handle system
		let punch_collide_result = collide(
			//apos
			Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y+32., 2.),
			//asize
			Vec2::new(80.,32.),
			//bpos
			Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 2.),
			//bsize
			Vec2::new(PLAYER_W, PLAYER_H)
		);
		if let Some(side) = punch_collide_result {
			match side {
				Collision::Left => player_send.send(CollideEvent(true, String::from("punchleft"))),
				Collision::Right => player_send.send(CollideEvent(true, String::from("punchright"))),
				Collision::Inside => {
					if player_transform.translation.x < enemy_transform.translation.x {
						player_send.send(CollideEvent(true, String::from("punchleft")));
					}
					else if player_transform.translation.x > enemy_transform.translation.x {
						player_send.send(CollideEvent(true, String::from("punchright")));
					}
				},
				Collision::Top => (),
				Collision::Bottom => (),	// top and bottom not used for knockback effect
			}
		}
    }
	if input.just_pressed(KeyCode::K)	// kick
		&& !(player_actions.blocking) 
		&& !(player_actions.attacking)
	{	
		player_actions.attacking = true;

        commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::GREEN,
				custom_size: Some(Vec2::new(80.,32.)),
				..default()
			},
            transform: Transform {
            translation: Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y-32., 2.),
            ..default()
        },
			..default()
		})
        .insert(DespawnTimer(Timer::from_seconds(0.4,false)))
		.insert(PlayerAttack);
		
		let kick_collide_result = collide(
			//apos
			Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y-32., 2.),
			//asize
			Vec2::new(80.,32.),
			//bpos
			Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 2.),
			//bsize
			Vec2::new(PLAYER_W, PLAYER_H)
		);
		if let Some(side) = kick_collide_result {
			match side {
				Collision::Left => player_send.send(CollideEvent(true, String::from("kickleft"))),
				Collision::Right => player_send.send(CollideEvent(true, String::from("kickright"))),
				Collision::Inside => {
					if player_transform.translation.x < enemy_transform.translation.x {
						player_send.send(CollideEvent(true, String::from("kickleft")));
					}
					else if player_transform.translation.x > enemy_transform.translation.x {
						player_send.send(CollideEvent(true, String::from("kickright")));
					}
				},
				Collision::Top => (),
				Collision::Bottom => (),	// top and bottom not used for knockback effect
			}
		}
    }
}

pub fn block(
	input: Res<Input<KeyCode>>, 
	mut player: Query<(&mut Sprite, &mut Actions), With<Player>>,
) {
	let (mut player_sprite, mut player_actions) = player.single_mut();

	if input.pressed(KeyCode::B) && !(player_actions.attacking) {
		player_actions.blocking = true;
		player_sprite.color = Color::MIDNIGHT_BLUE;	// change player sprite color so we know the blocking is working
	}
	if input.just_released(KeyCode::B) {
		player_actions.blocking = false;
		player_sprite.color = Color::BLUE;
	}
}

pub fn player_remove_attack(
	time: Res<Time>,
	mut attack_popup: Query<(&mut DespawnTimer, &mut Visibility), With<PlayerAttack>>,
	mut player: Query<&mut Actions, With<Player>>,
) {
	let mut player_actions = player.single_mut();

	for (mut timer, mut vis_map) in attack_popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			vis_map.is_visible = false;
			player_actions.attacking = false;
		}
	}
}

//========================ENEMY FUNCTIONS===============================

//movement decision-making will come later as a part of AI
pub fn move_enemy(
	time: Res<Time>,
	mut enemy_send: EventWriter<CollideEvent>,
	mut enemy: Query<(&mut Transform, &mut Velocity, &mut Actions), (With<Enemy>, Without<Player>)>,
	mut player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
	let (mut enemy_transform, mut enemy_velocity, enemy_actions) = enemy.single_mut();
	let player_transform = player.single_mut();

	let mut deltav = Vec2::splat(0.);

	// (this is where decision making about movement will go)
	let begin_state = StateMachine::new();
	let next_state = StateMachine::<Move>::from(begin_state);

	let attack_length: f32 = 80.;
	if !enemy_actions.attacking && !enemy_actions.blocking {
		//follow the player and move within range of an attack 
		if player_transform.translation.x+attack_length < enemy_transform.translation.x-PLAYER_W/2. {
			deltav.x = -next_state.state.x;
		} else if player_transform.translation.x-attack_length > enemy_transform.translation.x+PLAYER_W/2. {
			deltav.x = next_state.state.x;
		} 
	} else {
		deltav.x = 0.;
	}
	
	deltav.y -= 1.;	// enemy is affected by gravity, if we allow enemy to jump this should be a conditional (like the player)
	
	// calculating by deltat instead of just relying on frames *should* normalize for different framerates
	let deltat = time.delta_seconds();
	let acc = ACCEL_RATE * deltat;
	let grav = GRAVITY * deltat;

	// calculate the velocity vector by multiplying it with the acceleration constant
	let new_vel_x = if deltav.x != 0. {
		(enemy_velocity.velocity.x + (deltav.normalize_or_zero().x * acc)).clamp(-PLAYER_SPEED, PLAYER_SPEED)
	} else if enemy_velocity.velocity.x > acc {	// if I try to be clever and do both in one conditional it doesn't work right
		enemy_velocity.velocity.x - acc
	} else if enemy_velocity.velocity.x < -acc {
		enemy_velocity.velocity.x + acc
	} else {
		0.
	};

	let new_vel_y = if deltav.y > 0. {
		//enemy has jumped
		deltav.normalize_or_zero().y * (grav * 25.)
	} else if deltav.y < 0. {
		//enemy is falling/not jumping
		enemy_velocity.velocity.y + (deltav.normalize_or_zero().y * grav)
	} else {
		0.
	};

	enemy_velocity.velocity = Vec2::new(
		new_vel_x,
		new_vel_y,
	);

	let change = enemy_velocity.velocity * deltat;

	let new_pos = enemy_transform.translation + Vec3::new(
		change.x,
		0.,
		0.,
	);

	let collide = check_collision(
		//apos
		new_pos,
		//asize
		Vec2::new(PLAYER_H/2., PLAYER_W/2.),
		//bpos
		player_transform.translation,
		//bsize
		Vec2::new(PLAYER_H/2.,PLAYER_W/2.)
	);

	if collide {
		if new_pos.x < enemy_transform.translation.x{
			enemy_send.send(CollideEvent(true,String::from("rightside")));
		}
		if new_pos.x > enemy_transform.translation.x{
			enemy_send.send(CollideEvent(true,String::from("leftside")));
		}
	}
	if !collide 
	  && new_pos.x >= -(crate::WIN_W/2.) + PLAYER_W/2. 
	  && new_pos.x <= crate::WIN_W/2. - PLAYER_W/2.
	{
		enemy_transform.translation = new_pos;
		enemy_send.send(CollideEvent(false,String::from("nocollision")));
	}


	let new_pos = enemy_transform.translation + Vec3::new(
		0.,
		if change.y + enemy_transform.translation.y < FLOOR_HEIGHT + PLAYER_H/2.{
			-1.*enemy_transform.translation.y + FLOOR_HEIGHT + PLAYER_H/2.
	   } else {
		   change.y
	   },
		0.,
	);

	let collide = check_collision(
		//apos
		new_pos,
		//asize
		Vec2::new(PLAYER_H/2., PLAYER_W/2.),
		//bpos
		player_transform.translation,
		//bsize
		Vec2::new(PLAYER_H/2.,PLAYER_W/2.)
	);

	if collide {
		if new_pos.y < player_transform.translation.y{
			enemy_send.send(CollideEvent(true,String::from("bottomside")));
		}
		if new_pos.y >= player_transform.translation.y+PLAYER_H/2.{
			enemy_send.send(CollideEvent(true,String::from("topside")));
		}
	}
	if !collide 
	  && new_pos.y >= -(crate::WIN_W/2.) + PLAYER_W/2. 
	  && new_pos.y <= crate::WIN_W/2. - PLAYER_W/2.
	{
		enemy_transform.translation = new_pos;
		enemy_send.send(CollideEvent(false,String::from("nocollision")));
	}
}

// thought we would need this but turns out we don't as of rn since i just handle the enemy attacks in the other collision_handle function
/*pub fn enemy_collision_handle(
	mut commands: Commands,
	player_healthbar_en: Query<Entity, (With<Player>,With<HealthBarTop>)>,
	mut lose_state: EventWriter<FightLossEvent>,
	mut enemy_receive: EventReader<CollideEvent>,
	mut player: Query<(&mut Transform, &mut Velocity, &mut Stats), (With<Player>, Without<Enemy>)>,
	mut enemy: Query<(&mut Transform, &mut Velocity, &mut Stats), (With<Enemy>, Without<Player>)>,
){
	let (mut player_transform, mut player_velocity, mut player_stats) = player.single_mut();
	let (mut enemy_transform, mut enemy_velocity, mut enemy_stats) = enemy.single_mut();
		for p in enemy_receive.iter(){
			if p.0 == true {
				if p.1.contains("rightside"){
					// if the collision is on the right side of the player then just adjust the player x pos so it can't pass through the enemy
					enemy_transform.translation = enemy_transform.translation + Vec3::new(
						player_transform.translation.x-enemy_transform.translation.x-PLAYER_W,
						0.,
						0.,
					);
					enemy_velocity.velocity.x = -1.;
			  	} else if p.1.contains("leftside") {
					// if the collision is on the left side of the player then just adjust the player x pos so it can't pass through the enemy
						enemy_transform.translation = enemy_transform.translation - Vec3::new(
						enemy_transform.translation.x-player_transform.translation.x-PLAYER_W,
						0.,
						0.,
					);
					enemy_velocity.velocity.x = 1.;
				} else if p.1.contains("bottomside") {
					// This is supposed to handle collisions for the bottom side of the player
					// for example if the player jumps on the enemy then there would be a collision on the bottom side of the player
					// It is currently not working because I think the gravity being applied needs to be taken into consideration
					enemy_transform.translation = Vec3::new(
						enemy_transform.translation.x,
						player_transform.translation.y + PLAYER_H,
						enemy_transform.translation.z,
					);
				} else if p.1.contains("punchleft") {
					// this handles punch collisions 
					// The current healthbar entity is despawned and a new entity with updated health, size and pos is spawned 
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						700.,
						0.,
					);
					if player_stats.health-PUNCHATTACK > 0.{ 
						player_stats.health = player_stats.health-PUNCHATTACK;
					} else {
						player_stats.health = 0.;
					}
					let player_healthbar_eid = player_healthbar_en.single();
					let x_size = 5.*player_stats.health;
					let x_pos = (crate::WIN_W/2. - 5.*player_stats.health/2.)-16.;
					commands.entity(player_healthbar_eid).despawn();
					commands.spawn_bundle(SpriteBundle {
						sprite: Sprite {
							color: Color::LIME_GREEN,
							custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
							..default()
						},
						transform: Transform {
							translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 1.),
							..default()
						},
						..default()
					})
					.insert(HealthBarTop)
					.insert(EnemyName(String::from("dummy")));
					if player_stats.health == 0. {
						lose_state.send(FightLossEvent());	// player health has reached zero, player has lost the fight
					}
				} else if p.1.contains("punchright") {
					// this handles punch collisions 
					// The current healthbar entity is despawned and a new entity with updated health, size and pos is spawned 
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						-700.,
						0.,
					);
					if player_stats.health-PUNCHATTACK > 0.{ 
						player_stats.health=player_stats.health-PUNCHATTACK;
					} else {
						player_stats.health = 0.;
					}
					let player_healthbar_eid = player_healthbar_en.single();
					let x_size = 5.*player_stats.health;
					let x_pos = (crate::WIN_W/2. - 5.*player_stats.health/2.)-16.;
					commands.entity(player_healthbar_eid).despawn();
					commands.spawn_bundle(SpriteBundle {
						sprite: Sprite {
							color: Color::LIME_GREEN,
							custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
							..default()
						},
						transform: Transform {
							translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 1.),
							..default()
						},
						..default()
					})
					.insert(HealthBarTop)
					.insert(EnemyName(String::from("Player")));
					if player_stats.health == 0. {
						lose_state.send(FightLossEvent());	// player health has reached zero, player has lost the fight
					}
				} else if p.1.contains("kickleft") {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						1000.,
						0.,
					);
					if player_stats.health - KICKATTACK > 0. {
						player_stats.health = player_stats.health-20.;
					} else {
						player_stats.health = 0.;
					}
					println!("Enemy health is {}",player_stats.health);
					let player_healthbar_eid = player_healthbar_en.single();
					let x_size = 5.*player_stats.health;
					let x_pos = (crate::WIN_W/2. - 5.*player_stats.health/2.)-16.;
					commands.entity(player_healthbar_eid).despawn();
					commands.spawn_bundle(SpriteBundle {
						sprite: Sprite {
							color: Color::LIME_GREEN,
							custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
							..default()
						},
						transform: Transform {
							translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 1.),
							..default()
						},
						..default()
					})
					.insert(HealthBarTop)
					.insert(EnemyName(String::from("Player")));
					if player_stats.health == 0. {
						lose_state.send(FightLossEvent());	// player health has reached zero, player has lost the fight
					}
				} else if p.1.contains("kickright") {
					player_velocity.velocity = player_velocity.velocity + Vec2::new(
						-1000.,
						0.,
					);
					if player_stats.health - KICKATTACK > 0. {
						player_stats.health=player_stats.health-20.;
					} else {
						player_stats.health=0.;
					}
					println!("Player health is {}",player_stats.health);
					let player_healthbar_eid = player_healthbar_en.single();
					let x_size = 5.*player_stats.health;
					let x_pos = (crate::WIN_W/2. - 5.*player_stats.health/2.)-16.;
					commands.entity(player_healthbar_eid).despawn();
					commands.spawn_bundle(SpriteBundle {
						sprite: Sprite {
							color: Color::LIME_GREEN,
							custom_size: Some(Vec2::new(x_size, HEALTHBAR_Y)),
							..default()
						},
						transform: Transform {
							translation: Vec3::new( x_pos, (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 1.),
							..default()
						},
						..default()
					})
					.insert(HealthBarTop)
					.insert(EnemyName(String::from("Player")));
					if player_stats.health == 0. {
						lose_state.send(FightLossEvent());	// player health has reached zero, player has lost the fight
					}
			  	}
			}
		}
}*/

pub fn enemy_take_action(
	time: Res<Time>,
	mut player: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
	mut enemy: Query<(&mut Transform, &mut ActionTimer, &mut Actions, &mut Sprite, & mut Stats), (With<Enemy>, Without<Player>)>,
	// these last three are to pass to the punch, kick, and block functions
	enemy_send: EventWriter<CollideEvent>,
	commands: Commands,
	level: ResMut<State<Level>>,
) {
	let (enemy_transform, mut enemy_timer, mut enemy_actions, mut enemy_sprite,mut enemy_stats) = enemy.single_mut();
	let player_transform = player.single_mut();
	let mut rng = rand::thread_rng();
	let attack_length: f32 = 80.;

	let mut enemy_within_range: bool = false;
	if player_transform.translation.x < enemy_transform.translation.x {
		if (enemy_transform.translation.x - PLAYER_W/2.) - (player_transform.translation.x + PLAYER_W/2.) < attack_length {
			enemy_within_range = true;
		}
	} else if player_transform.translation.x - PLAYER_W > enemy_transform.translation.x {
		if (player_transform.translation.x - PLAYER_W/2.) - (enemy_transform.translation.x + PLAYER_W/2.) < attack_length {
			enemy_within_range = true;
		}
	}

	enemy_timer.0.tick(time.delta());
	if enemy_timer.0.finished() && enemy_actions.blocking {
		enemy_unblock(&mut enemy_sprite, &mut enemy_actions);
	}

	if enemy_timer.0.finished() && (!enemy_actions.attacking && !enemy_actions.blocking) && enemy_within_range {
		enemy_timer.0.reset();

		// choose an action for the enemy to take (punch, kick, or block)
		let mut next_choice = rng.gen_range(0..3);	// generate 0, 1, or 2 since we have 3 options
		


		match level.current(){
			Level::Level1 =>{
				if(enemy_stats.health < 20.0){
					let choices=[0,0,0,1,1,1,2];
					 next_choice = choices.into_iter().choose(&mut rng).unwrap();	// generate 0, 1, or 2 since we have 3 options
				  }
			}
			Level::Level2 =>{
				if(enemy_stats.health < 30.0){
					let choices=[0,0,0,1,1,1,2];
					 next_choice = choices.into_iter().choose(&mut rng).unwrap();	// generate 0, 1, or 2 since we have 3 options
				  }
			}
			Level::Level3 =>{
				if(enemy_stats.health < 50.0){
					enemy_timer.0=Timer::from_seconds(0.1, false);
					let choices=[0,0,0,1,1,1,2];
					 next_choice = choices.into_iter().choose(&mut rng).unwrap();	// generate 0, 1, or 2 since we have 3 options
				  }
			}
			Level::Level4 =>{
				enemy_timer.0=Timer::from_seconds(0.01, false);
				if(enemy_stats.health < 60.0){
					let choices=[0,0,0,1,1,1,2];
					 next_choice = choices.into_iter().choose(&mut rng).unwrap();	// generate 0, 1, or 2 since we have 3 options
				  }

			}
			Level::Level5 =>{
				enemy_timer.0=Timer::from_seconds(0.01, false);
				if(enemy_stats.health < 70.0){
					let choices=[0,0,0,1,1,1,2];
					 next_choice = choices.into_iter().choose(&mut rng).unwrap();	// generate 0, 1, or 2 since we have 3 options
				  }
			}

		}

		match next_choice {
			0 => {
				enemy_punch(
					enemy_send,
					player,
					commands,
					enemy
				);
			},
			1 => {
				enemy_kick(
					enemy_send,
					player,
					commands,
					enemy
				);
			},
			2 => {
				enemy_block(&mut enemy_sprite, &mut enemy_actions);
			},
			_ => info!("We should never get here..."),
		}
	}
}

pub fn enemy_punch( 
	mut enemy_send: EventWriter<CollideEvent>,
	mut player: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
	mut commands: Commands, 
	mut enemy: Query<(&mut Transform, &mut ActionTimer, &mut Actions, &mut Sprite, &mut Stats), (With<Enemy>, Without<Player>)>,
){
    let player_transform = player.single_mut();
	let (enemy_transform, _enemy_timer, mut enemy_actions, mut enemy_sprite, mut enemy_stats) = enemy.single_mut();
	let mut attack_xpos = 60.;
	if enemy_transform.translation.x > player_transform.translation.x {
		 attack_xpos = -60.;
	}
	
	enemy_actions.attacking = true;

	commands
	.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			color: Color::GREEN,
			custom_size: Some(Vec2::new(80.,32.)),
			..default()
		},
		transform: Transform {
		translation: Vec3::new(enemy_transform.translation.x+attack_xpos, enemy_transform.translation.y+32., 2.),
		..default()
	},
		..default()
	})
	.insert(DespawnTimer(Timer::from_seconds(0.2,false)))
	.insert(EnemyAttack);
	// The collision function is called to see if a collision occurred
	// if there was a collision a signal is sent to the collision_handle system
	let punch_collide_result = collide(
		//apos
		Vec3::new(enemy_transform.translation.x+attack_xpos, enemy_transform.translation.y+32., 2.),
		//asize
		Vec2::new(80.,32.),
		//bpos
		Vec3::new(player_transform.translation.x, player_transform.translation.y, 2.),
		//bsize
		Vec2::new(PLAYER_W, PLAYER_H)
	);
	if let Some(side) = punch_collide_result {
		info!("enemy punch collision!");
		match side {
			Collision::Left => enemy_send.send(CollideEvent(true, String::from("enemy_punchleft"))),
			Collision::Right => enemy_send.send(CollideEvent(true, String::from("enemy_punchright"))),
			Collision::Inside => {
				if enemy_transform.translation.x < player_transform.translation.x {
					enemy_send.send(CollideEvent(true, String::from("enemy_punchleft")));
				}
				else if enemy_transform.translation.x > player_transform.translation.x {
					enemy_send.send(CollideEvent(true, String::from("enemy_punchright")));
				}
			},
			Collision::Top => (),
			Collision::Bottom => (),	// top and bottom not used for knockback effect
		}
	}
}

pub fn enemy_kick(
	mut enemy_send: EventWriter<CollideEvent>,
	mut player: Query<&mut Transform, (With<Player>, Without<Enemy>)>,
	mut commands: Commands, 
	mut enemy: Query<(&mut Transform, &mut ActionTimer, &mut Actions, &mut Sprite, &mut Stats), (With<Enemy>, Without<Player>)>,
){
    let player_transform = player.single_mut();
	let (enemy_transform, _enemy_timer, mut enemy_actions, mut enemy_sprite, mut enemu_stats) = enemy.single_mut();
	let mut attack_xpos = 60.;
	if enemy_transform.translation.x > player_transform.translation.x {
		 attack_xpos = -60.;
	}
	
	enemy_actions.attacking = true;

	commands
	.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			color: Color::GREEN,
			custom_size: Some(Vec2::new(80.,32.)),
			..default()
		},
		transform: Transform {
		translation: Vec3::new(enemy_transform.translation.x+attack_xpos, enemy_transform.translation.y-32., 2.),
		..default()
	},
		..default()
	})
	.insert(DespawnTimer(Timer::from_seconds(0.4,false)))
	.insert(EnemyAttack);
	
	let kick_collide_result = collide(
		//apos
		Vec3::new(enemy_transform.translation.x+attack_xpos, enemy_transform.translation.y-32., 2.),
		//asize
		Vec2::new(80.,32.),
		//bpos
		Vec3::new(player_transform.translation.x, player_transform.translation.y, 2.),
		//bsize
		Vec2::new(PLAYER_W, PLAYER_H)
	);
	if let Some(side) = kick_collide_result {
		info!("enemy kick collision!");
		match side {
			Collision::Left => enemy_send.send(CollideEvent(true, String::from("enemy_kickleft"))),
			Collision::Right => enemy_send.send(CollideEvent(true, String::from("enemy_kickright"))),
			Collision::Inside => {
				if enemy_transform.translation.x < player_transform.translation.x {
					enemy_send.send(CollideEvent(true, String::from("enemy_kickleft")));
				}
				else if player_transform.translation.x > enemy_transform.translation.x {
					enemy_send.send(CollideEvent(true, String::from("enemy_kickright")));
				}
			},
			Collision::Top => (),
			Collision::Bottom => (),	// top and bottom not used for knockback effect
		}
	}
}

pub fn enemy_block(
    mut enemy_sprite: &mut Sprite,
    mut enemy_actions: &mut Actions,
) {
    enemy_actions.blocking = true;

    enemy_sprite.color = Color::rgb(enemy_sprite.color.r()*0.5, 
                                    enemy_sprite.color.g()*0.5, 
                                    enemy_sprite.color.b()*0.5);    // change enemy sprite color so we know the blocking is working
}



pub fn enemy_unblock(
    mut enemy_sprite: &mut Sprite,
    mut enemy_actions: &mut Actions,
) {
    enemy_actions.blocking = false;
    enemy_sprite.color = Color::rgb(enemy_sprite.color.r()*2.,
                                    enemy_sprite.color.g()*2.,
                                    enemy_sprite.color.b()*2.,);

    
}

pub fn enemy_remove_attack(
	time: Res<Time>,
	mut attack_popup: Query<(&mut DespawnTimer, &mut Visibility), With<EnemyAttack>>,
	mut enemy: Query<&mut Actions, With<Enemy>>,
) {
	let mut enemy_actions = enemy.single_mut();

	for (mut timer, mut vis_map) in attack_popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			vis_map.is_visible = false;
			enemy_actions.attacking = false;
		}
	}
}
