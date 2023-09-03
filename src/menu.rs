use crate::loading::FontAssets;
use crate::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<ButtonColors>()
			.add_systems(OnEnter(GameState::Menu), setup_menu)
			.add_systems(
				Update,
				(
					handle_hover_buttons.run_if(in_state(GameState::Menu)),
					menu_action.run_if(in_state(GameState::Menu)),
				),
			)
			.add_systems(OnExit(GameState::Menu), cleanup_menu);
	}
}

#[derive(Resource)]
struct ButtonColors {
	clicked: Color,
	normal: Color,
	hovered: Color,
	fade: Color,
}

#[derive(Component)]
enum MenuButtonAction {
	Play,
	Quit,
}

impl Default for ButtonColors {
	fn default() -> Self {
		ButtonColors {
			clicked: Color::rgb(0.1, 0.1, 0.1),
			normal: Color::rgba(0.15, 0.15, 0.15, 0.0),
			fade: Color::rgba(0.0, 0.0, 0.0, 0.4),
			hovered: Color::rgb(0.25, 0.25, 0.25),
		}
	}
}

fn setup_menu(
	mut commands: Commands,
	font_assets: Res<FontAssets>,
	button_colors: Res<ButtonColors>,
) {
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
			background_color: button_colors.fade.into(),
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn((
					ButtonBundle {
						style: Style {
							width: Val::Px(200.0),
							height: Val::Px(75.0),
							//margin: UiRect::all(Val::Auto),
							justify_content: JustifyContent::Center,
							align_items: AlignItems::Center,
							..Default::default()
						},
						background_color: button_colors.normal.into(),
						..Default::default()
					},
					MenuButtonAction::Play,
				))
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Play",
						TextStyle {
							font: font_assets.fira_sans.clone(),
							font_size: 40.0,
							color: Color::rgb(0.9, 0.9, 0.9),
						},
					));
				});
			parent
				.spawn((
					ButtonBundle {
						style: Style {
							width: Val::Px(200.0),
							height: Val::Px(50.0),
							//margin: UiRect::all(Val::Auto),
							justify_content: JustifyContent::Center,
							align_items: AlignItems::Center,
							..Default::default()
						},
						//background_color: button_colors.normal.into(),
						..Default::default()
					},
					MenuButtonAction::Quit,
				))
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section(
						"Exit",
						TextStyle {
							font: font_assets.fira_sans.clone(),
							font_size: 20.0,
							color: Color::rgb(0.8, 0.8, 0.8),
						},
					));
				});
		});
}

fn handle_hover_buttons(
	button_colors: Res<ButtonColors>,
	mut interaction_query: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>),
	>,
) {
	for (interaction, mut color) in &mut interaction_query {
		match *interaction {
			Interaction::Pressed => {
				*color = button_colors.clicked.into();
			}
			Interaction::Hovered => {
				*color = button_colors.hovered.into();
			}
			Interaction::None => {
				*color = button_colors.normal.into();
			}
		}
	}
}

fn menu_action(
	interaction_query: Query<
		(&Interaction, &MenuButtonAction),
		(Changed<Interaction>, With<Button>),
	>,
	mut app_exit_events: EventWriter<AppExit>,
	mut game_state: ResMut<NextState<GameState>>,
) {
	for (interaction, menu_button_action) in &interaction_query {
		if *interaction == Interaction::Pressed {
			match menu_button_action {
				MenuButtonAction::Quit => app_exit_events.send(AppExit),
				MenuButtonAction::Play => {
					game_state.set(GameState::Playing);
				}
			}
		}
	}
}

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<Node>>) {
	for entity in &button {
		commands.entity(entity).despawn_recursive();
	}
}
