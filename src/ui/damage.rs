use bevy::prelude::*;

use crate::{loading::FontAssets, GameState};

const HINT_SPEED: f32 = 20.0;
const HINT_TIME: f32 = 1.0;

pub struct DamageHintPlugin;

#[derive(Component)]
pub struct DamageHint {
	damage: u32,
	timer: Timer,
}

pub struct EventDamageHintSpawn {
	pub damage: u32,
	pub position: Vec2,
}

impl Plugin for DamageHintPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<EventDamageHintSpawn>()
			.add_system(spawn_damage_hint.in_set(OnUpdate(GameState::Playing)))
			.add_system(damage_hint_animation.in_set(OnUpdate(GameState::Playing)));
	}
}

fn get_text_element(damage: u32, fa: Handle<Font>, transparency: f32) -> Text {
	let style = TextStyle {
		font: fa,
		font_size: 12.0,
		color: Color::rgba(0.98, 0.92, 0.84, transparency),
	};
	let alignment = TextAlignment::Center;
	Text::from_section(format!("{damage}"), style).with_alignment(alignment)
}

fn spawn_damage_hint(
	mut commands: Commands,
	f_assets: Res<FontAssets>,
	mut event: EventReader<EventDamageHintSpawn>,
) {
	for ev in event.iter() {
		commands
			.spawn(Text2dBundle {
				text: get_text_element(ev.damage, f_assets.fira_sans.clone(), 1.0),
				transform: Transform::from_xyz(ev.position.x, ev.position.y, 2.0),
				..Default::default()
			})
			.insert(DamageHint {
				damage: ev.damage,
				timer: Timer::from_seconds(HINT_TIME, TimerMode::Once),
			});
	}
}

fn damage_hint_animation(
	mut commands: Commands,
	time: Res<Time>,
	f_assets: Res<FontAssets>,
	mut query: Query<(&mut Transform, &mut Text, &mut DamageHint, Entity), With<DamageHint>>,
) {
	for (mut transform, mut text, mut damage_hint, entity) in query.iter_mut() {
		if damage_hint.timer.finished() {
			commands.entity(entity).despawn();
		} else {
			damage_hint.timer.tick(time.delta());
		}
		transform.translation.y += time.delta_seconds() * HINT_SPEED;

		let transparency = 1.0 - damage_hint.timer.elapsed().as_secs_f32() * 20.0 / HINT_SPEED;
		*text = get_text_element(damage_hint.damage, f_assets.fira_sans.clone(), transparency);
	}
}
