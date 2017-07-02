extern crate rand;

use rand::Rng;

// defines a player struct:
#[derive(Clone)]
pub struct Player {
	pub username: String,
	pub health: i8,
	pub score: i8
}

impl Player {
	pub fn add_score(&mut self, points: i8) {
		self.score = self.score + points;
		println!("---{} gained {} points!", self.username, points);
		println!("---{} has {} points.", self.username, self.score);
	}

	pub fn take_damage(&mut self, points: i8) {
		self.health = self.health - points;

		if self.health <= 0 {
			println!("{} took {} damage!", self.username, points);
			println!("{} is dead!", self.username);
		} else {
			println!("{} took {} damage!", self.username, points);
			println!("{}'s health: {}", self.username, self.health);
		}
	}

	pub fn shoot(&mut self, mut enemy: Enemy) -> Enemy {
		let damage = rand::thread_rng().gen_range(20, 50);
		enemy.take_damage(damage);

		if enemy.health < 0 {
			enemy.health = 0;
		}

		println!("Enemy took {} damage!", damage);
		println!("Enemy's health: {}", enemy.health);

		if enemy.health <= 0 {
			self.add_score(1);
			self.health = 100;
			enemy.health = 100;
		}

		enemy
	}
}


// defines an enemy:
#[derive(Clone, Copy)]
pub struct Enemy {
	pub health: i8,
	pub damage: i8
}

impl Enemy {
	pub fn take_damage(&mut self, points: i8) {
		self.health = self.health - points;

	}

	pub fn shoot(&mut self, mut p: Player) -> Player {
		let damage = rand::thread_rng().gen_range(5, 20);
		p.take_damage(damage);

		p
	}
}