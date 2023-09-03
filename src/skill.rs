use bevy::prelude::Image;

enum SkillType {
	AttackSpeed,
	AttackRange,
	DamageFlat,
	MoveSpeed,
	DamagePercent,
	MaxHp,
	Penetration,
	BulletSize,
	TimeDamage,
	TimeDamageDuration,
	RicochetCount,
	RicochetChance,
	Spirit,
	SpiritDamage,
}

struct Skill {
	icon: Image,
	name: String,
	mode: SkillType,
	value: f32,
}

struct SkillManager {
	skills: Vec<Skill>,
}

//impl Default for SkillManager {
//	fn default() -> Self {
//		let skills = Vec::new();
//	}
//}
//
//impl SkillManager {
//	pub fn get_skills() -> [Skill; 3] {}
//}
