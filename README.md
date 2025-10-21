# CLI Metronome

A cross-platform dual-mode metronome application written in Rust. Features both a graphical user interface (GUI) and command-line interface (CLI) with accurate timing, multiple time signatures, and customizable sounds for musicians and music practice.

## Features

### Core Features
- 🎵 **Accurate Timing**: High-precision metronome with BPM range from 60 to 200
- 🖥️ **Dual Mode**: Both GUI and CLI interfaces for different use cases
- 🎼 **Time Signatures**: Support for 2/4, 3/4, 4/4, and 6/8 time signatures
- 🔊 **Multiple Sounds**: Built-in sounds (Click, Wood, Beep) plus custom sound file support
- 🎯 **Strong/Weak Beats**: Accent patterns based on time signature (strong, medium, weak beats)
- 👁️ **Visual Indicators**: Real-time visual beat indicators and status display
- 🖥️ **Cross-Platform**: Works on Windows and Linux
- ⚡ **Lightweight**: Minimal resource usage with efficient Rust implementation
- 🛡️ **Robust**: Graceful fallback to visual-only mode when audio is unavailable

### GUI Mode Features
- 🖱️ **Intuitive Interface**: Easy-to-use graphical controls
- 🎛️ **BPM Controls**: Input field with increment/decrement buttons
- 🎼 **Time Signature Selection**: Dropdown menu for time signature selection
- 🔊 **Sound Selection**: Separate controls for beat and accent sounds
- ▶️ **Start/Stop Controls**: Simple playback controls
- 📊 **Real-time Display**: Live BPM, time, beat count, and visual beat indicators
- 🎨 **Beat Visualization**: Color-coded beat strength indicators
- 🔧 **Sound Testing**: Test button for each sound type

### CLI Mode Features
- ⚡ **Fast Startup**: Quick command-line operation
- 📊 **Enhanced Display**: Detailed beat pattern visualization
- 🎨 **Color-coded Beats**: Terminal colors for strong/medium/weak beats
- 📈 **Progress Indicators**: Measure progress bars and beat position tracking
- 🎼 **Time Signature Legends**: Visual explanation of beat patterns

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### From Source

1. Clone the repository:
```bash
git clone <repository-url>
cd cli-metronome
```

2. Build the application:
```bash
cargo build --release
```

3. (Optional) Install globally:
```bash
cargo install --path .
```

### Build Options

The application supports different feature combinations:

```bash
# Full build with GUI and audio support (default)
cargo build --release

# GUI only (no audio)
cargo build --release --no-default-features --features gui

# CLI only (no GUI, with audio)
cargo build --release --no-default-features --features audio

# Minimal build (CLI only, no audio)
cargo build --release --no-default-features
```

### Windows Build Notes

On Windows, the application includes specific dependencies for GUI support. If you encounter build errors related to `winapi` or `eframe`, ensure you have:

1. **Visual Studio Build Tools** or **Visual Studio Community** installed
2. **Windows SDK** installed
3. Run the build in a **Developer Command Prompt** or **PowerShell**

```powershell
# Windows PowerShell build command
cargo build --release --features gui
```

The `Cargo.toml` includes Windows-specific dependencies that should resolve most compilation issues automatically.

### Audio Support

The metronome includes optional audio support with multiple built-in sounds and custom file support. If audio libraries are not available on your system, the application will automatically fall back to visual-only mode.

#### Linux Audio Setup

For audio support on Linux, you may need to install ALSA development libraries:

```bash
# Ubuntu/Debian
sudo apt-get install libasound2-dev

# Fedora/RHEL
sudo dnf install alsa-lib-devel

# Arch Linux
sudo pacman -S alsa-lib
```

## Usage

The metronome operates in two modes depending on how you launch it:

### GUI Mode (Default)

Launch the graphical interface by running without arguments:

```bash
# Start GUI mode
cli-metronome
```

#### GUI Controls

- **BPM Input**: Enter BPM directly or use +/- buttons (60-200 range)
- **Time Signature**: Select from 2/4, 3/4, 4/4, or 6/8 time signatures
- **Beat Sound**: Choose sound for regular beats (Click, Wood, Beep)
- **Accent Sound**: Choose sound for strong beats (Click, Wood, Beep)
- **Start/Stop**: Control metronome playback
- **Reset**: Reset beat count and position
- **Test Sounds**: Preview selected sounds

#### GUI Features

- Real-time BPM, elapsed time, and beat count display
- Visual beat indicators with color-coded strength:
  - 🔴 **Red**: Strong beats (downbeats)
  - 🟡 **Yellow**: Medium beats (partial accents)
  - ⚪ **White**: Weak beats
- Beat position within measure (e.g., "Beat 2/4")
- Measure progress visualization
- Audio status indicator

### CLI Mode

Launch command-line mode by providing a BPM argument:

```bash
# Start CLI mode at 120 BPM
cli-metronome 120

# Start CLI mode at 80 BPM
cli-metronome 80

# Start CLI mode at 180 BPM
cli-metronome 180
```

#### Command Line Options

```bash
cli-metronome [BPM]

Arguments:
  [BPM]  Beats per minute (60-200) - enables CLI mode

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

#### CLI Controls

- **Ctrl+C**: Stop the metronome and exit

#### CLI Display Features

- Enhanced beat pattern visualization with time signature legends
- Color-coded beat indicators (when terminal supports colors):
  - 🔴 **●**: Strong beats (red)
  - 🟡 **◐**: Medium beats (yellow)  
  - ⚪ **○**: Weak beats (gray)
- Real-time status with BPM, beat count, elapsed time
- Beat position within measure
- Measure progress bars

### Time Signatures

The metronome supports four common time signatures with appropriate accent patterns:

#### 2/4 Time (Two-Four)
- **Pattern**: Strong-weak
- **Visual**: ● ○
- **Use**: Marches, polkas

#### 3/4 Time (Three-Four) 
- **Pattern**: Strong-weak-weak
- **Visual**: ● ○ ○
- **Use**: Waltzes, folk music

#### 4/4 Time (Four-Four)
- **Pattern**: Strong-weak-medium-weak
- **Visual**: ● ○ ◐ ○
- **Use**: Most popular music, rock, pop

#### 6/8 Time (Six-Eight)
- **Pattern**: Strong-weak-weak-medium-weak-weak
- **Visual**: ● ○ ○ ◐ ○ ○
- **Use**: Compound time, ballads, folk music

### Sound Options

#### Built-in Sounds
- **Click**: Sharp, high-frequency click sound
- **Wood**: Warm wood block sound with harmonics
- **Beep**: Clean sine wave beep

#### Custom Sounds
- Support for WAV, MP3, and OGG audio files
- Automatic fallback to built-in sounds if custom files fail to load
- File size limit: 10MB per sound file

### Display Examples

#### CLI Mode Output
```
CLI Metronome v0.1.0
==================
BPM: 120
Time Signature: 4/4
Audio Status: Audio available
Press Ctrl+C to stop

Beat Pattern Legend:
  ● = Strong beat (accent)
  ◐ = Medium beat (partial accent)  
  ○ = Weak beat (no accent)

  4/4 Time Signature:
    Beat positions: 1 2 3 4
    Pattern: ● ○ ◐ ○
    Description: Strong-weak-medium-weak (common time)

BPM: 120 | Beat:   42 | Time: 00:21 | 4/4: [●] ○ ◐ ○ (1/4) | STRONG
```

#### GUI Mode
The GUI provides an intuitive interface with:
- BPM input field and adjustment buttons
- Time signature dropdown menu
- Sound selection dropdowns with test buttons
- Start/Stop/Reset controls
- Real-time beat visualization with colored indicators
- Status display showing current settings and elapsed time

## Examples

### GUI Mode Examples

```bash
# Launch GUI for interactive use
cli-metronome

# The GUI allows you to:
# - Set BPM using input field or +/- buttons
# - Select time signature (2/4, 3/4, 4/4, 6/8)
# - Choose different sounds for beats and accents
# - Start/stop with visual feedback
# - See real-time beat visualization
```

### CLI Mode Examples

```bash
# Practice session at 120 BPM in 4/4 time
cli-metronome 120

# Slow practice at 60 BPM
cli-metronome 60

# Fast practice at 180 BPM  
cli-metronome 180

# Get help and see all options
cli-metronome --help
```

### Use Cases

#### For Musicians
- **Practice Sessions**: Use GUI mode for interactive practice with visual feedback
- **Performance Preparation**: Use CLI mode for focused, distraction-free practice
- **Different Genres**: Select appropriate time signatures (3/4 for waltzes, 6/8 for ballads)
- **Accent Training**: Use different sounds for beats and accents to improve timing

#### For Music Teachers
- **Demonstration**: GUI mode for showing students beat patterns visually
- **Exercises**: Different time signatures for rhythm training
- **Custom Sounds**: Load specific sounds that match lesson requirements

#### For Composers
- **Tempo Testing**: Quick CLI access to test different tempos
- **Time Signature Exploration**: Easy switching between time signatures
- **Reference Tool**: Visual beat patterns for composition reference

## Technical Details

### Architecture

The application is built with a modular architecture:

- **CLI Parser**: Command-line argument parsing and validation
- **Metronome Core**: High-precision timing and beat counting
- **Audio Engine**: Cross-platform audio playback with fallback support
- **Display Engine**: Console output and visual indicators

### Timing Accuracy

The metronome uses Rust's `std::time::Instant` for high-precision timing with drift correction to maintain accuracy over long periods.

### Cross-Platform Support

- **Windows**: Uses WASAPI for audio output
- **Linux**: Uses ALSA/PulseAudio for audio output
- **Fallback**: Visual-only mode when audio is unavailable

## Development

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# With audio support
cargo build --release --features audio
```

### Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run specific test suites
cargo test --test integration_tests  # Integration tests
cargo test --test gui_tests          # GUI functionality tests
cargo test --test audio_tests        # Audio engine tests
cargo test --test dual_mode_tests    # Dual mode integration tests

# Run tests with GUI features
cargo test --features gui

# Run tests with audio features
cargo test --features audio

# Run all tests with all features
cargo test --all-features
```

### Code Structure

```
src/
├── main.rs           # Application entry point
├── lib.rs            # Library root
├── app/              # Application launcher and mode detection
│   ├── mod.rs        # Module exports
│   └── launcher.rs   # Mode detection and app launching
├── cli/              # Command-line interface
│   └── mod.rs        # CLI argument parsing
├── gui/              # Graphical user interface (optional)
│   ├── mod.rs        # GUI module exports
│   └── app.rs        # GUI application implementation
├── metronome/        # Core metronome logic
│   └── mod.rs        # Metronome and controller implementation
├── audio/            # Audio playback engine
│   └── mod.rs        # Cross-platform audio with multiple sounds
├── display/          # Console display engine
│   └── mod.rs        # Enhanced CLI display with time signatures
├── models.rs         # Data models (Beat, TimeSignature, SoundType, etc.)
└── error.rs          # Error types

tests/
├── integration_tests.rs  # Integration tests
├── gui_tests.rs          # GUI functionality tests
├── audio_tests.rs        # Audio engine tests
└── dual_mode_tests.rs    # Dual mode integration tests
```

## Troubleshooting

### Mode Selection Issues

**Problem**: GUI doesn't start
**Solution**: 
1. Ensure the application was built with GUI support: `cargo build --features gui`
2. Check if GUI libraries are available on your system
3. The application will show an error message and suggest using CLI mode

**Problem**: Want to force CLI mode
**Solution**: Always provide a BPM argument to use CLI mode: `cli-metronome 120`

### Windows Build Issues

**Problem**: Build fails with `winapi` or `eframe` errors
**Solution**:
1. Install Visual Studio Build Tools or Visual Studio Community
2. Ensure Windows SDK is installed
3. Use Developer Command Prompt or PowerShell
4. Try building with explicit features: `cargo build --release --features gui`

**Problem**: `error: could not compile 'eframe'` on Windows
**Solution**:
1. Update Rust to the latest version: `rustup update`
2. Clear cargo cache: `cargo clean`
3. Rebuild: `cargo build --release`
4. If issues persist, try building without GUI: `cargo build --release --no-default-features --features audio`

**Problem**: Missing Windows dependencies
**Solution**:
The `Cargo.toml` includes Windows-specific `winapi` features. If you still encounter issues:
1. Check that you have the latest Windows SDK
2. Ensure Visual C++ redistributables are installed
3. Try building in a clean environment

### Audio Issues

**Problem**: No audio output
**Solution**: 
1. Check if audio libraries are installed (see Installation section)
2. The application will automatically fall back to visual-only mode
3. Try building with `--features audio` for full audio support
4. In GUI mode, check the audio status indicator

**Problem**: Audio initialization failed
**Solution**: 
- On Linux: Install ALSA development libraries
- On Windows: Ensure audio drivers are properly installed
- The application will continue in visual-only mode

**Problem**: Custom sound files won't load
**Solution**:
1. Ensure file format is supported (WAV, MP3, OGG)
2. Check file size is under 10MB
3. Verify file path is correct
4. The application will fall back to built-in sounds automatically

### GUI Issues

**Problem**: GUI controls not responding
**Solution**:
1. Ensure BPM input is valid (60-200)
2. Check that time signature and sounds are properly selected
3. Try resetting the metronome with the Reset button

**Problem**: Beat visualization not working
**Solution**:
1. Ensure metronome is started
2. Check that visual indicators are enabled
3. Try different time signatures to see beat patterns

### Performance Issues

**Problem**: High CPU usage
**Solution**: This is typically not an issue, but if encountered:
1. Check system load
2. Ensure no other high-priority audio applications are running
3. Try using CLI mode for lower resource usage

### BPM Validation Errors

**Problem**: "Invalid BPM value" error
**Solution**: Ensure BPM is between 60 and 200 (inclusive)

```bash
# ✅ Valid
cli-metronome 120

# ❌ Invalid
cli-metronome 300  # Too high
cli-metronome 30   # Too low
```

### Time Signature Issues

**Problem**: Beat pattern doesn't match expectation
**Solution**:
1. Check the time signature legend in CLI mode
2. Verify the selected time signature in GUI mode
3. Remember that 6/8 time has 6 eighth-note beats, not 6 quarter-note beats

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass: `cargo test`
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Changelog

### v0.1.0
- **Dual Mode Interface**: Both GUI and CLI modes
- **Time Signature Support**: 2/4, 3/4, 4/4, and 6/8 time signatures
- **Multiple Sound Options**: Built-in sounds (Click, Wood, Beep) plus custom file support
- **Strong/Weak Beat Patterns**: Accent patterns based on time signature
- **Enhanced Visual Indicators**: Color-coded beat strength indicators
- **GUI Features**:
  - Intuitive graphical interface with egui/eframe
  - BPM controls with input field and buttons
  - Time signature and sound selection dropdowns
  - Real-time beat visualization and status display
  - Sound testing capabilities
- **CLI Enhancements**:
  - Enhanced display with time signature legends
  - Color-coded beat indicators
  - Measure progress visualization
  - Beat position tracking
- **Audio Engine**: 
  - Multiple built-in sound generation
  - Custom sound file support (WAV, MP3, OGG)
  - Sound caching system
  - Graceful fallback handling
- **Cross-platform Support**: Windows and Linux
- **Comprehensive Testing**: GUI, audio, and dual-mode integration tests
- **BPM Range**: 60-200 with validation
- **Robust Error Handling**: Graceful degradation and user-friendly error messages