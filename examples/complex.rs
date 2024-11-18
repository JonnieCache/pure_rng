//! This example demonstrates the use of PureRNG in a more complex setting like
//! a game. Observe how the chain of rng objects distributes unique seed values
//! down through successive function calls. To use an arbitrary root seed value,
//! pass it on the command line.
//!
//! For example:
//!
//! cargo run --example complex -- "my seed value"

use std::{
    fmt::{Display, Formatter, Result},
    io::Read,
    time::SystemTime,
};

use pure_rng::PureRng;

use rand_distr::{num_traits::Float, Normal};

fn main() {
    // The root rng, seeded with the first command line arg,
    // or the system time
    let root_rng = match std::env::args().nth(1) {
        Some(arg) => PureRng::new(arg),
        None => PureRng::new(
            SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        ),
    };

    // Define an rng for generating monsters
    let monster_rng = root_rng.seed("monster generation");

    // Generate the monsters. We're passing each call its own RNG.
    // In a real game you'd need some unique state for every one, eg.
    // a globally incrementing counter. See the game loop below for
    // an example of this.
    let mut red = Monster::generate(Color::Red, monster_rng.seed("red monster"));
    let mut blue = Monster::generate(Color::Blue, monster_rng.seed("blue monster"));

    print_game_state(&red, &blue);
    pause();

    let mut turn = 1;
    let game_rng = root_rng.seed("game logic resolution");

    loop {
        println!("~~~ Turn {} ~~~", turn);

        // Spawn a new RNG for each turn via the game logic rng defined
        // above, differentiated by the turn counter.
        let turn_rng = game_rng.seed(turn);

        // Resolve the attacks by passing a specific rng for each one,
        // similarly to how the monsters were generated.
        resolve_attack(&red, &mut blue, turn_rng.seed(red.color));
        resolve_attack(&blue, &mut red, turn_rng.seed(blue.color));

        println!();
        print_game_state(&red, &blue);
        println!();
        pause();

        turn += 1;
    }
}

fn resolve_attack(attacker: &Monster, target: &mut Monster, rng: PureRng) {
    // Seed an RNG for a specific value and consume it inline
    let hit = attacker.hit_chance > rng.seed("hit roll").gen();

    if hit {
        target.health -= attacker.damage;

        println!(
            "The {0:?} monster hits for {1} damage!",
            attacker.color, attacker.damage
        );

        if target.health <= 0 {
            println!();
            println!("The {0:?} monster was slain!", target.color);
            println!("The {0:?} monster is victorious!", attacker.color);
            std::process::exit(0);
        }
    } else {
        println!("The {0:?} monster misses!", attacker.color);
    }
}

#[derive(Debug)]
struct Monster {
    health: i32,
    max_health: i32,
    damage: i32,
    hit_chance: f32,
    color: Color,
}

impl Monster {
    fn generate(color: Color, rng: PureRng) -> Self {
        // Here we use the rand crate's distribution API
        // to get normally-distributed rolls. This may or may not be
        // a sensible way to balance your game :)
        let health: f32 = rng
            .seed("health")
            .sample(Normal::new(20., 6.0).unwrap())
            .round();
        let health = health as i32;

        let damage: i32 = rng
            .seed("damage")
            .sample(Normal::new(3., 1.5).unwrap())
            .clamp(1., 10.)
            .round() as i32;

        let hit_chance = rng
            .seed("hit chance")
            .sample(Normal::new(0.8, 0.3).unwrap())
            .clamp(0.5, 1.);

        Monster {
            health,
            max_health: health,
            damage,
            hit_chance,
            color,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash)]
enum Color {
    Red,
    Blue,
}

impl Display for Monster {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let health_bar = (0..self.health)
            .map(|_| '█')
            .chain((self.health..self.max_health).map(|_| ' '))
            .collect::<String>();

        write!(
            f,
            "{0:?} monster: {1}/{2}hp [{3}]\n{4} dmg at {5:.0}% hit chance",
            self.color,
            self.health,
            self.max_health,
            health_bar,
            self.damage,
            self.hit_chance * 100.
        )
    }
}

fn pause() {
    println!("Press ENTER to continue...");
    let buffer = &mut [0u8];
    std::io::stdin().read_exact(buffer).unwrap();
}

fn print_game_state(red: &Monster, blue: &Monster) {
    println!("{}", red);
    println!();
    println!("{}", blue);
    println!();
}
