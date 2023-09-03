mod actions;
mod audio;
mod enemy;
mod loading;
mod menu;
pub mod player;
mod skill;
mod ui;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use enemy::EnemySpawnPlugin;
use player::Player;
use ui::damage::DamageHintPlugin;
use ui::exp::PlayerExpBar;
use ui::health::PlayerHealthBar;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
	#[default]
	Loading,
	Playing,
	Upgrade,
	Menu,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
	#[default]
	Menu,
	Disabled,
}

pub const DESPAWN_BULLET_DISTANCE: f32 = 2000.0;
pub const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 400.;

#[derive(Default, Resource)]
pub struct Mouse {
	pub position: Vec2,
	pub area: Vec2,
}

#[derive(Resource)]
pub struct GameData {
	pub enemy_spawn_timer: Timer,
	pub player_shooting_timer: Timer,
	pub player_godmod_timer: Timer,
}

impl Default for GameData {
	fn default() -> Self {
		Self {
			enemy_spawn_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
			player_shooting_timer: Timer::from_seconds(1.0, TimerMode::Once),
			player_godmod_timer: Timer::from_seconds(1.0, TimerMode::Once),
		}
	}
}

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Enemy {
	damage: i32,
}

#[derive(Component)]
pub struct Bullet {
	damage: i32,
}

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

#[derive(Component)]
pub struct Movable {
	pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Killable {
	pub hp: i32,
	pub god_mode: bool,
	pub hp_max: i32,
}

#[derive(Component)]
pub struct Velocity {
	pub x: f32,
	pub y: f32,
	pub speed: f32,
}

#[derive(Component)]
pub struct FromPlayer;

pub struct GamePlugin;

impl Plugin for GamePlugin {
	fn build(&self, app: &mut App) {
		app.add_state::<GameState>()
			.add_state::<MenuState>()
			.add_plugin(LoadingPlugin)
			.add_plugin(MenuPlugin)
			.add_plugin(ActionsPlugin)
			.add_plugin(InternalAudioPlugin)
			.add_plugin(EnemySpawnPlugin)
			.add_plugin(PlayerPlugin)
			.add_plugin(PlayerHealthBar)
			.add_plugin(PlayerExpBar)
			.add_plugin(DamageHintPlugin);

		#[cfg(debug_assertions)]
		{
			app.add_plugin(FrameTimeDiagnosticsPlugin::default())
				.add_plugin(LogDiagnosticsPlugin::default());
		}
	}
}

fn movable_system(
	mut commands: Commands,
	//cursor: Res<Mouse>,
	mut query: Query<(Entity, &Velocity, &mut Transform, &Movable), With<Bullet>>,
	player_q: Query<&Transform, (With<Player>, Without<Bullet>)>,
) {
	if let Ok(player) = player_q.get_single() {
		for (entity, velocity, mut transform, movable) in query.iter_mut() {
			let translation = &mut transform.translation;
			translation.x += velocity.x * TIME_STEP * BASE_SPEED;
			translation.y += velocity.y * TIME_STEP * BASE_SPEED;

			if movable.auto_despawn {
				let distance = translation.distance(player.translation);
				if DESPAWN_BULLET_DISTANCE < distance {
					commands.entity(entity).despawn_recursive();
				}
				//const MARGIN: f32 = 200.;
				//if translation.y > cursor.area.y / 2. + MARGIN
				//	|| translation.y < -cursor.area.y / 2. - MARGIN
				//	|| translation.x > cursor.area.x / 2. + MARGIN
				//	|| translation.x < -cursor.area.x / 2. - MARGIN
				//{
				//	commands.entity(entity).despawn();
				//}
			}
		}
	}
}
