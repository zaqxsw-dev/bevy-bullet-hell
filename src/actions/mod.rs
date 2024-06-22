use bevy::prelude::*;

use crate::actions::game_control::{get_movement, GameControl};
use crate::GameState;

use self::game_control::{get_fire, menu_open};

mod game_control;

pub struct ActionsPlugin;

impl Plugin for ActionsPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<Actions>().add_systems(
			Update,
			(
				set_movement_actions.run_if(in_state(GameState::Playing)),
				set_shooting_actions.run_if(in_state(GameState::Playing)),
				set_menu_actions.run_if(in_state(GameState::Playing)),
			),
		);
	}
}

#[derive(Default, Resource)]
pub struct Actions {
	pub player_movement: Option<Vec2>,
	pub player_is_shooting: bool,
	pub player_is_second_shooting: bool,
}

pub fn set_menu_actions(
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mouse_input: Res<ButtonInput<MouseButton>>,
	mut game_state: ResMut<NextState<GameState>>,
) {
	if menu_open(GameControl::MenuOpen, &keyboard_input, &mouse_input) {
		game_state.set(GameState::Menu);
	}
}

pub fn set_shooting_actions(
	mut actions: ResMut<Actions>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mouse_input: Res<ButtonInput<MouseButton>>,
) {
	actions.player_is_shooting = get_fire(GameControl::Fire, &keyboard_input, &mouse_input);
	actions.player_is_second_shooting =
		get_fire(GameControl::SecondFire, &keyboard_input, &mouse_input);
}

pub fn set_movement_actions(
	mut actions: ResMut<Actions>,
	keyboard_input: Res<ButtonInput<KeyCode>>,
	mouse_input: Res<ButtonInput<MouseButton>>,
) {
	let player_movement = Vec2::new(
		get_movement(GameControl::Right, &keyboard_input, &mouse_input)
			- get_movement(GameControl::Left, &keyboard_input, &mouse_input),
		get_movement(GameControl::Up, &keyboard_input, &mouse_input)
			- get_movement(GameControl::Down, &keyboard_input, &mouse_input),
	);

	if player_movement != Vec2::ZERO {
		actions.player_movement = Some(player_movement.normalize());
	} else {
		actions.player_movement = None;
	}
}
