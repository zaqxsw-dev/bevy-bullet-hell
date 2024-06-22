use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
	fn build(&self, app: &mut App) {
		app.add_loading_state(
			LoadingState::new(GameState::Loading)
				.continue_to_state(GameState::Menu)
				.load_collection::<FontAssets>()
				.load_collection::<AudioAssets>()
				.load_collection::<TextureAssets>(),
		);
	}
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
	#[asset(path = "fonts/light_pixel-7.ttf")]
	pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
	#[asset(path = "audio/flying.ogg")]
	pub flying: Handle<AudioSource>,
	#[asset(path = "audio/grass_steps.ogg")]
	pub grass_steps: Handle<AudioSource>,
	#[asset(path = "audio/main_menu_audio.ogg")]
	pub main_menu: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
	#[asset(path = "textures/bevy.png")]
	pub texture_bevy: Handle<Image>,
	#[asset(path = "textures/player.png")]
	pub player: Handle<Image>,
	#[asset(path = "textures/enemy.png")]
	pub enemy: Handle<Image>,
	#[asset(path = "textures/heart.png")]
	pub heart: Handle<Image>,
}
