use colored::Colorize;
use serde_json;
use std::fs;
use std::path::Path;
use std::process::Command;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: ./shipnext <command>");
        println!("Commands:");
        println!("  status   - Check git status");
        println!("  bootstrap - Bootstrap Tauri v2 with iOS for an existing Next.js app");
        return;
    }

    match args[1].as_str() {
        "status" => run_command("git", &["status"]),
        "bootstrap" => bootstrap_tauri_v2_ios(),
        "simulator" => simulator_ios(),
        cmd => println!("Unknown command: {}", cmd),
    }
}

fn run_command(cmd: &str, args: &[&str]) {
    println!(
        "{}",
        format!("Debug: Running command: {} {}", cmd, args.join(" ")).yellow()
    );
    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect(&format!("Failed to execute '{}'", cmd));

    if output.status.success() {
        println!("{}", "Debug: Command succeeded".green());
        println!(
            "{}",
            format!("Output: {}", String::from_utf8_lossy(&output.stdout)).cyan()
        );
    } else {
        eprintln!("{}", "Debug: Command failed".red());
        eprintln!(
            "{}",
            format!("Error: {}", String::from_utf8_lossy(&output.stderr)).red()
        );
        std::process::exit(1);
    }
}

fn bootstrap_tauri_v2_ios() {
    if !Path::new("package.json").exists() {
        eprintln!(
            "{}",
            "Error: package.json not found. Please run this in a Next.js project directory.".red()
        );
        std::process::exit(1);
    }

    println!("{}", "Bootstrapping Tauri v2 with iOS support...".green());

    // Step 1: Modify next.config.ts
    let next_config_path = if Path::new("next.config.ts").exists() {
        "next.config.ts"
    } else if Path::new("next.config.js").exists() {
        "next.config.js"
    } else {
        println!(
            "{}",
            "Debug: No next.config file found, creating next.config.ts".yellow()
        );
        "next.config.ts"
    };

    let next_config_content = r#"
import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  output: 'export',
};

export default nextConfig;
"#;
    fs::write(next_config_path, next_config_content).expect("Failed to write next.config.ts");
    println!(
        "{}",
        format!("Debug: Updated {} with output: 'export'", next_config_path).yellow()
    );

    // Step 2: Install Tauri CLI v2
    run_command(
        "cargo",
        &["install", "tauri-cli", "--version", "^2.0.0-beta"],
    );

    // Step 3: Install Tauri JS dependencies and update package.json
    run_command("npm", &["install", "@tauri-apps/api", "--save"]);
    run_command("npm", &["install", "@tauri-apps/cli", "--save-dev"]);

    // Modify package.json to add "tauri": "tauri" to scripts
    let package_json_path = "package.json";
    let package_json_content =
        fs::read_to_string(package_json_path).expect("Failed to read package.json");
    let mut package_json: serde_json::Value =
        serde_json::from_str(&package_json_content).expect("Failed to parse package.json");

    if let Some(scripts) = package_json
        .get_mut("scripts")
        .and_then(|s| s.as_object_mut())
    {
        scripts.insert(
            "tauri".to_string(),
            serde_json::Value::String("tauri".to_string()),
        );
    } else {
        let mut scripts = serde_json::Map::new();
        scripts.insert(
            "tauri".to_string(),
            serde_json::Value::String("tauri".to_string()),
        );
        package_json
            .as_object_mut()
            .unwrap()
            .insert("scripts".to_string(), serde_json::Value::Object(scripts));
    }

    fs::write(
        package_json_path,
        serde_json::to_string_pretty(&package_json).expect("Failed to serialize package.json"),
    )
    .expect("Failed to write updated package.json");
    println!(
        "{}",
        "Debug: Added 'tauri': 'tauri' to package.json scripts".yellow()
    );

    // Step 4: Initialize Tauri v2
    if !Path::new("src-tauri").exists() {
        run_command(
            "cargo",
            &[
                "tauri",
                "init",
                "--ci",
                "--app-name",
                "shipnext",
                "--frontend-dist",
                "../out",
                "--dev-url",
                "http://localhost:3000",
            ],
        );
    } else {
        println!(
            "{}",
            "Debug: src-tauri already exists, skipping initialization".yellow()
        );
    }

    // Step 5: Add iOS support
    if !Path::new("src-tauri/gen/apple").exists() {
        run_command("cargo", &["tauri", "ios", "init"]);
    } else {
        println!(
            "{}",
            "Debug: iOS support already initialized, skipping ios init".yellow()
        );
    }

    // Step 6: Configure tauri.conf.json
    let tauri_conf = r#"
{
  "$schema": "https://schema.tauri.app/config/2.0.0-rc",
  "productName": "shipnext",
  "version": "0.1.0",
  "identifier": "com.shipnext.dev",
  "build": {
    "frontendDist": "../out",
    "devUrl": "http://localhost:3000",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [
      {
        "title": "shipnext",
        "width": 800,
        "height": 600,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": null
    }
  },
  "bundle": {
    "iOS": {
      "developmentTeam": "NXR8WH6TN8",
      "minimumSystemVersion": "13.0"
    },
    "active": true,
    "targets": "all",
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  }
}
"#;
    fs::write("src-tauri/tauri.conf.json", tauri_conf).expect("Failed to write tauri.conf.json");
    println!("{}", "Debug: Configured tauri.conf.json".yellow());

    // Step 7: Reinstall npm dependencies
    run_command("npm", &["install"]);

    // Step 8: Build for iOS
    run_command("cargo", &["tauri", "ios", "build"]);

    println!("{}", "Tauri v2 with iOS bootstrapped successfully!".green());
    println!("To run:");
    println!("  npm run tauri dev  # For development");
    println!("  Check src-tauri/gen/apple/build/arm64/[your-app-name].ipa for the .ipa");
}

fn simulator_ios() {
    println!("{}", "Let's start the simulation for you as well".green());
    run_command("cargo", &["tauri", "ios", "dev", "iPhone 15 Pro"]);
    println!(
        "{}",
        "Tauri v2 with iOS bootstrapped and Simulator started successfully!".green()
    );
    println!("Check the Simulator for your app running on iPhone 15 Pro");
}
