use std::{
    io::{self, Write},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
};

use rand::RngExt;
use tracing::{debug, info};

use crate::{concept::Concept, registry::ConceptRegistry};

mod concept;
mod logging;
mod registry;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    if std::env::args().any(|arg| arg == "-V" || arg == "--version") {
        print_version();
        return Ok(());
    }

    let verbose = std::env::args().any(|arg| arg == "-v" || arg == "--verbose");
    let _guard = logging::setup(verbose);

    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    })?;

    game_loop(running)?;

    Ok(())
}

fn game_loop(running: Arc<AtomicBool>) -> color_eyre::Result<()> {
    let (tx, rx) = mpsc::channel();

    std::thread::spawn(move || {
        let mut buffer = String::new();
        while io::stdin().read_line(&mut buffer).unwrap_or(0) > 0 {
            if tx.send(buffer.clone()).is_err() {
                break;
            }
            buffer.clear();
        }
    });

    let mut rng = rand::rng();
    let registry = ConceptRegistry::new()?;
    let mut cpu_concept: Option<&Concept> = None;

    let mut player_wins: u32 = 0;
    let mut cpu_wins: u32 = 0;

    while running.load(Ordering::SeqCst) {
        while let None = cpu_concept {
            let cpu_set_index = rng.random_range(0..registry.count());
            cpu_concept = registry.at_index(cpu_set_index);
        }
        debug!("The CPU chose {}", cpu_concept.unwrap().id);

        print!("What do you choose? ");
        io::stdout().flush()?;

        let Ok(input) = rx.recv() else { break };
        let input = input.trim().to_lowercase();

        if input == "exit" || input == "quit" || input == "q" {
            break;
        }

        if !running.load(Ordering::SeqCst) {
            break;
        }

        let player_concept = registry.get(&input);

        if player_concept == None {
            debug!("The player chose {} (invalid)", input);
            println!("Your choice ({}) was invalid, please choose one of these options:", input);
            println!("{}", registry.all_ids().join(", "));
        } else {
            let pc = player_concept.unwrap();
            let cpuc = cpu_concept.unwrap();

            let winner = match (pc.beats.contains(&cpuc.id), cpuc.beats.contains(&pc.id)) {
                (true, true) => "Tie",
                (true, false) => {
                    player_wins += 1;
                    "You"
                }
                (false, true) => {
                    cpu_wins += 1;
                    "CPU"
                }
                (false, false) => "Nobody",
            };

            info!("You chose {}", pc.pretty_name);
            info!("CPU chose {}", cpuc.pretty_name);
            info!("Winner: {}", winner);
            info!("You've won {} times, CPU has won {} times", player_wins, cpu_wins);

            cpu_concept = None;
        }
    }

    println!("\nGoodbye!");
    Ok(())
}

fn print_version() {
    println!("rock paper scissors v{}", env!("CARGO_PKG_VERSION"));
}
