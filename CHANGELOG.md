# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-10-20

### Added
- Initial release of CLI Metronome
- Core metronome functionality with BPM range 60-200
- Cross-platform audio support using rodio crate
- Visual-only fallback mode when audio is unavailable
- Real-time status display showing BPM, beat count, and elapsed time
- Visual beat indicators (● for downbeats, ○ for other beats)
- Command-line argument parsing with clap
- Comprehensive error handling and user-friendly error messages
- Ctrl+C signal handling for graceful shutdown
- Modular architecture with separate CLI, metronome, audio, and display modules
- Extensive unit test coverage (19 tests)
- Integration tests for cross-platform compatibility
- Detailed README with installation and usage instructions

### Features
- **Accurate Timing**: High-precision timing using `std::time::Instant`
- **Cross-Platform**: Works on Windows and Linux
- **Audio Support**: Optional audio playback with automatic fallback
- **Visual Indicators**: Real-time beat visualization
- **Robust Error Handling**: Graceful handling of invalid inputs and system errors
- **Lightweight**: Minimal resource usage

### Technical Details
- Written in Rust 2021 edition
- Uses clap for CLI argument parsing
- Uses rodio for cross-platform audio (optional feature)
- Uses thiserror for structured error handling
- Uses ctrlc for signal handling
- Comprehensive test suite with both unit and integration tests

### Supported Platforms
- Linux (with optional ALSA support)
- Windows (with WASAPI support)
- Visual-only mode available on all platforms