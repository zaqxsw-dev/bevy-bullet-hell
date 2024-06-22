mod actions;
mod audio;
pub mod components;
pub mod constants;
mod loading;
mod menu;
pub mod player;
pub mod plugins;
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
use constants::{BASE_SPEED, TIME_STEP};
use plugins::despawner::Despawner;
use plugins::enemy::EnemySpawnPlugin;
use plugins::gameover::GameOverPlugin;
use ui::damage::DamageHintPlugin;
use ui::exp::PlayerExpBar;
use ui::health::PlayerHealthBar;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
	#[default]
	Loading,
	Playing,
	Upgrade,
	Gameover,
	Menu,
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum MenuState {
	#[default]
	Menu,
	Disabled,
}

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
pub struct SceneObject;

#[derive(Component)]
pub struct Enemy {
	damage: i32,
	kill_exp: u32,
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
		app.init_state::<GameState>().init_state::<MenuState>().add_plugins((
			LoadingPlugin,
			MenuPlugin,
			GameOverPlugin,
			ActionsPlugin,
			InternalAudioPlugin,
			EnemySpawnPlugin,
			PlayerPlugin,
			PlayerHealthBar,
			PlayerExpBar,
			DamageHintPlugin,
			Despawner,
		));

		#[cfg(debug_assertions)]
		{
			app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
		}
	}
}

fn movable_system(mut query: Query<(&Velocity, &mut Transform), With<Bullet>>) {
	for (velocity, mut transform) in query.iter_mut() {
		let translation = &mut transform.translation;
		translation.x += velocity.x * TIME_STEP * BASE_SPEED;
		translation.y += velocity.y * TIME_STEP * BASE_SPEED;
	}
}
