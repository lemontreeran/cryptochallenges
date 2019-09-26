extern crate cryptochallenges;
use failure::Error;
// use cryptochallenges::errors::ChallengeError;
use std::env;

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

fn main() {
    let mut challenges = Vec::<fn() -> Result<(), Error>>::new();
    cryptochallenges::set_1_basics::add_challenges(&mut challenges);

    let challenges_count = challenges.len();

    match challenge_indices(challenges_count) {
        Ok(indices) => {
            for i in indices {
                run_challenge(challenges[i - 1], i);
            }
        }
        Err(arg) => {
            println!(
                "Provided argument \"{}\" is invalid. Expected a number between 1 and {} ",
                arg, challenges_count
            );
        }
    }
}

fn run_challenge<F>(exercise: F, challenge_number: usize)
where
    F: Fn() -> Result<(), Error>,
{
    match exercise() {
        Ok(_) => println!("Challenge {:02}: Success", challenge_number),
        Err(ref e) => {
          println!("Challenge {:02}: {}", challenge_number, e);
        }
    };
}

fn challenge_indices(challenges_count: usize) -> std::result::Result<Vec<usize>, String> {
    let args = env::args();
    if args.len() <= 1 {
        return Ok((1..=challenges_count).collect());
    }

    let mut indices = Vec::new();
    for arg in args.skip(1) {
        if let Ok(index) = arg.parse::<usize>() {
            if index >= 1 && index <= challenges_count {
                indices.push(index);
                continue;
            }
        }
        return Err(arg);
    }
    Ok(indices)
}
