use cli_metronome::{
    app::{determine_mode, launch_app},
    MetronomeError,
};

fn main() {
    // Setup cleanup on exit
    let result = std::panic::catch_unwind(|| {
        if let Err(e) = run() {
            handle_application_error(&e);
            std::process::exit(1);
        }
    });
    
    // Ensure cleanup happens even if panic occurs
    cleanup_resources();
    
    if result.is_err() {
        eprintln!("Application panicked. Exiting...");
        std::process::exit(1);
    }
}

fn run() -> Result<(), MetronomeError> {
    // Determine application mode based on command line arguments
    let mode = determine_mode()?;
    
    // Launch application in appropriate mode
    launch_app(mode)?;
    
    Ok(())
}

// Cleanup resources before exit
fn cleanup_resources() {
    // Flush any remaining output
    use std::io::{self, Write};
    let _ = io::stdout().flush();
    let _ = io::stderr().flush();
    
    // Additional cleanup can be added here if needed
    // For example: closing files, network connections, etc.
}

// Enhanced error handling for the application
fn handle_application_error(error: &MetronomeError) {
    match error {
        MetronomeError::InvalidBpm(bpm) => {
            eprintln!("Invalid BPM value: {}. Please use a value between 60 and 200.", bpm);
        }
        MetronomeError::AudioError(audio_err) => {
            eprintln!("Audio system error: {}", audio_err);
            eprintln!("The metronome will continue in visual-only mode.");
        }
        MetronomeError::CliError(cli_err) => {
            eprintln!("Command line error: {}", cli_err);
            eprintln!("Use --help for usage information.");
        }
        MetronomeError::SystemError(sys_err) => {
            eprintln!("System error: {}", sys_err);
        }
        MetronomeError::ConfigError(config_err) => {
            eprintln!("Configuration error: {}", config_err);
        }
        MetronomeError::GuiError(gui_err) => {
            eprintln!("GUI error: {}", gui_err);
            eprintln!("The application will attempt to continue or fall back to CLI mode.");
        }
        MetronomeError::InvalidVolume(volume) => {
            eprintln!("Invalid volume value: {}. Please use a value between 0.0 and 1.0.", volume);
        }
    }
}