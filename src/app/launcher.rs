// Application launcher and mode detection logic

use crate::cli::CliArgs;
use crate::error::Result;

/// Application mode enumeration
#[derive(Debug, Clone)]
pub enum AppMode {
    /// CLI mode with parsed arguments
    Cli(CliArgs),
    /// GUI mode (no arguments provided)
    Gui,
}

/// Determine the application mode based on command line arguments
pub fn determine_mode() -> Result<AppMode> {
    // Parse CLI arguments - returns None if no BPM provided (GUI mode)
    match crate::cli::parse_args()? {
        Some(cli_args) => Ok(AppMode::Cli(cli_args)),
        None => Ok(AppMode::Gui),
    }
}

/// Launch the application in the appropriate mode
pub fn launch_app(mode: AppMode) -> Result<()> {
    match mode {
        AppMode::Cli(cli_args) => {
            launch_cli_mode(cli_args)
        }
        AppMode::Gui => {
            launch_gui_mode()
        }
    }
}

/// Launch the application in CLI mode
fn launch_cli_mode(cli_args: CliArgs) -> Result<()> {
    use crate::metronome::MetronomeController;
    use crate::display::DisplayEngine;

    use std::time::Instant;
    use std::thread;
    
    // Note: Help is handled by clap automatically, so we don't need to check for it here
    
    // Initialize components
    let mut controller = MetronomeController::new(cli_args.bpm)?;
    let display = DisplayEngine::new();
    let mut audio = crate::audio::CrossPlatformAudio::new();
    
    // Get initial state for display
    let initial_state = controller.get_metronome().get_state();
    
    // Initialize audio system
    let audio_status = if let Err(e) = audio.initialize() {
        eprintln!("Warning: Audio initialization failed: {}", e);
        eprintln!("Continuing in visual-only mode...");
        crate::audio::AudioStatus::Disabled
    } else {
        crate::audio::AudioStatus::Available
    };
    
    // Show startup information with time signature
    display.show_startup_info(cli_args.bpm, initial_state.time_signature, &audio_status);
    
    // Setup signal handling
    controller.setup_ctrl_c_handler()?;
    
    // Start metronome
    controller.start()?;
    
    let mut last_beat_time = Instant::now();
    
    // Main loop
    while controller.should_continue() {
        let should_beat = {
            let metronome = controller.get_metronome();
            metronome.should_play_beat(last_beat_time)
        };
        
        if should_beat {
            let beat = controller.get_metronome_mut().increment_beat();
            
            // Play audio if available
            if audio.is_audio_available() {
                if let Err(e) = audio.play_beat_sound() {
                    eprintln!("Audio playback error: {}", e);
                }
            }
            
            // Show enhanced visual indicator with beat information
            display.show_visual_beat(&beat);
            
            // Show enhanced status with time signature and beat position
            let state = {
                let metronome = controller.get_metronome();
                metronome.get_state()
            };
            display.show_status(
                state.bpm, 
                state.beat_count, 
                state.get_elapsed_time(),
                state.time_signature,
                state.current_beat_in_measure
            );
            
            last_beat_time = Instant::now();
        }
        
        // Small sleep to prevent excessive CPU usage
        thread::sleep(std::time::Duration::from_millis(1));
    }
    
    controller.stop();
    display.show_goodbye();
    Ok(())
}

/// Launch the application in GUI mode
fn launch_gui_mode() -> Result<()> {
    #[cfg(feature = "gui")]
    {
        use crate::gui::MetronomeApp;
        use crate::error::GuiError;
        
        println!("Starting GUI metronome...");
        
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([680.0, 580.0])
                .with_min_inner_size([580.0, 480.0])
                .with_title("CLI Metronome"),
            ..Default::default()
        };
        
        let app = MetronomeApp::new();
        
        eframe::run_native(
            "Metronome",
            options,
            Box::new(|_cc| Box::new(app)),
        ).map_err(|e| GuiError::InitializationFailed(format!("Failed to start GUI: {}", e)))?;
        
        Ok(())
    }
    
    #[cfg(not(feature = "gui"))]
    {
        eprintln!("GUI mode is not available. This build was compiled without GUI support.");
        eprintln!("Please provide BPM as an argument to use CLI mode.");
        eprintln!("Example: {} 120", std::env::args().next().unwrap_or_else(|| "metronome".to_string()));
        Err(crate::MetronomeError::SystemError(
            "GUI feature not enabled".to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_app_mode_variants() {
        let cli_args = CliArgs { bpm: 120 };
        let cli_mode = AppMode::Cli(cli_args);
        let gui_mode = AppMode::Gui;
        
        match cli_mode {
            AppMode::Cli(args) => assert_eq!(args.bpm, 120),
            _ => panic!("Expected CLI mode"),
        }
        
        match gui_mode {
            AppMode::Gui => (),
            _ => panic!("Expected GUI mode"),
        }
    }
    
    #[test]
    fn test_determine_mode_logic() {
        // This test demonstrates the logic, but actual testing would require
        // mocking command line arguments, which is better done in integration tests
        let cli_args = CliArgs { bpm: 120 };
        let cli_mode = AppMode::Cli(cli_args);
        let gui_mode = AppMode::Gui;
        
        // Verify that modes can be created and matched
        match cli_mode {
            AppMode::Cli(args) => assert_eq!(args.bpm, 120),
            _ => panic!("Expected CLI mode"),
        }
        
        match gui_mode {
            AppMode::Gui => (),
            _ => panic!("Expected GUI mode"),
        }
    }
}