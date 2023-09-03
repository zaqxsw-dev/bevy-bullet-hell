use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::ui::damage::EventDamageHintSpawn;
use crate::{
	movable_system, Bullet, Enemy, FromPlayer, GameData, GameState, Killable, MainCamera,
	Mouse, Movable, SpriteSize, Velocity,
};
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
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

pub struct PlayerGetDamageEvent {
	damage: i32,
}

pub struct PlayerGetExpEvent {
	pub exp: u32,
}

impl Plugin for PlayerPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<PlayerGetDamageEvent>()
			.add_event::<PlayerGetExpEvent>()
			.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
			.add_system(move_player.in_set(OnUpdate(GameState::Playing)))
			.add_system(turn_player.in_set(OnUpdate(GameState::Playing)))
			.add_system(player_fire_system.in_set(OnUpdate(GameState::Playing)))
			.add_system(movable_system.in_set(OnUpdate(GameState::Playing)))
			.add_system(player_damage_system.in_set(OnUpdate(GameState::Playing)))
			.add_system(player_bullet_hit_system.in_set(OnUpdate(GameState::Playing)))
			.add_system(get_player_damage_event.in_set(OnUpdate(GameState::Playing)))
			.add_system(get_player_exp_event.in_set(OnUpdate(GameState::Playing)));
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
) {
	let mut player = match query.get_single_mut() {
		Ok(val) => val,
		Err(_) => return,
	};
	for ev in ev_pdamage.iter() {
		player.hp -= ev.damage;
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
	for ev in event.iter() {
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
		.insert(SpriteSize(image.size()))
		.insert(PlayerMove)
		.insert(Player::default());

	commands
		.spawn(MaterialMesh2dBundle {
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

	let spawner = Spawner::rate(100.0.into());
	let effect = effects.add(
		EffectAsset {
			name: "Effect".into(),
			capacity: 1000,
			spawner,
			..Default::default()
		}
		.init(InitPositionCircleModifier {
			center: Vec3::ZERO,
			axis: Vec3::Z,
			radius: 30.0,
			dimension: ShapeDimension::Surface,
		})
		.init(InitVelocityCircleModifier {
			center: Vec3::ZERO,
			axis: Vec3::Z,
			speed: 50.0.into(),
		})
		.init(InitLifetimeModifier { lifetime: 1_f32.into() })
		.render(SizeOverLifetimeModifier {
			gradient: Gradient::constant(Vec2::splat(1.0)),
		})
		.render(ColorOverLifetimeModifier { gradient }),
	);
	commands
		.spawn(ParticleEffectBundle {
			effect: ParticleEffect::new(effect).with_z_layer_2d(Some(10.0)),
			..default()
		})
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
			let collision = collide(
				enemy_transform.translation,
				enemy_transform.scale.truncate(),
				player_transform.translation,
				sprite.0 * player_transform.scale.truncate(),
			);

			if let Some(_) = collision {
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
			let collision = collide(
				bullet_transform.translation,
				bullet_transform.scale.truncate(),
				enemy_transform.translation,
				Vec2 { x: 25.0, y: 25.0 },
			);

			if let Some(_) = collision {
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
	kb: Res<Input<KeyCode>>,
	buttons: Res<Input<MouseButton>>,
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
			println!("shot");
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
						mesh: meshes.add(shape::Circle::default().into()).into(),
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

//fn particles(time: Res<Time>, mut query: Query<&mut Transform, With<Mesh2dHandle>>) {
//	//Move the plane back and forth to show particles ordering relative to it
//	let mut transform = query.single_mut();
//	transform.translation.z = (time.elapsed_seconds() * 2.5).sin() * 0.045 + 0.1;
//}

fn move_player(
	time: Res<Time>,
	actions: Res<Actions>,
	mut player_query: Query<&mut Transform, With<PlayerMove>>,
	mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<PlayerMove>)>,
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
	for mut camera in &mut camera_query {
		camera.translation += movement;
	}
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
