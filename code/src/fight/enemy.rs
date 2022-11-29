use bevy::{
    prelude::*
};
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;
use super::CollideEvent;
use super::FightWinEvent;

const ENEMY_W: f32 = super::PLAYER_W;
const ENEMY_H: f32 = super::PLAYER_H;
const ENEMY_SPEED: f32 = super::PLAYER_SPEED;