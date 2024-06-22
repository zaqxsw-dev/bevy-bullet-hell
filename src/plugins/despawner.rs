use bevy::prelude::*;

use crate::{
	components::killable::Killable,
	constants::DESPAWN_BULLET_DISTANCE,
	player::{Player, PlayerGetExpEvent},
	Bullet, Enemy, GameState, Movable,
};

pub struct Despawner;

impl Plugin for Despawner {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, despawn_system.run_if(in_state(GameState::Playing)));
	}
}

fn despawn_system(
	mut commands: Commands,
	mut query: Query<(Entity, &Transform, &Movable), With<Bullet>>,
	player_q: Query<&Transform, With<Player>>,
	killable_query: Query<(Entity, &Killable, &Enemy), With<Enemy>>,
	mut exp_event: EventWriter<PlayerGetExpEvent>,
) {
	if let Ok(player) = player_q.get_single() {
		for (entity, transform, movable) in query.iter_mut() {
			if movable.auto_despawn {
				let distance = transform.translation.distance(player.translation);
				if DESPAWN_BULLET_DISTANCE < distance {
					commands.entity(entity).despawn_recursive();
					println!("despawn_recursive");
				}
			}
		}
		for (entity, killable, enemy) in killable_query.iter() {
			if killable.hp <= 0 {
				exp_event.send(PlayerGetExpEvent { exp: enemy.kill_exp });
				commands.entity(entity).despawn_recursive();
			}
		}
	}
}
