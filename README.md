# ShipNext

A utility tool for managing your NextJS projects.

## Installation

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
   ```

## Usage
