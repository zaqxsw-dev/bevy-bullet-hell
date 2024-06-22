use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Killable {
	pub hp: i32,
	pub god_mode: bool,
	pub hp_max: i32,
}

impl Killable {
	pub fn new(hp: i32, hp_max: i32, god_mode: bool) -> Self {
		Self { hp, hp_max, god_mode }
	}

	pub fn hit(&mut self, dmg: i32) {
		self.hp -= dmg;
	}
}
