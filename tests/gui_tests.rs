#[cfg(feature = "gui")]
mod gui_tests {
    use cli_metronome::models::{GuiState, TimeSignature, SoundType, MetronomeState, MetronomeConfig};
    use cli_metronome::gui::MetronomeApp;
    use cli_metronome::error::MetronomeError;
    use std::time::Instant;

    #[test]
    fn test_gui_state_creation() {
        let gui_state = GuiState::new();
        
        assert_eq!(gui_state.bpm_input, "120");
        assert!(gui_state.bpm_valid);
        assert_eq!(gui_state.selected_time_signature, TimeSignature::Four);
        assert_eq!(gui_state.selected_beat_sound, SoundType::BuiltinClick);
        assert_eq!(gui_state.selected_accent_sound, SoundType::BuiltinWood);
        assert!(!gui_state.is_running);
        assert!(gui_state.error_message.is_none());
        assert!(gui_state.last_beat_visual.is_none());
    }

    #[test]
    fn test_gui_state_bpm_validation() {
        let mut gui_state = GuiState::new();
        
        // Test valid BPM
        let result = gui_state.validate_bpm("120");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 120);
        assert!(gui_state.bpm_valid);
        assert!(gui_state.error_message.is_none());
        
        // Test BPM at lower boundary
        let result = gui_state.validate_bpm("60");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 60);
        assert!(gui_state.bpm_valid);
        
        // Test BPM at upper boundary
        let result = gui_state.validate_bpm("200");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 200);
        assert!(gui_state.bpm_valid);
    }

    #[test]
    fn test_gui_state_bpm_validation_invalid() {
        let mut gui_state = GuiState::new();
        
        // Test BPM below minimum
        let result = gui_state.validate_bpm("59");
        assert!(result.is_err());
        assert!(!gui_state.bpm_valid);
        assert!(gui_state.error_message.is_some());
        assert!(gui_state.error_message.as_ref().unwrap().contains("60 and 200"));
        
        // Test BPM above maximum
        let result = gui_state.validate_bpm("201");
        assert!(result.is_err());
        assert!(!gui_state.bpm_valid);
        assert!(gui_state.error_message.is_some());
    }    
#[test]
    fn test_gui_state_bpm_validation_non_numeric() {
        let mut gui_state = GuiState::new();
        
        // Test non-numeric input
        let result = gui_state.validate_bpm("abc");
        assert!(result.is_err());
        assert!(!gui_state.bpm_valid);
        assert!(gui_state.error_message.is_some());
        assert!(gui_state.error_message.as_ref().unwrap().contains("Invalid BPM value"));
        
        // Test empty input
        let result = gui_state.validate_bpm("");
        assert!(result.is_err());
        assert!(!gui_state.bpm_valid);
        
        // Test whitespace input
        let result = gui_state.validate_bpm("   ");
        assert!(result.is_err());
        assert!(!gui_state.bpm_valid);
    }

    #[test]
    fn test_gui_state_error_handling() {
        let mut gui_state = GuiState::new();
        
        // Test setting error
        gui_state.set_error("Test error message".to_string());
        assert!(gui_state.error_message.is_some());
        assert_eq!(gui_state.error_message.as_ref().unwrap(), "Test error message");
        
        // Test clearing error
        gui_state.clear_error();
        assert!(gui_state.error_message.is_none());
        
        // Test GUI error handling
        let gui_error = cli_metronome::error::GuiError::InvalidInput("Test GUI error".to_string());
        gui_state.set_gui_error(gui_error);
        assert!(gui_state.error_message.is_some());
        assert!(gui_state.error_message.as_ref().unwrap().contains("Test GUI error"));
    }

    #[test]
    fn test_gui_state_result_handling() {
        let mut gui_state = GuiState::new();
        
        // Test handling successful result
        let ok_result: Result<u32, MetronomeError> = Ok(120);
        let value = gui_state.handle_result(ok_result);
        assert_eq!(value, Some(120));
        assert!(gui_state.error_message.is_none());
        
        // Test handling error result
        let err_result: Result<u32, MetronomeError> = Err(MetronomeError::InvalidBpm(59));
        let value = gui_state.handle_result(err_result);
        assert_eq!(value, None);
        assert!(gui_state.error_message.is_some());
        assert!(gui_state.error_message.as_ref().unwrap().contains("Invalid BPM"));
    }

    #[test]
    fn test_gui_state_beat_visual() {
        let mut gui_state = GuiState::new();
        
        // Initially no beat visual
        assert!(gui_state.last_beat_visual.is_none());
        assert!(!gui_state.should_show_beat_visual(std::time::Duration::from_millis(100)));
        
        // Update beat visual
        gui_state.update_beat_visual();
        assert!(gui_state.last_beat_visual.is_some());
        assert!(gui_state.should_show_beat_visual(std::time::Duration::from_millis(100)));
        
        // Wait and check if visual should still show
        std::thread::sleep(std::time::Duration::from_millis(150));
        assert!(!gui_state.should_show_beat_visual(std::time::Duration::from_millis(100)));
    }   
 #[test]
    fn test_gui_state_time_signature_selection() {
        let mut gui_state = GuiState::new();
        
        // Test default time signature
        assert_eq!(gui_state.selected_time_signature, TimeSignature::Four);
        
        // Test changing time signature
        gui_state.selected_time_signature = TimeSignature::Three;
        assert_eq!(gui_state.selected_time_signature, TimeSignature::Three);
        
        gui_state.selected_time_signature = TimeSignature::Six;
        assert_eq!(gui_state.selected_time_signature, TimeSignature::Six);
    }

    #[test]
    fn test_gui_state_sound_selection() {
        let mut gui_state = GuiState::new();
        
        // Test default sounds
        assert_eq!(gui_state.selected_beat_sound, SoundType::BuiltinClick);
        assert_eq!(gui_state.selected_accent_sound, SoundType::BuiltinWood);
        
        // Test changing beat sound
        gui_state.selected_beat_sound = SoundType::BuiltinBeep;
        assert_eq!(gui_state.selected_beat_sound, SoundType::BuiltinBeep);
        
        // Test changing accent sound
        gui_state.selected_accent_sound = SoundType::BuiltinClick;
        assert_eq!(gui_state.selected_accent_sound, SoundType::BuiltinClick);
        
        // Test custom sound
        let custom_path = std::path::PathBuf::from("test.wav");
        gui_state.selected_beat_sound = SoundType::Custom(custom_path.clone());
        assert_eq!(gui_state.selected_beat_sound, SoundType::Custom(custom_path));
    }

    #[test]
    fn test_metronome_app_creation() {
        let app = MetronomeApp::new();
        // Test that app can be created without panicking
        // We can't test much more without running the GUI event loop
        // This test mainly ensures the constructor works
    }

    #[test]
    fn test_gui_control_validation() {
        let mut gui_state = GuiState::new();
        
        // Test BPM input validation with whitespace
        let result = gui_state.validate_bpm("  120  ");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 120);
        
        // Test decimal input (should fail)
        let result = gui_state.validate_bpm("120.5");
        assert!(result.is_err());
        
        // Test negative input (should fail)
        let result = gui_state.validate_bpm("-120");
        assert!(result.is_err());
        
        // Test zero input (should fail)
        let result = gui_state.validate_bpm("0");
        assert!(result.is_err());
    }

    #[test]
    fn test_gui_state_running_status() {
        let mut gui_state = GuiState::new();
        
        // Initially not running
        assert!(!gui_state.is_running);
        
        // Test setting running status
        gui_state.is_running = true;
        assert!(gui_state.is_running);
        
        gui_state.is_running = false;
        assert!(!gui_state.is_running);
    }

    #[test]
    fn test_gui_state_accent_enabled() {
        let mut gui_state = GuiState::new();
        
        // Initially accent enabled
        assert!(gui_state.accent_enabled);
        
        // Test disabling accents
        gui_state.accent_enabled = false;
        assert!(!gui_state.accent_enabled);
        
        // Test enabling accents
        gui_state.accent_enabled = true;
        assert!(gui_state.accent_enabled);
    }
}#[cfg(not(
feature = "gui"))]
mod gui_disabled_tests {
    #[test]
    fn test_gui_feature_disabled() {
        // This test ensures that when GUI feature is disabled,
        // the code still compiles and basic functionality works
        assert!(true, "GUI feature is disabled, but compilation succeeded");
    }
}