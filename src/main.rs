extern crate rand;
mod characters;

use rand::Rng;

fn main() {
	const POINTS_TO_WIN: i8 = 5;

    let mut p1 = characters::Player {
    	username: String::from("jakegornall"),
    	health: 100,
    	score: 0
    };

    let mut e1 = characters::Enemy {
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
