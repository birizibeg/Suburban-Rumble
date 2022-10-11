use bevy::{
    prelude::*
};

const PLAYER_W: f32 = 32.;
const PLAYER_H: f32 = 64.;
const PLAYER_SPEED: f32 = 500.; // play around with these values to make movement feel right for a fighting game
const ACCEL_RATE: f32 = 4000.;  // I just copied them over from the examples

#[derive(Component)]
pub struct Player;

// use a velocity component to track the player's velocity
#[derive(Component)]
pub struct Velocity {
	velocity: Vec2,
}

impl Velocity {
	fn new() -> Self {
		Self { velocity: Vec2::splat(0.) }
	}
}

//as of right now, just changes the clear color and spawns a player sprite
pub fn setup_fight(mut commands: Commands, mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = Color::DARK_GRAY;

    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLUE,
            custom_size: Some(Vec2::new(PLAYER_W, PLAYER_H)),
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(-crate::WIN_W/4., -crate::WIN_H/4., 0.),
            ..default()
        },
        ..default()
    })
    .insert(Velocity::new())
    .insert(Player);
}

//changes the clear color back to black and despawns the player entity,
//might want to move the clear_color change over to setup_credits so it
//doesn't have to rely on other states transitioning correctly
pub fn clear_fight(
    mut commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    mut player: Query<Entity, With<Player>>
) {
    clear_color.0 = Color::BLACK;

    let player_eid = player.single_mut();
    commands.entity(player_eid).despawn();
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

	if input.pressed(KeyCode::W) {
		//deltav.y += 1.;
        //copied from bevy examples, this should make the player jump and not just move up
        //use a "gravity" acceleration value to make a jump arc?
	}

	if input.pressed(KeyCode::S) {
		//deltav.y -= 1.;
        //copied from bevy examples, this should make the player crouch and not just move down
	}

	// calculating by deltat instead of just relying on frames *should* normalize for different framerates
	let deltat = time.delta_seconds();
	let acc = ACCEL_RATE * deltat;

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
		change.x,
		0.,
		0.,
	);
	// check for player staying within the window with new x position
	if new_pos.x >= -(crate::WIN_W/2.) + PLAYER_W/2. && new_pos.x <= crate::WIN_W/2. - PLAYER_W/2. {
		player_transform.translation = new_pos;
	}

    //this shouldn't be used right now but I copied it over from the bevy examples anyways
	let new_pos = player_transform.translation + Vec3::new(
		0.,
		change.y,
		0.,
	);
	// check for player staying within the window with new y position
	if new_pos.y >= -(crate::WIN_H/2.) + PLAYER_H/2. && new_pos.y <= crate::WIN_H/2. - PLAYER_H/2. {
		player_transform.translation = new_pos;
	}
}