use clap::{Parser, Subcommand};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Parser)]
#[command(name = "containerquest")]
#[command(about = "ContainerQuest - Terminal loot & container game")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Play the game (deploy a command or use an item)
    Play {
        /// Deploy command (e.g. "build fast --optimize") or item name
        #[arg(long)]
        command: Option<String>,

        /// Item to use (e.g. "Medkit")
        #[arg(long)]
        item: Option<String>,

        /// Reddit username
        #[arg(long, default_value = "guest")]
        user: String,
    },

    /// Show the leaderboard
    Leaderboard,

    /// Reset the leaderboard (clear all scores)
    Reset,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Leaderboard {
    entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Entry {
    user: String,
    command: String,
    score: u32,
    success: bool,
    health: i32,
    timestamp: u64,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Play { command, item, user } => {
            if let Some(item_name) = item {
                use_item(item_name, user);
            } else if let Some(cmd) = command {
                play_game(cmd, user);
            } else {
                println!("❌ You must provide either --command or --item");
            }
        }
        Commands::Leaderboard => {
            print_leaderboard();
        }
        Commands::Reset => {
            reset_leaderboard();
        }
    }
}

fn play_game(command: &str, user: &str) {
    let mut rng = rand::thread_rng();

    let build_speed = rng.gen_range(1..=50);
    let success_points = if rng.gen_bool(0.7) { 30 } else { 0 };
    let creativity_points = rng.gen_range(0..=20);
    let daily_bonus = streak_bonus(user);

    let fail_messages = [
        "Container exploded during build!",
        "Kernel panic inside the container!",
        "Failed to mount /dev/null!",
        "Docker daemon took a nap!",
    ];

    let mut health = 100;
    if success_points == 0 {
        health -= 20; // lose health on failure
    }

    let total_score = build_speed + success_points + creativity_points + daily_bonus;
    let success = success_points > 0;

    slow_print(&format!("Deploying: {}", command));
    slow_print(&format!("⚡ Build speed: {} pts", build_speed));

    if success {
        slow_print(&format!("✅ Success: {} pts", success_points));
    } else {
        let msg = fail_messages[rng.gen_range(0..fail_messages.len())];
        slow_print(&format!("{} (0 pts)", msg));
    }

    slow_print(&format!("🎨 Creativity: {} pts", creativity_points));
    if daily_bonus > 0 {
        slow_print(&format!("🌟 Streak bonus: {} pts", daily_bonus));
    }
    slow_print(&format!("👉 Final Score: {} pts", total_score));
    slow_print(&format!("❤️ Health remaining: {}", health));

    let entry = Entry {
        user: user.to_string(),
        command: command.to_string(),
        score: total_score,
        success,
        health,
        timestamp: now(),
    };

    save_score(entry);
}

fn use_item(item: &str, user: &str) {
    let mut health = 100;
    match item.to_lowercase().as_str() {
        "medkit" => {
            health += 30;
            slow_print("🩹 You used a Medkit! Restored 30 health.");
        }
        "potion" => {
            health += 20;
            slow_print("🧪 You drank a Potion! Restored 20 health.");
        }
        "bandage" => {
            health += 10;
            slow_print("📦 You used a Bandage! Restored 10 health.");
        }
        _ => {
            println!("❌ Unknown item: {}", item);
            return;
        }
    }

    if health > 100 {
        health = 100;
    }

    let entry = Entry {
        user: user.to_string(),
        command: format!("Used item: {}", item),
        score: 0,
        success: true,
        health,
        timestamp: now(),
    };

    save_score(entry);
}

fn save_score(entry: Entry) {
    let path = "leaderboard.json";

    let mut leaderboard = if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap_or(Leaderboard { entries: vec![] })
    } else {
        Leaderboard { entries: vec![] }
    };

    leaderboard.entries.push(entry);

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    file.write_all(serde_json::to_string_pretty(&leaderboard).unwrap().as_bytes())
        .unwrap();

    println!("✅ Score saved!");
}

fn print_leaderboard() {
    let path = "leaderboard.json";
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        if let Ok(mut leaderboard) = serde_json::from_str::<Leaderboard>(&contents) {
            leaderboard.entries.sort_by(|a, b| b.score.cmp(&a.score));

            println!("\n🏆 Leaderboard (Top 10):");
            for (i, entry) in leaderboard.entries.iter().take(10).enumerate() {
                println!(
                    "{}. {} - {} pts (cmd: `{}`, health: {}, success: {})",
                    i + 1,
                    entry.user,
                    entry.score,
                    entry.command,
                    entry.health,
                    entry.success
                );
            }
        }
    } else {
        println!("No leaderboard data found.");
    }
}

fn reset_leaderboard() {
    let path = "leaderboard.json";
    let empty = Leaderboard { entries: vec![] };

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .unwrap();

    file.write_all(serde_json::to_string_pretty(&empty).unwrap().as_bytes())
        .unwrap();

    println!("🧹 Leaderboard has been reset!");
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn streak_bonus(user: &str) -> u32 {
    let path = "leaderboard.json";
    if let Ok(mut file) = File::open(path) {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        if let Ok(leaderboard) = serde_json::from_str::<Leaderboard>(&contents) {
            if let Some(last) = leaderboard
                .entries
                .iter()
                .rev()
                .find(|e| e.user == user)
            {
                let last_play = last.timestamp / 86400; // days
                let today = now() / 86400;
                if today == last_play + 1 {
                    return 10;
                }
            }
        }
    }
    0
}

fn slow_print(text: &str) {
    for c in text.chars() {
        print!("{}", c);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        sleep(Duration::from_millis(40));
    }
    println!();
}
