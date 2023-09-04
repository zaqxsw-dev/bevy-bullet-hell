use bevy::prelude::*;

use crate::{
	loading::FontAssets,
	player::{Player, PlayerMove},
	GameState, Killable, MainCamera, SceneObject,
};

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(OnEnter(GameState::Gameover), init_gameover)
			.add_systems(Update, on_key_press.run_if(in_state(GameState::Gameover)))
			.add_systems(OnExit(GameState::Gameover), cleanup_gameover);
	}
}

fn on_key_press(
	mut game_state: ResMut<NextState<GameState>>,
	keyboard_input: Res<Input<KeyCode>>,
) {
	if keyboard_input.pressed(KeyCode::Space) {
		game_state.set(GameState::Menu);
	}
}

fn init_gameover(mut commands: Commands, font_assets: Res<FontAssets>) {
	commands
		.spawn(NodeBundle {
			style: Style {
				position_type: PositionType::Absolute,
				width: Val::Percent(100.0),
				height: Val::Percent(100.0),
				flex_direction: FlexDirection::Column,
				align_items: AlignItems::Center,
				justify_content: JustifyContent::Center,
				..default()
			},
			background_color: Color::rgba(0.0, 0.0, 0.0, 0.4).into(),
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn((NodeBundle {
					style: Style {
						width: Val::Px(390.0),
						height: Val::Px(75.0),
						justify_content: JustifyContent::Center,
						align_items: AlignItems::Center,
						flex_direction: FlexDirection::Column,
						..Default::default()
					},
					..Default::default()
				},))
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Game Over",
						TextStyle {
							font: font_assets.fira_sans.clone(),
							font_size: 60.0,
							color: Color::rgb(0.9, 0.9, 0.9),
						},
					));
				})
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Press SPACE key to continue",
						TextStyle {
							font: font_assets.fira_sans.clone(),
							font_size: 20.0,
							color: Color::rgb(0.9, 0.9, 0.9),
						},
					));
				});
		});
}

fn cleanup_gameover(
	mut commands: Commands,
	button: Query<Entity, With<Node>>,
	q_scene_objects: Query<Entity, With<SceneObject>>,
	mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<PlayerMove>)>,
) {
	for entity in &button {
		commands.entity(entity).despawn_recursive();
	}
	for entity in &q_scene_objects {
		commands.entity(entity).despawn_recursive();
	}
	for mut camera in &mut camera_query {
		camera.translation.x = 0.0;
		camera.translation.y = 0.0;
	}
}
