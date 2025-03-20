# ShipNext

A utility tool for managing your NextJS projects.

## Installation

### Pre-built Binary

You can download the pre-built binary directly:

```bash
# Download the binary
curl -L https://github.com/vanha777/shipnext-binary-cli/releases/download/alpha/shipnext -o shipnext

# Make it executable
chmod +x shipnext

# Move to a directory in your PATH (optional)
sudo mv shipnext /usr/local/bin/
```

### Building from Source

1. Clone this repository:
   ```bash
   git clone https://github.com/yourusername/shipnext.git
   cd shipnext
   ```

2. Build the binary for your platform:

   For macOS ARM (M1/M2/M3):
   ```bash
   cargo build --release --target aarch64-apple-darwin
   ```

   For macOS Intel:
   ```bash
   cargo build --release --target x86_64-apple-darwin
   ```

   For Linux:
   ```bash
   cargo build --release --target x86_64-unknown-linux-gnu
   ```

   For Windows:
   ```bash
   cargo build --release --target x86_64-pc-windows-msvc
   ```

3. Copy the binary to your NextJS project:
   ```bash
   # Replace with your NextJS project path
   cp target/aarch64-apple-darwin/release/shipnext /path/to/your/nextjs/project/
   ```

4. Make the binary executable:
   ```bash
   cd /path/to/your/nextjs/project/
   chmod +x shipnext
   ```

5. Now you can run the tool:
   ```bash
   ./shipnext status
   ./shipnext modify
   ./shipnext bootstrap
   ```

## iOS Development Prerequisites

If you want to develop or run a Tauri iOS app on macOS, you'll need:

1. **macOS:** You need macOS 12 (Monterey) or later.

2. **Xcode:** Install the latest version of Xcode from the Mac App Store or Apple's developer website.
   ```bash
   xcode-select --install
   ```

3. **iOS Simulator:** Included with Xcode, needed for testing without a physical device.

4. **Apple Developer Account:** Required for deploying to physical devices and App Store submission.

5. **Rust Setup:**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # Add iOS targets
   rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
   ```

6. **Node.js and npm/yarn:** Required for the web portion of your Tauri app.

7. **Tauri CLI:**
   ```bash
   cargo install tauri-cli
   ```

8. **iOS-specific dependencies:**
   ```bash
   # Install CocoaPods
   sudo gem install cocoapods
   
   # Additional dependencies
   brew install libiconv
   ```

## Usage
