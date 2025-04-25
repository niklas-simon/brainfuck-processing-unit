#!/bin/bash

# Install required packages
sudo apt update -y
sudo apt install git -y

# Set Wifi Country to DE
sudo iw reg set DE

# Install Rust
curl https://sh.rustup.rs -sSf | sh -s -- -y
. "$HOME/.cargo/env"

# Install Node.js
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.1/install.sh | bash
export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh" # This loads nvm
nvm install 22.11.0

# Install pnpm
curl -fsSL https://get.pnpm.io/install.sh | sh -
source /home/pi/.bashrc

# Clone Repository
git clone https://github.com/niklas-simon/brainfuck-processing-unit
cd brainfuck-processing-unit

# Build Client
cd interface/client
pnpm i
pnpm run build

# Build Server
cd ../server
cargo build --release

# Copy stuff to /etc/bfpu
cd ..
sudo mkdir -p /etc/bfpud
sudo cp server/target/release/bf_itp  /etc/bfpud/
sudo mkdir -p /etc/bfpud/static
sudo cp -r client/dist/* /etc/bfpud/static

# Configure and start service
sudo cp raspberry/bfpud.service /etc/systemd/system/
sudo cp raspberry/bfpu_chromium.service /etc/systemd/system/
sudo cp raspberry/bfpu_hotspot.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable bfpud.service
sudo systemctl start bfpud.service