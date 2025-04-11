use bevy::prelude::{ButtonInput, KeyCode, MouseButton, Res};

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
		keyboard_input: &Res<ButtonInput<KeyCode>>,
		_mouse_input: &Res<ButtonInput<MouseButton>>,
	) -> bool {
		match self {
			GameControl::Up => {
				keyboard_input.pressed(KeyCode::KeyW)
					|| keyboard_input.pressed(KeyCode::ArrowUp)
			}
			GameControl::Down => {
				keyboard_input.pressed(KeyCode::KeyS)
					|| keyboard_input.pressed(KeyCode::ArrowDown)
			}
			GameControl::Left => {
				keyboard_input.pressed(KeyCode::KeyA)
					|| keyboard_input.pressed(KeyCode::ArrowLeft)
			}
			GameControl::Right => {
				keyboard_input.pressed(KeyCode::KeyD)
					|| keyboard_input.pressed(KeyCode::ArrowRight)
			}
			_ => false,
		}
	}
}

pub fn get_movement(
	control: GameControl,
	input: &Res<ButtonInput<KeyCode>>,
	minput: &Res<ButtonInput<MouseButton>>,
) -> f32 {
	if control.pressed(input, minput) {
		1.0
	} else {
		0.0
	}
}

pub fn get_fire(
	control: GameControl,
	_input: &Res<ButtonInput<KeyCode>>,
	minput: &Res<ButtonInput<MouseButton>>,
) -> bool {
	match control {
		GameControl::Fire => minput.pressed(MouseButton::Left),
		GameControl::SecondFire => minput.pressed(MouseButton::Right),
		_ => false,
	}
}
