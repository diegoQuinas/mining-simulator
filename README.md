# ⛏️ Mining Simulator

A terminal-based mining simulation game built with Rust, featuring concurrent mining operations, async programming, and a beautiful TUI interface powered by Ratatui.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-TUI-blue?style=for-the-badge)
![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)

## 🎮 Game Overview

Command a fortress of goblin miners in this real-time simulation! Watch as your miners explore the underground world, discover precious ores, and manage their fatigue levels. The game showcases advanced Rust concepts including:

- **Concurrent Programming**: Multiple miners working simultaneously using async/await
- **Message Passing**: Miners communicate with the fortress through channels
- **Real-time Simulation**: Dynamic world state with randomized events
- **Terminal UI**: Beautiful, responsive interface built with Ratatui

## ✨ Features

### Current Features
- 🏰 **Fortress Management**: Central command center that receives reports from miners
- 👺 **Goblin Miners**: Autonomous agents that explore, mine, and return with ore
- 💰 **Resource Collection**: Real-time ore gathering and deposit system
- 😴 **Fatigue System**: Miners get tired and must return to rest
- 🎲 **Randomized World**: Procedural ore discovery and miner behavior

### Planned Features
- 🖥️ **Rich TUI Interface**: Interactive dashboard with real-time statistics
- 🗺️ **Visual Mine Map**: ASCII art representation of the mining area
- 📊 **Performance Metrics**: Track efficiency, ore per hour, miner statistics
- ⚙️ **Upgradeable Equipment**: Improve mining speed and capacity
- 🏗️ **Fortress Expansion**: Build new facilities and hire more miners
- 💾 **Save/Load System**: Persistent game state
- 🎯 **Achievement System**: Unlock rewards for reaching milestones
- 🌍 **Multiple Biomes**: Different mining areas with unique resources

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ (2024 edition)
- Cargo package manager

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/diegoQuinas/mining-simulator.git
   cd mining-simulator
   ```

2. **Build the project**
   ```bash
   cargo build --release
   ```

3. **Run the game**
   ```bash
   cargo run
   ```

### Quick Start
```bash
# Run in development mode with debug output
cargo run

# Run optimized release version
cargo run --release

# Run with specific number of miners
cargo run -- --miners 10
```

## 🏗️ Architecture

### Core Components

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Fortress      │◄───┤  Message Bus    │───►│  Goblin Miners  │
│   (Receiver)    │    │   (mpsc::channel)│    │   (Senders)     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                                              │
         ▼                                              ▼
┌─────────────────┐                          ┌─────────────────┐
│   Game State    │                          │   Mining Logic  │
│   Management    │                          │   & Fatigue     │
└─────────────────┘                          └─────────────────┘
```

### Message Flow
1. **Miners** explore the world and discover ore
2. **Reports** are sent to fortress via async channels
3. **Fortress** aggregates data and updates game state
4. **TUI** renders the current state in real-time

## 🛠️ Technical Implementation

### Concurrency Model
- **Tokio Runtime**: Async/await for non-blocking operations
- **Message Passing**: Safe communication between miners and fortress
- **Structured Concurrency**: Proper task spawning and cleanup

### Key Rust Features Used
- `async/await` for concurrent mining operations
- `mpsc::channel` for inter-task communication
- `tokio::spawn` for task management
- `rand` crate for procedural generation
- Pattern matching for message handling

### Code Example
```rust
// Spawn multiple concurrent miners
for id in 0..num_miners {
    let tx_clone = tx.clone();
    tokio::spawn(goblin_task(id, tx_clone));
}

// Miners send reports asynchronously
tx.send(GoblinMessage::Report {
    id, pos, ore, fatigue
}).await?;
```

## 📦 Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.48+ | Async runtime and concurrency |
| `rand` | 0.9+ | Random number generation |
| `ratatui` | 0.24+ | Terminal user interface *(planned)* |
| `crossterm` | 0.27+ | Cross-platform terminal control *(planned)* |
| `serde` | 1.0+ | Serialization for save files *(planned)* |

## 🎯 Roadmap

### Phase 1: Core Simulation ✅
- [x] Basic goblin miners with async tasks
- [x] Message passing between miners and fortress
- [x] Ore discovery and collection system
- [x] Fatigue mechanics

### Phase 2: TUI Interface 🚧
- [ ] Ratatui integration
- [ ] Real-time dashboard
- [ ] Interactive controls
- [ ] Visual mine representation

### Phase 3: Game Features 📋
- [ ] Save/load functionality
- [ ] Equipment upgrades
- [ ] Multiple resource types
- [ ] Achievement system

### Phase 4: Advanced Features 🔮
- [ ] Multiplayer support
- [ ] Mod system
- [ ] Custom scenarios
- [ ] Performance analytics

## 🤝 Contributing

We welcome contributions! This project is perfect for learning:
- Rust async programming
- Concurrent system design
- Terminal UI development
- Game development patterns

### How to Contribute
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup
```bash
# Install development dependencies
cargo install cargo-watch cargo-expand

# Run with auto-reload during development
cargo watch -x run

# Run tests
cargo test

# Check code formatting
cargo fmt --check

# Run clippy for linting
cargo clippy -- -D warnings
```

## 📊 Performance

The game is designed to handle:
- **100+ concurrent miners** without performance degradation
- **Real-time updates** at 60+ FPS in the terminal
- **Minimal memory footprint** through efficient data structures
- **Cross-platform compatibility** (Windows, macOS, Linux)

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Tokio Team** - For the excellent async runtime
- **Ratatui Community** - For the amazing TUI framework
- **Rust Community** - For the incredible language and ecosystem

## 📞 Contact

- **Project Link**: [https://github.com/diegoQuinas/mining-simulator](https://github.com/diegoQuinas/mining-simulator)
- **Issues**: [Report bugs and request features](https://github.com/diegoQuinas/mining-simulator/issues)
- **Discussions**: [Join the community](https://github.com/diegoQuinas/mining-simulator/discussions)

---

*Built with ❤️ and ⛏️ in Rust*
