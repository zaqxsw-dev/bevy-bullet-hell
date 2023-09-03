use bevy::prelude::{Input, KeyCode, MouseButton, Res};

pub enum GameControl {
	Up,
	Down,
	Left,
	Right,
	Fire,
	SecondFire,
	MenuOpen,
}

impl GameControl {
	pub fn pressed(
		&self,
		keyboard_input: &Res<Input<KeyCode>>,
		_mouse_input: &Res<Input<MouseButton>>,
	) -> bool {
		match self {
			GameControl::Up => {
				keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up)
			}
			GameControl::Down => {
				keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down)
			}
			GameControl::Left => {
				keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left)
			}
			GameControl::Right => {
				keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right)
			}
			_ => false,
		}
	}
}

pub fn get_movement(
	control: GameControl,
	input: &Res<Input<KeyCode>>,
	minput: &Res<Input<MouseButton>>,
) -> f32 {
	if control.pressed(input, minput) {
		1.0
	} else {
		0.0
	}
}

pub fn get_fire(
	control: GameControl,
	_input: &Res<Input<KeyCode>>,
	minput: &Res<Input<MouseButton>>,
) -> bool {
	match control {
		GameControl::Fire => minput.pressed(MouseButton::Left),
		GameControl::SecondFire => minput.pressed(MouseButton::Right),
		_ => false,
	}
}

pub fn menu_open(
	control: GameControl,
	input: &Res<Input<KeyCode>>,
	_minput: &Res<Input<MouseButton>>,
) -> bool {
	match control {
		GameControl::MenuOpen => input.just_pressed(KeyCode::Escape),
		_ => false,
	}
}
