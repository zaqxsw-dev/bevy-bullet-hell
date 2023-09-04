use bevy::prelude::*;

use crate::{player::Player, GameState};

#[derive(Bundle)]
struct ExpBarBundle {
	text: TextBundle,
}

#[derive(Component)]
struct ExpBar;

pub struct PlayerExpBar;

impl Plugin for PlayerExpBar {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				spawn_exp_bar.run_if(in_state(GameState::Playing)),
				update_exp_bar.run_if(in_state(GameState::Playing)),
			),
		);
	}
}

fn get_exp_bar_style(player: &Player) -> Style {
	let lvl_percent = (player.exp * 100) as f32 / player.next_lvl_exp as f32;
	Style {
		margin: UiRect::all(Val::Px(2.0)),
		width: Val::Percent(lvl_percent),
		height: Val::Px(6.0),
		..Default::default()
	}
}

fn update_exp_bar(query: Query<&Player>, mut eb_query: Query<&mut Style, With<ExpBar>>) {
	if let Ok(mut exp) = eb_query.get_single_mut() {
		if let Ok(player) = query.get_single() {
			*exp = get_exp_bar_style(player);
		}
	}
}

fn spawn_exp_bar(mut commands: Commands, query: Query<&Player>, eb_query: Query<&ExpBar>) {
	if eb_query.iter().len() > 0 {
		return;
	}
	if let Ok(player) = query.get_single() {
		commands
			.spawn(NodeBundle {
				style: Style {
					position_type: PositionType::Absolute,
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					align_items: AlignItems::End,
					justify_content: JustifyContent::Start,
					..default()
				},
				..default()
			})
			.with_children(|parent| {
				parent
					.spawn((NodeBundle {
						background_color: BackgroundColor(Color::GRAY),
						style: Style {
							width: Val::Percent(100.0),
							height: Val::Px(10.0),
							..Default::default()
						},
						..Default::default()
					},))
					.with_children(|parent| {
						parent
							.spawn(NodeBundle {
								background_color: BackgroundColor(Color::PURPLE),
								style: get_exp_bar_style(player),
								..Default::default()
							})
							.insert(ExpBar);
					});
			});
	}
}
