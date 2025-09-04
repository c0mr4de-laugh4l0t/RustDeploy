# RustDeploy

Deploy in one command.

RustDeploy is a lightweight DevOps tool written in Rust that builds and deploys applications to the cloud with a single CLI command. It automates Docker builds, pushes images to a registry, and spins up a live service with minimal configuration.

---

## Features
- `deploy` — build and push a Docker image from any folder
- `logs` — view container logs from the deployed app
- `status` — check health and status of the app
- Optional TUI dashboard for monitoring (planned)

---

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) installed
- [Docker](https://docs.docker.com/get-docker/) installed and running
- A Docker Hub account (or alternative registry)

### Installation
Clone the repo and build with Cargo:
```bash
git clone https://github.com/<your-username>/rustdeploy.git
cd rustdeploy
cargo build --release
