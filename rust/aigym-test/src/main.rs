extern crate gym;
extern crate rand;

use gym::{Action, GymClient, State};
use rand::Rng;

// Hyperparameters
const ALPHA: f64 = 0.1;
const GAMMA: f64 = 0.6;
const EPSILON: f64 = 0.1;
const INFINITY: f64 = -1.0 / 0.0;

fn argmax(v: &[f64]) -> usize {
	let mut i_max = 0;
	let mut f_max = -1.0 / 0.0;
	for (i, &f) in v.iter().enumerate() {
		if f > f_max {
			i_max = i;
			f_max = f;
		}
	}
	i_max
}

fn main() {
	let mut rng = rand::thread_rng();
	let client = GymClient::default();
	let env = client.make("Taxi-v3");
	let mut qtable = [[0.0; 6]; 500];

	// Exploration
	for ep in 0..100_000 {
		let mut epochs = 0;
		let mut done = false;
		let obs = env.reset().expect("Unable to reset");
		let mut state: usize = obs.get_discrete().unwrap();

		while !done {
			let action = if rng.gen_bool(EPSILON) {
				env.action_space().sample().get_discrete().unwrap()
			}
			else {
				argmax(&qtable[state])
			};

			let State {
				observation,
				reward,
				is_done,
			} = env.step(&Action::DISCRETE(action)).unwrap();
            env.render();
			let next_state: usize = observation.get_discrete().unwrap();

			let old_value = qtable[state][action];
			let next_max = qtable[next_state].iter().cloned().fold(INFINITY, f64::max);

			let next_value = (1.0 - ALPHA) * old_value + ALPHA * (reward + GAMMA * next_max);
			qtable[state][action] = next_value;

			state = next_state;
			epochs += 1;
			done = is_done;
		}

		if ep % 100 == 0 {
			println!("Finished episode {} in {}", ep, epochs);
		}
	}
}
