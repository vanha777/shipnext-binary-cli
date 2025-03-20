use colored::Colorize;
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

// Run a terminal command with colored debug output
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
        std::process::exit(1); // Exit on failure
    }
}

// Bootstrap Tauri v2 with iOS for an existing Next.js app
fn bootstrap_tauri_v2_ios() {
    // Check for package.json
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
const isProd = process.env.NODE_ENV === 'production';

const internalHost = process.env.TAURI_DEV_HOST || 'localhost';

/** @type {import('next').NextConfig} */
const nextConfig = {
  // Ensure Next.js uses SSG instead of SSR
  // https://nextjs.org/docs/pages/building-your-application/deploying/static-exports
  output: 'export',
  // Note: This feature is required to use the Next.js Image component in SSG mode.
  // See https://nextjs.org/docs/messages/export-image-api for different workarounds.
  images: {
    unoptimized: true,
  },
  // Configure assetPrefix or else the server won't properly resolve your assets.
  assetPrefix: isProd ? undefined : `http://${internalHost}:3000`,
};

export default nextConfig;
"#;
    fs::write(next_config_path, next_config_content).expect("Failed to write next.config.ts");
    println!(
        "{}",
        format!(
            "Debug: Updated {} with output: 'export'",
            next_config_path
        )
        .yellow()
    );

    // Step 2: Install Tauri CLI v2
    run_command(
        "cargo",
        &["install", "tauri-cli", "--version", "^2.0.0-beta"],
    );

    // Step 3: Initialize Tauri v2 (only if src-tauri doesn't exist)
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

    // Step 4: Install Tauri JS dependencies
    run_command("npm", &["install", "@tauri-apps/api", "--save"]);
    run_command("npm", &["install", "@tauri-apps/cli", "--save-dev"]);

    // Step 5: Add iOS support (only if not already initialized)
    if !Path::new("src-tauri/ios").exists() {
        run_command("cargo", &["tauri", "ios", "init", "--ci"]);
    } else {
        println!(
            "{}",
            "Debug: iOS support already initialized, skipping ios init".yellow()
        );
    }

    // Step 6: Configure tauri.conf.json
    let tauri_conf = r#"
{
  "$schema": "https://schema.tauri.app/config/2.0.0-rc.1",
  "productName": "shipnext",
  "version": "0.1.0",
  "identifier": "com.shipnext.dev",
  "build": {
    "frontendDist": "../out",
    "devUrl": "http://localhost:3000",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "bundle": {
    "iOS": {
      "developmentTeam": "NXR8WH6TN8",
      "minimumSystemVersion": "13.0"
    },
    "category": "Entertainment",
    "active": true,
    "targets": "all",
    "createUpdaterArtifacts": true
  }
}
"#;
    fs::write("src-tauri/tauri.conf.json", tauri_conf).expect("Failed to write tauri.conf.json");
    println!("{}", "Debug: Configured tauri.conf.json".yellow());

    // Step 7: Reinstall npm dependencies before build
    run_command("npm", &["install"]);

    // Step 8: Build for iOS
    run_command("cargo", &["tauri", "ios", "build"]);

    println!("{}", "Tauri v2 with iOS bootstrapped successfully!".green());
    println!("To run:");
    println!("  npm run tauri dev  # For development");
    println!("  Check src-tauri/gen/apple/build/arm64/[your-app-name].ipa for the .ipa");

    // Step 9: Start iOS Simulator with a default device (iPhone 14 Pro)
    // println!("{}", "Let's start the simulation for you as well".green());

    // Boot the default device (iPhone 14 Pro) - UDID may vary, using name for simplicity
    // run_command("xcrun", &["simctl", "boot", "iPhone 15"]);
    // run_command("cargo", &["tauri", "ios", "dev","iPhone 15"]);

    // println!(
    //     "{}",
    //     "Tauri v2 with iOS bootstrapped and Simulator started successfully!".green()
    // );
    // println!("Check the Simulator for your app running on iPhone 15!");
}

fn simulator_ios() {
    println!("{}", "Let's start the simulation for you as well".green());
    // Boot the default device (iPhone 14 Pro) - UDID may vary, using name for simplicity
    // run_command("xcrun", &["simctl", "boot", "iPhone 15"]);
    run_command("cargo", &["tauri", "ios", "dev", "iPhone 15 Pro Max"]);
    println!(
        "{}",
        "Tauri v2 with iOS bootstrapped and Simulator started successfully!".green()
    );
    println!("Check the Simulator for your app running on iPhone 15 Pro Max!");
}
