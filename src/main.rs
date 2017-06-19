extern crate rand;

use rand::Rng;

fn main() {
	const POINTS_TO_WIN: i8 = 5;

    let mut p1 = Player {
    	username: String::from("jakegornall"),
    	health: 100,
    	score: 0
    };

    let mut e1 = Enemy {
    	health: 100,
    	damage: rand::thread_rng().gen_range(5, 20)
    };

    // main gameplay loop.
    loop {
    	e1 = p1.shoot(e1);

    	if p1.score >= POINTS_TO_WIN {
    		println!("{} won the game!", p1.username);
    		break;
    	}

    	p1 = e1.shoot(p1);

    	if p1.health <= 0 {
    		break;
    	}

    }
}

// defines a player struct:
#[derive(Clone)]
struct Player {
	username: String,
	health: i8,
	score: i8
}

impl Player {
	fn add_score(&mut self, points: i8) {
		self.score = self.score + points;
		println!("---{} gained {} points!", self.username, points);
		println!("---{} has {} points.", self.username, self.score);
	}

	fn take_damage(&mut self, points: i8) {
		self.health = self.health - points;

		if self.health <= 0 {
			println!("{} took {} damage!", self.username, points);
			println!("{} is dead!", self.username);
		} else {
			println!("{} took {} damage!", self.username, points);
			println!("{}'s health: {}", self.username, self.health);
		}
	}

	fn shoot(&mut self, mut enemy: Enemy) -> Enemy {
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
struct Enemy {
	health: i8,
	damage: i8
}

impl Enemy {
	fn take_damage(&mut self, points: i8) {
		self.health = self.health - points;

	}

	fn shoot(&mut self, mut p: Player) -> Player {
		let damage = rand::thread_rng().gen_range(5, 20);
		p.take_damage(damage);

		p
	}
}
