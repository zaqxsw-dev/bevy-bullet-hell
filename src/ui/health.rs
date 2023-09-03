use bevy::prelude::*;

use crate::{
	loading::{FontAssets, TextureAssets},
	player::Player,
	GameState, Killable,
};

#[derive(Bundle)]
struct HealthBarBundle {
	text: TextBundle,
}

#[derive(Component)]
struct HealthBarComponent {
	hp: i32,
	hp_max: i32,
}

pub struct PlayerHealthBar;

impl Plugin for PlayerHealthBar {
	fn build(&self, app: &mut App) {
		app.add_systems(
			Update,
			(
				spawn_health_bar.run_if(in_state(GameState::Playing)),
				update_health_bar.run_if(in_state(GameState::Playing)),
			),
		);
	}
}

fn get_text_element(ka: &Killable, fa: Res<FontAssets>) -> Text {
	let style = TextStyle {
		font: fa.fira_sans.clone(),
		font_size: 15.0,
		color: Color::ANTIQUE_WHITE,
	};
	let current = ka.hp;
	let max = ka.hp_max;
	Text::from_section(format!("{current}/{max}"), style).with_alignment(TextAlignment::Right)
}

fn update_health_bar(
	query: Query<&Killable, With<Player>>,
	f_assets: Res<FontAssets>,
	mut hb_query: Query<(&mut Text, &HealthBarComponent), With<HealthBarComponent>>,
) {
	if let Ok((mut text, hp_b)) = hb_query.get_single_mut() {
		if let Ok(killable) = query.get_single() {
			if hp_b.hp != killable.hp || hp_b.hp_max != killable.hp {
				*text = get_text_element(killable, f_assets);
			}
		}
	}
}

fn spawn_health_bar(
	mut commands: Commands,
	f_assets: Res<FontAssets>,
	t_assets: Res<TextureAssets>,
	query: Query<&Killable, With<Player>>,
	hb_query: Query<&HealthBarComponent>,
) {
	if hb_query.iter().count() > 0 {
		return;
	}
	if let Ok(killable) = query.get_single() {
		let text = get_text_element(killable, f_assets);
		commands
			.spawn(NodeBundle {
				style: Style {
					position_type: PositionType::Absolute,
					width: Val::Percent(100.0),
					height: Val::Percent(100.0),
					align_items: AlignItems::Start,
					justify_content: JustifyContent::End,
					..default()
				},
				..default()
			})
			.with_children(|parent| {
				parent
					.spawn(HealthBarBundle {
						text: TextBundle { text, ..default() }.with_style(Style {
							margin: UiRect::new(
								Val::Px(0.),
								Val::Px(0.),
								Val::Px(15.),
								Val::Px(0.),
							),
							..default()
						}),
					})
					.insert(HealthBarComponent {
						hp: killable.hp,
						hp_max: killable.hp_max,
					});
				parent.spawn(ImageBundle {
					image: UiImage {
						texture: t_assets.heart.clone(),
						..Default::default()
					},
					style: Style {
						margin: UiRect::new(
							Val::Px(0.),
							Val::Px(0.),
							Val::Px(-10.),
							Val::Px(0.),
						),
						..default()
					},
					transform: Transform {
						scale: Vec3::new(0.5, 0.5, 1.),
						..Default::default()
					},
					..Default::default()
				});
			});
	}
}
