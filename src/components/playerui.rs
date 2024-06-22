use bevy::ecs::component::Component;

#[derive(Component)]
pub struct HealthBarComponent {
	pub hp: i32,
	pub hp_max: i32,
}

impl HealthBarComponent {
	pub fn new(hp: i32, hp_max: i32) -> Self {
		Self { hp, hp_max }
	}

	pub fn hit(&mut self, dmg: i32) {
		self.hp -= dmg;
	}
}
