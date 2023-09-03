use crate::actions::{set_movement_actions, Actions};
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugin(AudioPlugin)
			.add_audio_channel::<MenuAudio>()
			.add_system(start_audio.in_schedule(OnEnter(GameState::Menu)))
			.add_system(start_menu_audio.in_schedule(OnEnter(GameState::Menu)))
			.add_system(stop_menu_audio.in_schedule(OnExit(GameState::Menu)))
			.add_system(
				control_flying_sound
					.after(set_movement_actions)
					.in_set(OnUpdate(GameState::Playing)),
			);
	}
}

#[derive(Resource)]
struct MenuAudio(Handle<AudioInstance>);

fn start_menu_audio(background: Res<AudioChannel<MenuAudio>>, audio_assets: Res<AudioAssets>) {
	background.stop();
	background.play(audio_assets.main_menu.clone()).looped().with_volume(0.8);
}

fn stop_menu_audio(background: Res<AudioChannel<MenuAudio>>) {
	background.stop();
}

#[derive(Resource)]
struct FlyingAudio(Handle<AudioInstance>);

fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
	audio.pause();
	let handle =
		audio.play(audio_assets.grass_steps.clone()).looped().with_volume(0.3).handle();
	commands.insert_resource(FlyingAudio(handle));

	//let handle = audio.play(audio_assets.main_menu.clone()).looped().with_volume(0.8).handle();
	//commands.insert_resource(MenuAudio(handle));
}

fn control_flying_sound(
	actions: Res<Actions>,
	audio: Res<FlyingAudio>,
	mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
	if let Some(instance) = audio_instances.get_mut(&audio.0) {
		match instance.state() {
			PlaybackState::Paused { .. } => {
				if actions.player_movement.is_some() {
					instance.resume(AudioTween::default());
				}
			}
			PlaybackState::Playing { .. } => {
				if actions.player_movement.is_none() {
					instance.pause(AudioTween::default());
				}
			}
			_ => {}
		}
	}
}
