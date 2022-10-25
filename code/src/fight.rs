use bevy::{
    prelude::*
};

const PLAYER_W: f32 = 64.;
const PLAYER_H: f32 = 128.;
const FLOOR_HEIGHT: f32 = -crate::WIN_H/4.;
const PLAYER_SPEED: f32 = 500.; // play around with these values to make movement feel right for a fighting game
const ACCEL_RATE: f32 = 5000.;  
const GRAVITY: f32 = 2000.;
const HEALTHBAR_X: f32 = 5.*100.;
const HEALTHBAR_Y: f32 = 32.;

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
    mut clear_color: ResMut<ClearColor>,
    mut player: Query<Entity, With<Player>>,
	mut enemy: Query<Entity, With<Enemy>>
) {
    clear_color.0 = Color::BLACK;

    let player_eid = player.single_mut();
    commands.entity(player_eid).despawn();
	let enemy_eid = enemy.single_mut();
	commands.entity(enemy_eid).despawn();
}

pub fn move_player(
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>
) {
    let (mut player_transform, mut player_velocity) = player.single_mut();

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
	// check for player staying within the window with new x position
	if new_pos.x >= -(crate::WIN_W/2.) + PLAYER_W/2. && new_pos.x <= crate::WIN_W/2. - PLAYER_W/2. {
		player_transform.translation = new_pos;
	}

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
	// check for player staying within the window and above the floor with new y position
	if new_pos.y >= FLOOR_HEIGHT + PLAYER_H/2. && new_pos.y <= crate::WIN_H/2. - PLAYER_H/2. {
		player_transform.translation = new_pos;
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

pub fn attack(input: Res<Input<KeyCode>>, mut player: Query<&Transform, With<Player>>,mut commands: Commands){
    let player_transform = player.single_mut();
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
            translation: Vec3::new(player_transform.translation.x+60., player_transform.translation.y+32., 0.1),
            ..default()
        },
			..default()
		})
        .insert(DespawnTimer(Timer::from_seconds(0.1,false)));
    }
    if input.just_released(KeyCode::K)
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
            translation: Vec3::new(player_transform.translation.x+60., player_transform.translation.y-32., 0.1),
            ..default()
        },
			..default()
		})
        .insert(DespawnTimer(Timer::from_seconds(0.1,false)));
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
pub fn apply_gravity(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut Velocity), With<Player>>
){
  
    let (mut player_transform, mut player_velocity) = player.single_mut();

	let mut deltav = Vec2::splat(0.);
    deltav.y -= 1.;
    let deltat = time.delta_seconds();
	let acc = GRAVITY * deltat;

	// calculate the velocity vector by multiplying it with the acceleration constant
	player_velocity.velocity = if deltav.length() > 0. {
		(player_velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
	}
	else if player_velocity.velocity.length() > acc {
		player_velocity.velocity + (player_velocity.velocity.normalize_or_zero() * -acc)
	}
	else {
		Vec2::splat(0.)
	};
	let change = player_velocity.velocity * deltat;

    let new_pos = player_transform.translation + Vec3::new(
		0.,
		change.y,
		0.,
	);
    
    if new_pos.y >= FLOOR_HEIGHT + PLAYER_H/2. {
		player_transform.translation = new_pos;
	}

}

