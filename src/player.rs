use crate::actions::Actions;
use crate::components::killable::Killable;
use crate::loading::TextureAssets;
use crate::ui::damage::EventDamageHintSpawn;
use crate::{
	movable_system, Bullet, Enemy, FromPlayer, GameData, GameState, MainCamera, Mouse,
	Movable, SceneObject, SpriteSize, Velocity,
};
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::math::primitives::Circle;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_hanabi::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
	pub exp: u32,
	pub next_lvl_exp: u32,
	pub lvl: u32,
}

impl Default for Player {
	fn default() -> Self {
		Self {
			exp: 0,
			next_lvl_exp: get_lvl_exp(1),
			lvl: 1,
		}
	}
}

#[derive(Component)]
pub struct PlayerMove;

#[derive(Component)]
pub struct DodgeRoll {
	pub timer: Timer,
	pub direction: Vec2,
}

#[derive(Event)]
pub struct PlayerGetDamageEvent {
	damage: i32,
}

#[derive(Event)]
pub struct PlayerGetExpEvent {
	pub exp: u32,
}

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<PlayerGetDamageEvent>()
			.add_event::<PlayerGetExpEvent>()
			.add_systems(OnEnter(GameState::Playing), spawn_player)
			.add_systems(
				Update,
				(
					move_player.run_if(in_state(GameState::Playing)),
					camera_move.run_if(in_state(GameState::Playing)),
					dodge_roll_system.run_if(in_state(GameState::Playing)),
					turn_player.run_if(in_state(GameState::Playing)),
					player_fire_system.run_if(in_state(GameState::Playing)),
					movable_system.run_if(in_state(GameState::Playing)),
					player_damage_system.run_if(in_state(GameState::Playing)),
					player_bullet_hit_system.run_if(in_state(GameState::Playing)),
					get_player_damage_event.run_if(in_state(GameState::Playing)),
					get_player_exp_event.run_if(in_state(GameState::Playing)),
				),
			);
	}
}

fn dodge_roll_system(
	mut commands: Commands,
	mut query: Query<(Entity, &mut Transform, Option<&mut DodgeRoll>), With<PlayerMove>>,
	//player_query: Query<&Transform, With<PlayerMove>>,
	time: Res<Time>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
) {
	for (entity, mut transform, dodge_roll) in query.iter_mut() {
		if let Some(mut roll) = dodge_roll {
			roll.timer.tick(time.delta());

			if roll.timer.finished() {
				commands.entity(entity).remove::<DodgeRoll>();
			} else {
				let roll_direction = Vec3::new(roll.direction.x, roll.direction.y, 0.0);
				transform.translation += roll_direction * 500.0 * time.delta_seconds();
			}
		} else if keyboard_input.just_pressed(KeyCode::ShiftLeft) {
			let mut roll_direction = Vec2::ZERO;

			if keyboard_input.pressed(KeyCode::KeyW) {
				roll_direction.y += 1.0;
			}
			if keyboard_input.pressed(KeyCode::KeyS) {
				roll_direction.y -= 1.0;
			}
			if keyboard_input.pressed(KeyCode::KeyA) {
				roll_direction.x -= 1.0;
			}
			if keyboard_input.pressed(KeyCode::KeyD) {
				roll_direction.x += 1.0;
			}

			if roll_direction.length() > 0.0 {
				roll_direction = roll_direction.normalize();
				commands.entity(entity).insert(DodgeRoll {
					timer: Timer::from_seconds(0.3, TimerMode::Once), // Roll duration
					direction: roll_direction,
				});
			}
		}
	}
}

fn get_lvl_exp(lvl: u32) -> u32 {
	match lvl {
		1 => 100,
		2 => 200,
		3 => 400,
		4 => 800,
		5 => 1600,
		6 => 3200,
		7 => 6400,
		8 => 12800,
		9 => 25600,
		10 => 51200,
		11 => 102400,
		12 => 204800,
		13 => 409600,
		14 => 819200,
		15 => 819200 * 2,
		_ => 100,
	}
}

fn get_player_damage_event(
	mut ev_pdamage: EventReader<PlayerGetDamageEvent>,
	mut query: Query<&mut Killable, With<Player>>,
	mut game_state: ResMut<NextState<GameState>>,
) {
	if let Ok(mut player) = query.get_single_mut() {
		for ev in ev_pdamage.read() {
			player.hit(ev.damage);
		}
		if player.hp <= 0 {
			game_state.set(GameState::Gameover)
		}
	}
}

fn get_player_exp_event(
	mut event: EventReader<PlayerGetExpEvent>,
	mut query: Query<&mut Player>,
) {
	let mut player = match query.get_single_mut() {
		Ok(val) => val,
		Err(_) => return,
	};
	for ev in event.read() {
		player.exp += ev.exp;

		if player.exp >= player.next_lvl_exp {
			player.lvl += 1;
			player.exp -= player.next_lvl_exp;
			player.next_lvl_exp = get_lvl_exp(player.lvl);
		}
	}
}

fn spawn_player(
	mut commands: Commands,
	textures: Res<TextureAssets>,
	images: Res<Assets<Image>>,
	query: Query<Entity, With<Player>>,
	mut effects: ResMut<Assets<EffectAsset>>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	if query.iter().count() > 0 {
		return;
	}
	let image = images.get(&textures.player).unwrap();
	let player_sprite = SpriteBundle {
		texture: textures.player.clone(),
		transform: Transform {
			translation: Vec3::new(0., 0., 1.),
			rotation: Quat::default(),
			scale: Vec3::new(0.25, 0.25, 1.),
		},
		..Default::default()
	};
	commands
		.spawn(player_sprite)
		.insert(Killable {
			hp: 10,
			god_mode: false,
			hp_max: 10,
		})
		.insert(SpriteSize(image.size_f32()))
		.insert(PlayerMove)
		.insert(SceneObject)
		.insert(Player::default());

	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes
				.add(Mesh::from(bevy::math::primitives::Rectangle::new(1., 1.)))
				.into(),
			material: materials.add(ColorMaterial {
				color: Color::PURPLE,
				..Default::default()
			}),
			..Default::default()
		})
		.insert(Name::new("square"));

	let mut gradient = Gradient::new();
	gradient.add_key(0.0, Vec4::new(0.5, 0.5, 1.0, 1.0));
	gradient.add_key(1.0, Vec4::new(0.5, 0.5, 1.0, 0.2));

	let writer = ExprWriter::new();
	let lifetime = writer.lit(1.).expr();
	let age = writer.lit(0.).expr();
	let init_age = SetAttributeModifier::new(Attribute::AGE, age);
	let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);
	let init_pos_circle = SetPositionCircleModifier {
		center: writer.lit(Vec3::ZERO).expr(),
		axis: writer.lit(Vec3::Z).expr(),
		radius: writer.lit(25.00).expr(),
		dimension: ShapeDimension::Surface,
	};
	let init_vel = SetVelocityCircleModifier {
		center: writer.lit(Vec3::ZERO).expr(),
		axis: writer.lit(Vec3::Z).expr(),
		speed: writer.lit(15.0).expr(),
	};

	let effect_asset =
		EffectAsset::new(vec![4000], Spawner::rate(200.0.into()), writer.finish())
			.with_name("Effect")
			.init(init_pos_circle)
			.init(init_vel)
			.init(init_age)
			.init(init_lifetime)
			//.update(update_drag)
			//.update(tangent_accel)
			.render(ColorOverLifetimeModifier { gradient })
			.render(SizeOverLifetimeModifier {
				gradient: Gradient::constant(Vec2::splat(1.0)),
				screen_space_size: false,
			});

	let effect = effects.add(effect_asset);
	commands
		.spawn(ParticleEffectBundle {
			effect: ParticleEffect::new(effect).with_z_layer_2d(Some(0.2)),
			transform: Transform::IDENTITY,
			..default()
		})
		.insert(SceneObject)
		.insert(PlayerMove);
}

fn player_damage_system(
	enemy_query: Query<(&Transform, &Enemy)>,
	mut player_query: Query<(&Transform, &Killable, &SpriteSize), With<Player>>,
	mut event: EventWriter<PlayerGetDamageEvent>,
	mut game_data: ResMut<GameData>,
	time: Res<Time>,
) {
	game_data.player_godmod_timer.tick(time.delta());
	for (enemy_transform, enemy) in enemy_query.iter() {
		for (player_transform, killable, sprite) in player_query.iter_mut() {
			if killable.god_mode || !game_data.player_godmod_timer.finished() {
				return;
			}
			let collision = Aabb2d::new(
				enemy_transform.translation.truncate(),
				enemy_transform.scale.truncate() / 2.,
			)
			.intersects(&Aabb2d::new(
				player_transform.translation.truncate(),
				sprite.0 * player_transform.scale.truncate() / 2.,
			));

			if collision {
				game_data.player_godmod_timer.reset();
				event.send(PlayerGetDamageEvent { damage: enemy.damage });
			}
		}
	}
}

fn player_bullet_hit_system(
	mut commands: Commands,
	mut enemy_query: Query<(&Transform, &mut Killable), With<Enemy>>,
	mut bullet_query: Query<(Entity, &Transform, &Bullet), With<Bullet>>,
	mut damage_hint_event: EventWriter<EventDamageHintSpawn>,
) {
	for (bullet_entity, bullet_transform, bullet) in bullet_query.iter_mut() {
		for (enemy_transform, mut killable) in enemy_query.iter_mut() {
			let collision = Aabb2d::new(
				bullet_transform.translation.truncate(),
				bullet_transform.scale.truncate(),
			)
			.intersects(&Aabb2d::new(
				enemy_transform.translation.truncate(),
				Vec2 { x: 25., y: 25. },
			));

			if collision {
				killable.hp -= bullet.damage;
				damage_hint_event.send(EventDamageHintSpawn {
					damage: bullet.damage as u32,
					position: enemy_transform.translation.truncate(),
				});
				commands.entity(bullet_entity).despawn();
			}
		}
	}
}

fn player_fire_system(
	mut commands: Commands,
	kb: Res<ButtonInput<KeyCode>>,
	buttons: Res<ButtonInput<MouseButton>>,
	query: Query<&Transform, (With<Player>, With<Killable>)>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mouse: Res<Mouse>,
	mut game_data: ResMut<GameData>,
	time: Res<Time>,
) {
	game_data.player_shooting_timer.tick(time.delta());
	if let Ok(player_tf) = query.get_single() {
		if game_data.player_shooting_timer.finished()
			&& (kb.pressed(KeyCode::Space) || buttons.pressed(MouseButton::Left))
		{
			game_data.player_shooting_timer.reset();
			let (x, y) = (player_tf.translation.x, player_tf.translation.y);
			let p_transform = Vec2 { x, y };
			let target = Vec2 {
				x: mouse.position.x,
				y: mouse.position.y,
			};
			let diff = target - p_transform;
			let x_offset = 0.;

			let mut spawn_laser = |x_offset: f32| {
				commands
					.spawn(MaterialMesh2dBundle {
						mesh: meshes.add(Circle::default()).into(),
						material: materials.add(ColorMaterial::from(Color::RED)),
						transform: Transform::from_translation(Vec3::new(
							x + x_offset,
							y + 0.,
							0.,
						))
						.with_scale(Vec3 { x: 10.0, y: 10.0, z: 0.0 }),
						..default()
					})
					.insert(Bullet { damage: 2 })
					.insert(SceneObject)
					.insert(FromPlayer)
					.insert(Movable { auto_despawn: true })
					.insert(Velocity {
						x: diff.normalize().x,
						y: diff.normalize().y,
						speed: 50.0,
					});
			};

			spawn_laser(x_offset);
		}
	}
}

fn camera_move(
	actions: Res<Actions>,
	player_query: Query<&Transform, With<PlayerMove>>,
	mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<PlayerMove>)>,
) {
	if actions.player_movement.is_none() {
		return;
	}
	for player_transform in &player_query {
		for mut camera in &mut camera_query {
			camera.translation = player_transform.translation;
		}
	}
}

fn move_player(
	time: Res<Time>,
	actions: Res<Actions>,
	mut player_query: Query<&mut Transform, With<PlayerMove>>,
	//mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<PlayerMove>)>,
) {
	if actions.player_movement.is_none() {
		return;
	}
	let speed = 100.;
	let movement = Vec3::new(
		actions.player_movement.unwrap().x * speed * time.delta_seconds(),
		actions.player_movement.unwrap().y * speed * time.delta_seconds(),
		0.,
	);
	for mut player_transform in &mut player_query {
		player_transform.translation += movement;
	}
	//for mut camera in &mut camera_query {
	//	camera.translation += movement;
	//}
}

fn turn_player(mouse: Res<Mouse>, mut player_query: Query<&mut Transform, With<PlayerMove>>) {
	for mut player_transform in &mut player_query {
		let p_transform = Vec2 {
			x: player_transform.translation.x,
			y: player_transform.translation.y,
		};
		let target = Vec2 {
			x: mouse.position.x,
			y: mouse.position.y,
		};
		let diff = target - p_transform;
		let angle = diff.y.atan2(diff.x) - f32::to_radians(90.0);
		if angle.is_nan() {
			continue;
		}
		player_transform.rotation = Quat::from_rotation_z(angle);
	}
}
