use clap::{Parser, Subcommand};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use chrono::Local;

#[derive(Parser)]
#[command(name = "containerquest")]
#[command(about = "ContainerQuest - Reddit-style container deploy game")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Simulate a container deploy with a fake command
    Simulate {
        /// Deploy command (e.g. "build fast --optimize")
        command: String,

        /// Reddit username
        #[arg(long, default_value = "guest")]
        user: String,
    },

    /// Show the leaderboard
    Leaderboard,
}

#[derive(Serialize, Deserialize, Debug)]
struct Leaderboard {
    entries: Vec<Entry>,
    streaks: HashMap<String, u32>, // Track user streaks
    date: String,                  // Leaderboard date
}

#[derive(Serialize, Deserialize, Debug)]
struct Entry {
    user: String,
    command: String,
    score: u32,
    success: bool,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Simulate { command, user } => {
            let mut rng = rand::thread_rng();

            let build_speed = rng.gen_range(1..=50);
            let success_points = if rng.gen_bool(0.7) { 30 } else { 0 };
            let creativity_points = rng.gen_range(0..=20);
            let daily_bonus = if rng.gen_bool(0.1) { 10 } else { 0 };

            // Load leaderboard
            let today = Local::now().format("%Y-%m-%d").to_string();
            let mut leaderboard = load_leaderboard(&today);

            // Handle streaks
            let mut streak_bonus = 0;
            if success_points > 0 {
                let streak = leaderboard.streaks.entry(user.clone()).or_insert(0);
                *streak += 1;
                streak_bonus = (*streak - 1) * 5; // +5 per extra day
            } else {
                leaderboard.streaks.insert(user.clone(), 0);
            }

            let total_score =
                build_speed + success_points + creativity_points + daily_bonus + streak_bonus;

            let success = success_points > 0;

            println!("🚀 Deploying: {}", command);
            println!("⚡ Build speed: {} pts", build_speed);
            println!("✅ Success: {} pts", success_points);
            println!("🎨 Creativity: {} pts", creativity_points);
            println!("🌟 Daily bonus: {} pts", daily_bonus);
            println!("🔥 Streak bonus: {} pts", streak_bonus);
            println!("👉 Final Score: {} pts", total_score);

            let entry = Entry {
                user: user.to_string(),
                command: command.to_string(),
                score: total_score,
                success,
            };

            leaderboard.entries.push(entry);

            save_leaderboard(&today, &leaderboard);

            println!("✅ Score saved!");
        }
        Commands::Leaderboard => {
            let today = Local::now().format("%Y-%m-%d").to_string();
            print_leaderboard(&today);
        }
    }
}

fn load_leaderboard(today: &str) -> Leaderboard {
    let path = format!("leaderboard-{}.json", today);
    if let Ok(mut file) = File::open(&path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap_or(Leaderboard {
            entries: vec![],
            streaks: HashMap::new(),
            date: today.to_string(),
        })
    } else {
        Leaderboard {
            entries: vec![],
            streaks: HashMap::new(),
            date: today.to_string(),
        }
    }
}

fn save_leaderboard(today: &str, leaderboard: &Leaderboard) {
    let path = format!("leaderboard-{}.json", today);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .unwrap();

    file.write_all(serde_json::to_string_pretty(&leaderboard).unwrap().as_bytes())
        .unwrap();
}

fn print_leaderboard(today: &str) {
    let path = format!("leaderboard-{}.json", today);
    if let Ok(mut file) = File::open(&path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        if let Ok(mut leaderboard) = serde_json::from_str::<Leaderboard>(&contents) {
            // sort entries by score (descending)
            leaderboard.entries.sort_by(|a, b| b.score.cmp(&a.score));

            println!("\n🏆 Leaderboard for {} (Top 10):", today);
            for (i, entry) in leaderboard.entries.iter().take(10).enumerate() {
                println!(
                    "{}. {} - {} pts (cmd: `{}`, success: {})",
                    i + 1,
                    entry.user,
                    entry.score,
                    entry.command,
                    entry.success
                );
            }
        }
    } else {
        println!("No leaderboard data found for today.");
    }
}
