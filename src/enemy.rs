use crate::{
	loading::TextureAssets,
	player::{Player, PlayerGetExpEvent},
	Enemy, GameData, GameState, Killable, Velocity,
};
use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub struct EnemySpawnPlugin;

impl Plugin for EnemySpawnPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				enemy_move_system.run_if(in_state(GameState::Playing)),
				enemy_despawn_system.run_if(in_state(GameState::Playing)),
				enemy_spawn_system.run_if(in_state(GameState::Playing)),
			),
		);
	}
}

fn enemy_spawn_system(
	mut commands: Commands,
	textures: Res<TextureAssets>,
	mut spawn_opt: ResMut<GameData>,
	time: Res<Time>,
	player_query: Query<&mut Transform, With<Player>>,
) {
	spawn_opt.enemy_spawn_timer.tick(time.delta());
	if spawn_opt.enemy_spawn_timer.finished() {
		if let Ok(player) = player_query.get_single() {
			let mut rng = thread_rng();

			let mut translation = Vec3 {
				x: rng.gen_range(-1.0..1.0),
				y: rng.gen_range(-1.0..1.0),
				z: 0.0,
			};
			translation = translation.normalize() * rng.gen_range(0.2f32..1.0).cbrt() * 400.;
			translation.x = translation.x + player.translation.x;
			translation.y = translation.y + player.translation.y;

			commands
				.spawn(SpriteBundle {
					transform: Transform {
						translation,
						scale: Vec3::new(0.35, 0.35, 0.),
						..Default::default()
					},
					texture: textures.enemy.clone(),
					..Default::default()
				})
				.insert(Killable {
					hp: 3,
					god_mode: false,
					hp_max: 3,
				})
				.insert(Velocity {
					x: 0.5,
					y: 0.5,
					speed: 30.0,
				})
				.insert(Enemy { damage: 1 });
		}
	}
}

fn enemy_move_system(
	time: Res<Time>,
	player_query: Query<&Transform, With<Player>>,
	mut enemy_query: Query<(&mut Velocity, &mut Transform), (With<Enemy>, Without<Player>)>,
) {
	if let Ok(player) = player_query.get_single() {
		for (mut velocity, mut enemy) in &mut enemy_query.iter_mut() {
			let diff = player.translation.truncate() - enemy.translation.truncate();
			velocity.x = diff.normalize().x;
			velocity.y = diff.normalize().y;

			enemy.translation.x += velocity.x * time.delta().as_secs_f32() * velocity.speed;
			enemy.translation.y += velocity.y * time.delta().as_secs_f32() * velocity.speed;
		}
	}
}

fn enemy_despawn_system(
	mut commands: Commands,
	query: Query<(Entity, &Killable), With<Enemy>>,
	mut exp_event: EventWriter<PlayerGetExpEvent>,
) {
	for (entity, killable) in query.iter() {
		if killable.hp <= 0 {
			exp_event.send(PlayerGetExpEvent { exp: 15 });
			commands.entity(entity).despawn();
		}
	}
}
