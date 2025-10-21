use clap::{Arg, Command};
use crate::error::{CliError, Result};

#[derive(Debug, Clone)]
pub struct CliArgs {
    pub bpm: u32,
}

impl CliArgs {
    pub fn new(bpm: u32) -> Self {
        Self { bpm }
    }
}

pub fn build_cli() -> Command {
    Command::new("cli-metronome")
        .version("0.1.0")
        .author("CLI Metronome Team")
        .about("A cross-platform dual-mode metronome application")
        .long_about("A cross-platform metronome that supports both GUI and CLI modes.\n\
                     \n\
                     GUI MODE (default):\n\
                     Run without arguments to launch the graphical interface with:\n\
                     - BPM control with input field and buttons\n\
                     - Time signature selection (2/4, 3/4, 4/4, 6/8)\n\
                     - Sound selection (built-in and custom sounds)\n\
                     - Visual beat indicators and status display\n\
                     \n\
                     CLI MODE:\n\
                     Provide BPM as argument for command-line operation.\n\
                     Supports Windows and Linux with audio playback.\n\
                     Press Ctrl+C to stop the metronome.")
        .arg(
            Arg::new("bpm")
                .help("Beats per minute (60-200) - enables CLI mode")
                .long_help("Set the metronome tempo in beats per minute for CLI mode.\n\
                           Valid range: 60-200 BPM\n\
                           Example: 120 for a moderate tempo\n\
                           \n\
                           If omitted, the application will start in GUI mode.")
                .required(false)  // Changed to optional
                .value_parser(clap::value_parser!(u32))
                .index(1)
        )
        .after_help("EXAMPLES:\n    \
                     cli-metronome           Start GUI mode (default)\n    \
                     cli-metronome 120       Start CLI mode at 120 BPM\n    \
                     cli-metronome 80        Start CLI mode at 80 BPM\n    \
                     cli-metronome 180       Start CLI mode at 180 BPM")
}

pub fn parse_args() -> Result<Option<CliArgs>> {
    let matches = build_cli().get_matches();
    
    // Check if BPM argument was provided
    if let Some(bpm) = matches.get_one::<u32>("bpm").copied() {
        // BPM validation
        if bpm < 60 || bpm > 200 {
            return Err(CliError::InvalidArgument(
                format!("BPM must be between 60 and 200, got {}", bpm)
            ).into());
        }
        
        Ok(Some(CliArgs::new(bpm)))
    } else {
        // No BPM provided - GUI mode
        Ok(None)
    }
}


pub fn show_help() {
    let mut cmd = build_cli();
    cmd.print_help().unwrap();
    println!();
}

pub fn validate_bpm(bpm: u32) -> Result<()> {
    if bpm < 60 || bpm > 200 {
        return Err(CliError::InvalidArgument(
            format!("Invalid BPM value: {}. Must be between 60 and 200", bpm)
        ).into());
    }
    Ok(())
}#[cfg(
test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cli_args_creation() {
        let args = CliArgs::new(120);
        assert_eq!(args.bpm, 120);
    }
    
    #[test]
    fn test_bpm_validation_valid() {
        assert!(validate_bpm(60).is_ok());
        assert!(validate_bpm(120).is_ok());
        assert!(validate_bpm(200).is_ok());
    }
    
    #[test]
    fn test_bpm_validation_invalid() {
        assert!(validate_bpm(59).is_err());
        assert!(validate_bpm(201).is_err());
        assert!(validate_bpm(0).is_err());
    }
    
    #[test]
    fn test_build_cli_structure() {
        let cmd = build_cli();
        assert_eq!(cmd.get_name(), "cli-metronome");
        assert_eq!(cmd.get_version(), Some("0.1.0"));
        
        // Verify BPM argument is now optional
        let bpm_arg = cmd.get_arguments().find(|arg| arg.get_id() == "bpm");
        assert!(bpm_arg.is_some());
        assert!(!bpm_arg.unwrap().is_required_set());
    }
    
    #[test]
    fn test_parse_args_with_bpm() {
        // This test would require mocking command line arguments
        // For now, we test the CliArgs creation directly
        let args = CliArgs::new(120);
        assert_eq!(args.bpm, 120);
    }
    
    #[test]
    fn test_cli_args_optional_behavior() {
        // Test that CliArgs can be created with valid BPM
        let args = CliArgs::new(120);
        assert_eq!(args.bpm, 120);
        
        // Test BPM validation still works
        assert!(validate_bpm(120).is_ok());
        assert!(validate_bpm(59).is_err());
        assert!(validate_bpm(201).is_err());
    }
}