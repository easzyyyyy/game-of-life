# Game of Life

A learning project implementing Conway's Game of Life in Rust, focused on exploring performance optimization techniques and building intuitive user experiences.

## 🎯 Project Goals

This is a **practice ground** for learning and experimenting with:

- **Performance Optimization**: Testing various rendering and computation techniques
- **User Experience Design**: Building natural, responsive controls and interactions
- **Clean Code Architecture**: Practicing proper separation of concerns

This project is meant to evolve as I learn and discover better approaches. Nothing here is final, it's an ongoing exploration of what makes software both fast and enjoyable to use.

## 🛠️ Technical Stack

- **Language**: Rust (for memory safety and performance)
- **GUI Framework**: [egui](https://github.com/emilk/egui) v0.33.3 (immediate mode GUI)
- **Application Framework**: [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) v0.33.3
- **Parallel Processing**: [rayon](https://github.com/rayon-rs/rayon) v1.10 (for multi-threaded generation computation)

## ✨ Current Features

Experiments include:

- Viewport culling and sparse rendering
- Multi-threaded computation
- Natural pan and zoom controls (trackpad & mouse)
- Playback controls with timeline navigation
- Real-time performance monitoring

_Features will evolve as I explore better approaches._

## 🚀 Running the Project

```bash
cargo run --release
```

## 📁 Project Structure

```
src/
├── main.rs      # Application entry point
├── game.rs      # Core game logic
├── camera.rs    # Viewport management
└── ui.rs        # User interface
```

## 🔮 Ideas to Explore

Areas I'm considering for future learning:

**Performance & Rendering**

- GPU acceleration with compute shaders
- Sparse data structures for infinite grids
- WebAssembly compilation

**User Experience**

- Pattern library (gliders, spaceships, etc.)
- Color themes and visual customization
- Selection and editing tools

**Analysis & Visualization**

- Population tracking and graphs
- Pattern detection and statistics
- Export/import capabilities

**Advanced Concepts**

- Alternative cellular automata rules
- Different grid topologies
- Sound or visual effects

## 📝 License

MIT License
