use cli_metronome::app::{AppMode, determine_mode, launch_app};
use cli_metronome::cli::CliArgs;
use cli_metronome::metronome::MetronomeController;
use cli_metronome::audio::CrossPlatformAudio;
use cli_metronome::display::DisplayEngine;
use cli_metronome::models::{TimeSignature, SoundType, MetronomeConfig};

#[test]
fn test_app_mode_creation() {
    // Test CLI mode creation
    let cli_args = CliArgs::new(120);
    let cli_mode = AppMode::Cli(cli_args);
    
    match cli_mode {
        AppMode::Cli(args) => assert_eq!(args.bpm, 120),
        _ => panic!("Expected CLI mode"),
    }
    
    // Test GUI mode creation
    let gui_mode = AppMode::Gui;
    match gui_mode {
        AppMode::Gui => (),
        _ => panic!("Expected GUI mode"),
    }
}

#[test]
fn test_cli_mode_components_integration() {
    // Test that CLI mode components work together
    let controller = MetronomeController::new(120).unwrap();
    let display = DisplayEngine::new();
    let mut audio = CrossPlatformAudio::new();
    
    // Test audio initialization
    assert!(audio.initialize().is_ok());
    
    // Test metronome controller functionality
    assert_eq!(controller.get_metronome().get_bpm(), 120);
    assert!(!controller.is_running());
    
    // Test that controller can start and stop
    assert!(controller.start_safe().is_ok());
    assert!(controller.is_running());
    
    controller.stop_safe();
    assert!(!controller.is_running());
}

#[test]
fn test_gui_mode_components_integration() {
    #[cfg(feature = "gui")]
    {
        use cli_metronome::gui::MetronomeApp;
        use cli_metronome::models::GuiState;
        
        // Test GUI components integration
        let _app = MetronomeApp::new();
        let mut gui_state = GuiState::new();
        
        // Test GUI state validation
        assert!(gui_state.validate_bpm("120").is_ok());
        assert_eq!(gui_state.selected_time_signature, TimeSignature::Four);
        assert_eq!(gui_state.selected_beat_sound, SoundType::BuiltinClick);
    }
    
    #[cfg(not(feature = "gui"))]
    {
        // When GUI feature is disabled, this test should still pass
        assert!(true, "GUI feature disabled, test passes");
    }
}

#[test]
fn test_dual_mode_consistency_bpm_validation() {
    // Test that BPM validation is consistent between CLI and GUI modes
    
    // CLI mode validation
    let cli_result_valid = cli_metronome::cli::validate_bpm(120);
    assert!(cli_result_valid.is_ok());
    
    let cli_result_invalid = cli_metronome::cli::validate_bpm(59);
    assert!(cli_result_invalid.is_err());
    
    // GUI mode validation
    #[cfg(feature = "gui")]
    {
        let mut gui_state = cli_metronome::models::GuiState::new();
        
        let gui_result_valid = gui_state.validate_bpm("120");
        assert!(gui_result_valid.is_ok());
        
        let gui_result_invalid = gui_state.validate_bpm("59");
        assert!(gui_result_invalid.is_err());
    }
    
    // Metronome config validation (used by both modes)
    let config_valid = MetronomeConfig::new(120);
    assert!(config_valid.validate().is_ok());
    
    let config_invalid = MetronomeConfig::new(59);
    assert!(config_invalid.validate().is_err());
}

#[test]
fn test_dual_mode_consistency_time_signatures() {
    // Test that time signature handling is consistent between modes
    
    let time_signatures = [
        TimeSignature::Two,
        TimeSignature::Three,
        TimeSignature::Four,
        TimeSignature::Six,
    ];
    
    for time_sig in &time_signatures {
        // Test time signature properties are consistent
        let beats_per_measure = time_sig.beats_per_measure();
        let display_str = time_sig.as_str();
        
        assert!(beats_per_measure > 0);
        assert!(!display_str.is_empty());
        
        // Test that metronome can use this time signature
        let controller = MetronomeController::new(120).unwrap();
        controller.get_metronome().set_time_signature(*time_sig);
        assert_eq!(controller.get_metronome().get_time_signature(), *time_sig);
        
        // Test GUI state can handle this time signature
        #[cfg(feature = "gui")]
        {
            let mut gui_state = cli_metronome::models::GuiState::new();
            gui_state.selected_time_signature = *time_sig;
            assert_eq!(gui_state.selected_time_signature, *time_sig);
        }
    }
}

#[test]
fn test_dual_mode_consistency_sound_types() {
    // Test that sound type handling is consistent between modes
    
    let sound_types = [
        SoundType::BuiltinClick,
        SoundType::BuiltinWood,
        SoundType::BuiltinBeep,
    ];
    
    for sound_type in &sound_types {
        // Test sound type properties
        let display_str = sound_type.as_str();
        assert!(!display_str.is_empty());
        assert!(sound_type.is_builtin());
        
        // Test that metronome can use this sound type
        let controller = MetronomeController::new(120).unwrap();
        controller.get_metronome().set_sounds(sound_type.clone(), sound_type.clone());
        
        // Test that audio engine can handle this sound type
        let mut audio = CrossPlatformAudio::new();
        assert!(audio.load_builtin_sounds().is_ok());
        assert!(audio.is_sound_cached(sound_type));
        
        // Test GUI state can handle this sound type
        #[cfg(feature = "gui")]
        {
            let mut gui_state = cli_metronome::models::GuiState::new();
            gui_state.selected_beat_sound = sound_type.clone();
            gui_state.selected_accent_sound = sound_type.clone();
            assert_eq!(gui_state.selected_beat_sound, *sound_type);
            assert_eq!(gui_state.selected_accent_sound, *sound_type);
        }
    }
}

#[test]
fn test_dual_mode_metronome_core_consistency() {
    // Test that the core metronome functionality works the same in both modes
    
    let bpm = 120;
    let time_sig = TimeSignature::Four;
    
    // Create metronome for CLI mode
    let cli_controller = MetronomeController::new(bpm).unwrap();
    cli_controller.get_metronome().set_time_signature(time_sig);
    
    // Test CLI mode metronome properties
    assert_eq!(cli_controller.get_metronome().get_bpm(), bpm);
    assert_eq!(cli_controller.get_metronome().get_time_signature(), time_sig);
    assert!(!cli_controller.is_running());
    
    // Start and test CLI mode metronome
    assert!(cli_controller.start_safe().is_ok());
    assert!(cli_controller.is_running());
    
    // Create metronome for GUI mode (same underlying implementation)
    #[cfg(feature = "gui")]
    {
        use cli_metronome::metronome::Metronome;
        
        let gui_metronome = Metronome::with_bpm(bpm).unwrap();
        gui_metronome.set_time_signature(time_sig);
        
        // Test GUI mode metronome properties
        assert_eq!(gui_metronome.get_bpm(), bpm);
        assert_eq!(gui_metronome.get_time_signature(), time_sig);
        assert!(!gui_metronome.is_running());
        
        // Start and test GUI mode metronome
        assert!(gui_metronome.start().is_ok());
        assert!(gui_metronome.is_running());
        
        // Test that both metronomes have same interval calculation
        assert_eq!(cli_controller.get_metronome().get_interval(), gui_metronome.get_interval());
        
        gui_metronome.stop();
    }
    
    cli_controller.stop_safe();
}

#[test]
fn test_dual_mode_audio_consistency() {
    // Test that audio functionality is consistent between modes
    
    let mut audio = CrossPlatformAudio::new();
    assert!(audio.initialize().is_ok());
    
    // Test basic audio operations that both modes use
    assert!(audio.play_beat_sound().is_ok());
    
    let beat_sound = SoundType::BuiltinClick;
    let accent_sound = SoundType::BuiltinWood;
    
    assert!(audio.play_beat(&beat_sound).is_ok());
    assert!(audio.play_accent(&accent_sound).is_ok());
    assert!(audio.play_beat_with_accent(false, &beat_sound, &accent_sound).is_ok());
    assert!(audio.play_beat_with_accent(true, &beat_sound, &accent_sound).is_ok());
    
    // Test audio status reporting
    let status = audio.get_audio_status();
    let status_str = status.to_string();
    assert!(!status_str.is_empty());
}#[test]

fn test_dual_mode_error_handling_consistency() {
    // Test that error handling is consistent between modes
    
    use cli_metronome::error::MetronomeError;
    
    // Test invalid BPM error in both modes
    let cli_error = cli_metronome::cli::validate_bpm(59);
    assert!(cli_error.is_err());
    
    let metronome_error = cli_metronome::metronome::MetronomeController::new(59);
    assert!(metronome_error.is_err());
    
    #[cfg(feature = "gui")]
    {
        let mut gui_state = cli_metronome::models::GuiState::new();
        let gui_error = gui_state.validate_bpm("59");
        assert!(gui_error.is_err());
    }
    
    // Test that all errors are of the same type family
    match metronome_error {
        Err(MetronomeError::InvalidBpm(bpm)) => assert_eq!(bpm, 59),
        _ => panic!("Expected InvalidBpm error"),
    }
}

#[test]
fn test_dual_mode_beat_generation_consistency() {
    // Test that beat generation is consistent between modes
    
    use cli_metronome::models::Beat;
    
    let time_sig = TimeSignature::Four;
    let bpm = 120;
    
    // Generate beats and test consistency
    for beat_num in 1..=8 {
        let beat = Beat::new(beat_num, time_sig, bpm);
        
        assert_eq!(beat.sequence_number, beat_num);
        assert_eq!(beat.time_signature, time_sig);
        assert_eq!(beat.bpm, bpm);
        
        // Test beat position in measure
        let expected_beat_in_measure = ((beat_num - 1) % 4) + 1;
        assert_eq!(beat.beat_in_measure, expected_beat_in_measure as u32);
        
        // Test accent pattern for 4/4 time
        let expected_accent = beat.beat_in_measure == 1 || beat.beat_in_measure == 3;
        assert_eq!(beat.is_accent, expected_accent);
        
        // Test beat strength classification
        if beat.beat_in_measure == 1 {
            assert!(beat.is_strong_beat());
            assert!(!beat.is_medium_beat());
            assert!(!beat.is_weak_beat());
        } else if beat.beat_in_measure == 3 {
            assert!(!beat.is_strong_beat());
            assert!(beat.is_medium_beat());
            assert!(!beat.is_weak_beat());
        } else {
            assert!(!beat.is_strong_beat());
            assert!(!beat.is_medium_beat());
            assert!(beat.is_weak_beat());
        }
    }
}

#[test]
fn test_dual_mode_configuration_consistency() {
    // Test that configuration handling is consistent between modes
    
    let config = MetronomeConfig::new(120)
        .with_time_signature(TimeSignature::Three)
        .with_sounds(SoundType::BuiltinWood, SoundType::BuiltinBeep)
        .with_sound(true)
        .with_visual(true);
    
    assert!(config.validate().is_ok());
    assert_eq!(config.bpm, 120);
    assert_eq!(config.time_signature, TimeSignature::Three);
    assert_eq!(config.beat_sound, SoundType::BuiltinWood);
    assert_eq!(config.accent_sound, SoundType::BuiltinBeep);
    assert!(config.sound_enabled);
    assert!(config.visual_enabled);
    
    // Test that both CLI and GUI modes can use this configuration
    let cli_controller = MetronomeController::new(config.bpm).unwrap();
    assert_eq!(cli_controller.get_metronome().get_bpm(), config.bpm);
    
    #[cfg(feature = "gui")]
    {
        use cli_metronome::metronome::Metronome;
        let gui_metronome = Metronome::from_config(config.clone()).unwrap();
        assert_eq!(gui_metronome.get_bpm(), config.bpm);
        assert_eq!(gui_metronome.get_time_signature(), config.time_signature);
    }
}

#[test]
fn test_dual_mode_display_consistency() {
    // Test that display functionality works for CLI mode
    // (GUI mode display is handled by egui and harder to test)
    
    let display = DisplayEngine::new();
    let time_sig = TimeSignature::Four;
    let audio_status = cli_metronome::audio::AudioStatus::Available;
    
    // Test that display methods don't panic
    display.show_startup_info(120, time_sig, &audio_status);
    display.show_status(120, 1, std::time::Duration::from_secs(1), time_sig, 1);
    
    let beat = cli_metronome::models::Beat::new(1, time_sig, 120);
    display.show_visual_beat(&beat);
    display.show_beat_indicator(&beat);
    
    display.clear_line();
    display.show_goodbye();
}

#[test]
fn test_mode_switching_simulation() {
    // Simulate switching between modes by testing that the same
    // underlying functionality works in both contexts
    
    let bpm = 140;
    let time_sig = TimeSignature::Three;
    
    // Simulate CLI mode operation
    {
        let controller = MetronomeController::new(bpm).unwrap();
        controller.get_metronome().set_time_signature(time_sig);
        
        assert!(controller.start_safe().is_ok());
        assert!(controller.is_running());
        
        // Simulate some beats
        let beat1 = controller.get_metronome().increment_beat();
        let beat2 = controller.get_metronome().increment_beat();
        let beat3 = controller.get_metronome().increment_beat();
        let beat4 = controller.get_metronome().increment_beat(); // Back to beat 1 of next measure
        
        assert_eq!(beat1.beat_in_measure, 1);
        assert_eq!(beat2.beat_in_measure, 2);
        assert_eq!(beat3.beat_in_measure, 3);
        assert_eq!(beat4.beat_in_measure, 1); // New measure
        
        controller.stop_safe();
        assert!(!controller.is_running());
    }
    
    // Simulate GUI mode operation with same settings
    #[cfg(feature = "gui")]
    {
        use cli_metronome::metronome::Metronome;
        
        let metronome = Metronome::with_bpm(bpm).unwrap();
        metronome.set_time_signature(time_sig);
        
        assert!(metronome.start().is_ok());
        assert!(metronome.is_running());
        
        // Simulate same beat sequence
        let beat1 = metronome.increment_beat();
        let beat2 = metronome.increment_beat();
        let beat3 = metronome.increment_beat();
        let beat4 = metronome.increment_beat();
        
        assert_eq!(beat1.beat_in_measure, 1);
        assert_eq!(beat2.beat_in_measure, 2);
        assert_eq!(beat3.beat_in_measure, 3);
        assert_eq!(beat4.beat_in_measure, 1);
        
        metronome.stop();
        assert!(!metronome.is_running());
    }
}