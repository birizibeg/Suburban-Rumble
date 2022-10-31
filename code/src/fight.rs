use bevy::{
    prelude::*
};
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use super::CollideEvent;

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



#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Deref, DerefMut)]
pub struct DespawnTimer(Timer);

#[derive(Component)]
pub struct PlayerName(String);

#[derive(Component)]
pub struct EnemyName(String);


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
pub struct HealthBarTop;
#[derive(Component)]
pub struct HealthBarBottom;

pub fn setup_fight(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::DARK_GRAY;	// subject to change

	// spawn the player sprite
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-crate::WIN_W/4., 0., 0.),
            ..default()
        },
        ..default()
    })
    .insert(Velocity::new())
	.insert(Stats::new())
    .insert(Player);

	// spawn a dummy enemy sprite
	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			color: Color::ORANGE_RED,
			custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
			..default()
		},
		transform: Transform {
			translation: Vec3::new(crate::WIN_W/4., 0., 0.),
			..default()
		},
		..default()
	})
	.insert(Velocity::new())
	.insert(Stats::new())
	.insert(Enemy);

	// spawn player health bar
	commands.spawn_bundle(SpriteBundle {
		sprite: Sprite {
			color: Color::LIME_GREEN,
			custom_size: Some(Vec2::new(HEALTHBAR_X, HEALTHBAR_Y)),
			..default()
		},
		transform: Transform {
			translation: Vec3::new( (-crate::WIN_W/2. + HEALTHBAR_X/2.)+16., (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 1.),
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
			translation: Vec3::new( (-crate::WIN_W/2. + HEALTHBAR_X/2.)+16., (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 0.),
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
			translation: Vec3::new( (crate::WIN_W/2. - HEALTHBAR_X/2.)-16., (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 1.),
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
			translation: Vec3::new( (crate::WIN_W/2. - HEALTHBAR_X/2.)-16., (crate::WIN_H/2. - HEALTHBAR_Y/2.)-16., 0.),
			..default()
		},
		..default()
	})
	.insert(HealthBarBottom)
	.insert(EnemyName(String::from("dummy")));
}

//changes the clear color back to black and despawns the character entities,
//might want to move the clear_color change over to setup_credits so it
//doesn't have to rely on other states transitioning correctly
pub fn clear_fight(
    mut commands: Commands,
    mut player: Query<Entity, With<Player>>,
	mut enemy: Query<Entity, With<Enemy>>,
	healthbar_tops: Query<Entity, With<HealthBarTop>>,
	healthbar_bottoms: Query<Entity, With<HealthBarBottom>>,
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

pub fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
	mut player_send: EventWriter<CollideEvent>,
    mut player: Query<(&mut Transform, &mut Velocity), Without<Enemy>>,
	mut enemy: Query<&Transform, With<Enemy>>
) {
    let (mut player_transform, mut player_velocity) = player.single_mut();
	let enemy_transform= enemy.single_mut();

	let mut deltav = Vec2::splat(0.);

	if input.pressed(KeyCode::A) {
		deltav.x -= 1.;
	}

	if input.pressed(KeyCode::D) {
		deltav.x += 1.;
	}

	// player needs to be on the floor to jump, hence the floor height check
	if input.pressed(KeyCode::W) && player_transform.translation.y <= (FLOOR_HEIGHT + PLAYER_H) {
		deltav.y += 1.;
	}
	else {
		deltav.y -= 1.;
	}

	if input.pressed(KeyCode::S) {
		//deltav.y -= 1.;
        //copied from bevy examples, this should make the player crouch and not just move down
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
	} else if deltav.y < 0. {
		//player is falling/not jumping
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
	let collide=check_collision(
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
	if collide{
		if new_pos.x<enemy_transform.translation.x{
			player_send.send(CollideEvent(true,String::from("rightside")));
		}
		if new_pos.x>enemy_transform.translation.x{
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
	 


	// check for player staying within the window with new x position
	//if new_pos.x >= -(crate::WIN_W/2.) + PLAYER_W/2. && new_pos.x <= crate::WIN_W/2. - PLAYER_W/2. {
	//	player_transform.translation = new_pos;
	//}

	let new_pos = player_transform.translation + Vec3::new(
		//changes the new position to FLOOR_HEIGHT + PLAYER_H/2 if it becomes less than that
		0.,
		if change.y + player_transform.translation.y < FLOOR_HEIGHT + PLAYER_H/2.{
			 -1.*player_transform.translation.y + FLOOR_HEIGHT + PLAYER_H/2.
		}else{
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
		 Vec3::new(enemy_transform.translation.x,enemy_transform.translation.y+PLAYER_H/2.,enemy_transform.translation.z),
		//bsize
		Vec2::new(PLAYER_H/2.,PLAYER_W/2.)
	);

	if collide{
		if new_pos.y<enemy_transform.translation.y{
			player_send.send(CollideEvent(true,String::from("topside")));
		}
		if new_pos.y>enemy_transform.translation.y{
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
	
	// check for player staying within the window and above the floor with new y position
	//if new_pos.y >= FLOOR_HEIGHT + PLAYER_H/2. && new_pos.y <= crate::WIN_H/2. - PLAYER_H/2. {
	//	player_transform.translation = new_pos;
	//}
}

// collision_handle system deals with all the collisions and what to do depending on the kind of collision
pub fn collision_handle(
	mut commands: Commands,
	enemy_healthbar_en: Query<Entity, (With<EnemyName>,With<HealthBarTop>)>,
	mut player_receive: EventReader<CollideEvent>,
	mut player: Query<&mut Transform,With<Player>>,
	mut enemy: Query<(&mut Transform, &mut Velocity, &mut Stats), (With<Enemy>, Without<Player>)>
){
	let mut player_transform = player.single_mut();
	let (enemy_transform, mut enemy_velocity, mut enemy_stats) = enemy.single_mut();
		for p in player_receive.iter(){
			if p.0 == true {
				// if the collision is on the right side of the player then just adjust the player x pos so it can't pass through the enemy
				if p.1.contains("rightside"){
				player_transform.translation=player_transform.translation + Vec3::new(
					enemy_transform.translation.x-player_transform.translation.x-PLAYER_W,
					0.,
					0.,
				);
			  }else if p.1.contains("leftside") {
				// if the collision is on the left side of the player then just adjust the player x pos so it can't pass through the enemy
				player_transform.translation=player_transform.translation - Vec3::new(
					player_transform.translation.x-enemy_transform.translation.x-PLAYER_W,
					0.,
					0.,
				);
			  }else if p.1.contains("bottomside") {
				// This is supposed to handle collisions for the bottom side of the player
				// for example if the player jumps on the enemy then there would be a collision on the bottom side of the player
				// It is currently not working because I think the gravity being applied needs to be taken into consideration
				player_transform.translation= Vec3::new(
					player_transform.translation.x,
					-enemy_transform.translation.y + PLAYER_H/2.,
					player_transform.translation.z,
				);
			  }else if p.1.contains("punchleft"){
				// this handles punch collisions 
				// The current entity is despawned and a new entity with updated health, size and pos is spawned 
				enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
					700.,
					0.,
			  	);
				if enemy_stats.health-PUNCHATTACK > 0.{ 
				enemy_stats.health=enemy_stats.health-PUNCHATTACK;
				} else {
					enemy_stats.health=0.;
				}
				let enemy_healthbar_eid=enemy_healthbar_en.single();
				let x_size=5.*enemy_stats.health;
				let x_pos=(crate::WIN_W/2. - 5.*enemy_stats.health/2.)-16.;
				commands.entity(enemy_healthbar_eid).despawn();
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
				if enemy_stats.health==0.{
					// For now this just resets the enemy health
					// In the future we can add code to transition to the next fight or conversation
					enemy_stats.health=110.;
				}
			  }else if p.1.contains("punchright"){
				// this handles punch collisions 
				// The current entity is despawned and a new entity with updated health, size and pos is spawned 
				enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
					-700.,
					0.,
			  	);
				if enemy_stats.health-PUNCHATTACK > 0.{ 
				enemy_stats.health=enemy_stats.health-PUNCHATTACK;
				} else {
					enemy_stats.health=0.;
				}
				let enemy_healthbar_eid=enemy_healthbar_en.single();
				let x_size=5.*enemy_stats.health;
				let x_pos=(crate::WIN_W/2. - 5.*enemy_stats.health/2.)-16.;
				commands.entity(enemy_healthbar_eid).despawn();
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
				if enemy_stats.health==0.{
					// For now this just resets the enemy health
					// In the future we can add code to transition to the next fight or conversation
					enemy_stats.health=110.;
				}
			  }else if p.1.contains("kickleft"){
				enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
					1000.,
					0.,
			  	);
				if enemy_stats.health - KICKATTACK > 0. {
					enemy_stats.health=enemy_stats.health-20.;
					}else{
						enemy_stats.health=0.;
					}
					println!("Enemy health is {}",enemy_stats.health);
					let enemy_healthbar_eid=enemy_healthbar_en.single();
					let x_size=5.*enemy_stats.health;
					let x_pos=(crate::WIN_W/2. - 5.*enemy_stats.health/2.)-16.;
					commands.entity(enemy_healthbar_eid).despawn();
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
					if enemy_stats.health==0.{
						enemy_stats.health=120.;
					}
			  } else if p.1.contains("kickright"){
				enemy_velocity.velocity = enemy_velocity.velocity + Vec2::new(
					-1000.,
					0.,
			  	);
				if enemy_stats.health - KICKATTACK > 0. {
					enemy_stats.health=enemy_stats.health-20.;
					}else{
						enemy_stats.health=0.;
					}
					println!("Enemy health is {}",enemy_stats.health);
					let enemy_healthbar_eid=enemy_healthbar_en.single();
					let x_size=5.*enemy_stats.health;
					let x_pos=(crate::WIN_W/2. - 5.*enemy_stats.health/2.)-16.;
					commands.entity(enemy_healthbar_eid).despawn();
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
					if enemy_stats.health==0.{
						enemy_stats.health=120.;
					}
			  }
			}
		}
}

//doesn't do anything right now other than apply gravity to the enemy
//movement decision-making will come later as a part of AI
pub fn move_enemy(
	time: Res<Time>,
	mut enemy: Query<(&mut Transform, &mut Velocity), With<Enemy>>
) {
	let (mut enemy_transform, mut enemy_velocity) = enemy.single_mut();

	let mut deltav = Vec2::splat(0.);

	// (this is where decision making about movement will go)

	deltav.y -= 1.;	// just make the enemy affected by gravity for now
	
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
	// check for enemy staying within the window with new x position
	if new_pos.x >= -(crate::WIN_W/2.) + PLAYER_W/2. && new_pos.x <= crate::WIN_W/2. - PLAYER_W/2. {
		enemy_transform.translation = new_pos;
	}

	let new_pos = enemy_transform.translation + Vec3::new(
		0.,
		if change.y + enemy_transform.translation.y < FLOOR_HEIGHT + PLAYER_H/2.{
			-1.*enemy_transform.translation.y + FLOOR_HEIGHT + PLAYER_H/2.
	   }else{
		   change.y
	   },
		0.,
	);
	// check for enemy staying within the window and above the floor with new y position
	if new_pos.y >= FLOOR_HEIGHT + PLAYER_H/2. && new_pos.y <= crate::WIN_H/2. - PLAYER_H/2. {
		enemy_transform.translation = new_pos;
	}
}

pub fn attack(
	input: Res<Input<KeyCode>>, 
	mut player_send: EventWriter<CollideEvent>,
	mut player: Query<&mut Transform, With<Player>>,
	mut commands: Commands, 
	mut enemy: Query<&mut Transform, (With<Enemy>, Without<Player>)>,
	
){
    let player_transform = player.single_mut();
	let enemy_transform = enemy.single_mut();
	let mut attack_xpos=60.;
	if player_transform.translation.x>enemy_transform.translation.x{
		 attack_xpos=-60.;
	}
	
	if input.just_released(KeyCode::P)
	&& !input.pressed(KeyCode::D)
	&& !input.pressed(KeyCode::A){

        commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::GREEN,
				custom_size: Some(Vec2::new(80.,32.)),
				..default()
			},
            transform: Transform {
            translation: Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y+32., 0.),
            ..default()
        },
			..default()
		})
        .insert(DespawnTimer(Timer::from_seconds(0.1,false)));
		// The collision function is called to see if a collision occurred
		// if there was a collision a signal is sent to the collision_handle system
		/*if check_collision(
			//apos
			Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y+32., 0.),
			//asize
			Vec2::new(80.,32.),
			//bpos
			Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y+32., 0.),
			//bsize
			Vec2::new(PLAYER_H/2., PLAYER_W/2.)
		) {
			println!("Enemy hit! Current health:");
			player_send.send(CollideEvent(true,String::from("punch")));

		}*/
		let punch_collide_result = collide(
			//apos
			Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y+32., 0.),
			//asize
			Vec2::new(80.,32.),
			//bpos
			Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 0.),
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
	if input.just_released(KeyCode::K)	// having to wait until the key is released feels clunky
	&& !input.pressed(KeyCode::D)	// maybe add a timer so the hitbox lasts for a set time and then you can
	&& !input.pressed(KeyCode::A){	// attack again after a "recovery window"

        commands
		.spawn_bundle(SpriteBundle {
			sprite: Sprite {
				color: Color::GREEN,
				custom_size: Some(Vec2::new(80.,32.)),
				..default()
			},
            transform: Transform {
            translation: Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y-32., 0.1),
            ..default()
        },
			..default()
		})
        .insert(DespawnTimer(Timer::from_seconds(0.1,false)));
		/*if check_collision(
			//apos
			Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y-32., 0.),
			//asize
			Vec2::new(80.,32.),
			//bpos
			Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y-32., 0.),
			//bsize
			Vec2::new(PLAYER_H/2., PLAYER_W/2.)
		){
			//enemy_stats.health -= 10.;
			println!("Enemy hit! Current health:");
			player_send.send(CollideEvent(true,String::from("kick")));

		}*/
		let kick_collide_result = collide(
			//apos
			Vec3::new(player_transform.translation.x+attack_xpos, player_transform.translation.y-32., 0.),
			//asize
			Vec2::new(80.,32.),
			//bpos
			Vec3::new(enemy_transform.translation.x, enemy_transform.translation.y, 0.),
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

pub fn remove_popup(
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