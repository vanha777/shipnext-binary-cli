use std::process::Command;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use colored::Colorize; // Import the colored crate

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./shipnext <command>");
        println!("Commands:");
        println!("  status   - Check git status");
        println!("  modify   - Modify a file");
        println!("  bootstrap - Bootstrap Tauri v2 with iOS for an existing Next.js app");
        return;
    }

    match args[1].as_str() {
        "status" => run_command("git", &["status"]),
        "modify" => modify_file(),
        "bootstrap" => bootstrap_tauri_v2_ios(),
        cmd => println!("Unknown command: {}", cmd),
    }
}

// Run a terminal command with colored debug output
fn run_command(cmd: &str, args: &[&str]) {
    println!("{}", format!("Debug: Running command: {} {}", cmd, args.join(" ")).yellow());
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect(&format!("Failed to execute '{}'", cmd));

    if output.status.success() {
        println!("{}", "Debug: Command succeeded".green());
        println!("{}", format!("Output: {}", String::from_utf8_lossy(&output.stdout)).cyan());
    } else {
        eprintln!("{}", "Debug: Command failed".red());
        eprintln!("{}", format!("Error: {}", String::from_utf8_lossy(&output.stderr)).red());
        std::process::exit(1); // Exit on failure
    }
}

// Modify a file (existing functionality)
fn modify_file() {
    let file_path = "example.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open file");

    writeln!(file, "Modified by shipnext at {}", chrono::Utc::now())
        .expect("Failed to write to file");
    println!("{}", format!("File {} modified successfully!", file_path).green());
    run_command("cat", &[file_path]);
}

// Bootstrap Tauri v2 with iOS for an existing Next.js app
fn bootstrap_tauri_v2_ios() {
    // Check for package.json
    if !Path::new("package.json").exists() {
        eprintln!("{}", "Error: package.json not found. Please run this in a Next.js project directory.".red());
        std::process::exit(1);
    }

    println!("{}", "Bootstrapping Tauri v2 with iOS support...".green());

    // Step 1: Modify next.config.ts
    let next_config_path = if Path::new("next.config.ts").exists() {
        "next.config.ts"
    } else if Path::new("next.config.js").exists() {
        "next.config.js"
    } else {
        println!("{}", "Debug: No next.config file found, creating next.config.ts".yellow());
        "next.config.ts"
    };

    let next_config_content = r#"
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  eslint: {
    ignoreDuringBuilds: true,
  },
  typescript: {
    ignoreBuildErrors: true,
  },
  output: "standalone",
};

export default nextConfig;
"#;
    fs::write(next_config_path, next_config_content).expect("Failed to write next.config.ts");
    println!("{}", format!("Debug: Updated {} with output: 'standalone'", next_config_path).yellow());

    // Step 2: Install Tauri CLI v2
    run_command("cargo", &["install", "tauri-cli", "--version", "^2.0.0-beta"]);

    // Step 3: Initialize Tauri v2
    run_command("cargo", &["tauri", "init", "--ci", "--app-name", "next-tauri-app", "--frontend-dist", "../out", "--dev-url", "http://localhost:3000"]);

    // Step 4: Install Tauri JS dependencies
    run_command("npm", &["install", "@tauri-apps/api", "--save"]);
    run_command("npm", &["install", "@tauri-apps/cli", "--save-dev"]);

    // Step 5: Update package.json scripts
    let package_json_content = r#"
{
  "scripts": {
    "dev": "next dev",
    "build": "next build && next export",
    "tauri": "tauri"
  }
}
"#;
    fs::write("package.json", package_json_content).expect("Failed to update package.json");
    println!("{}", "Debug: Updated package.json with Tauri scripts".yellow());

    // Step 6: Add iOS support
    run_command("cargo", &["tauri", "ios", "init", "--ci"]);

    // Step 7: Configure tauri.conf.json
    let tauri_conf = r#"
{
  "build": {
    "beforeBuildCommand": "npm run build",
    "frontendDist": "../out"
  },
  "tauri": {
    "bundle": {
      "identifier": "com.example.nexttauri",
      "iOS": {
        "developmentTeam": "56D6LN4B5K"
      }
    }
  }
}
"#;
    fs::write("src-tauri/tauri.conf.json", tauri_conf).expect("Failed to write tauri.conf.json");
    println!("{}", "Debug: Configured tauri.conf.json".yellow());

    // Step 8: Build for iOS
    run_command("cargo", &["tauri", "ios", "build", "--release"]);

    println!("{}", "Tauri v2 with iOS bootstrapped successfully!".green());
    println!("To run:");
    println!("  npm run tauri dev  # For development");
    println!("  Check src-tauri/gen/apple/build/arm64/ for the .ipa");
}