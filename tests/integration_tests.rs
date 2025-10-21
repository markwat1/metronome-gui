use cli_metronome::{
    cli::validate_bpm,
    metronome::MetronomeController,
    audio::CrossPlatformAudio,
    display::DisplayEngine,
    models::MetronomeConfig,
};
use std::time::Duration;

#[test]
fn test_end_to_end_metronome_creation() {
    // Test creating a complete metronome system
    let mut controller = MetronomeController::new(120).unwrap();
    let mut audio = CrossPlatformAudio::new();
    let _display = DisplayEngine::new();
    
    // Initialize audio (should work in fallback mode)
    assert!(audio.initialize().is_ok());
    
    // Start metronome
    assert!(controller.start().is_ok());
    assert!(controller.is_running());
    
    // Stop metronome
    controller.stop();
    assert!(!controller.is_running());
}

#[test]
fn test_bpm_validation_integration() {
    // Test BPM validation across different components
    assert!(validate_bpm(60).is_ok());
    assert!(validate_bpm(120).is_ok());
    assert!(validate_bpm(200).is_ok());
    
    assert!(validate_bpm(59).is_err());
    assert!(validate_bpm(201).is_err());
    
    // Test with MetronomeConfig
    let config = MetronomeConfig::new(120);
    assert!(config.validate().is_ok());
    
    let config = MetronomeConfig::new(59);
    assert!(config.validate().is_err());
}

#[test]
fn test_audio_fallback_integration() {
    // Test that the system works without audio
    let mut audio = CrossPlatformAudio::new();
    
    // Should initialize successfully in fallback mode
    assert!(audio.initialize().is_ok());
    
    // Should not fail when trying to play sound
    assert!(audio.play_beat_sound().is_ok());
    
    // Audio status should indicate either available or fallback mode
    let status = audio.get_audio_status();
    // With audio feature enabled, it should be available
    #[cfg(feature = "audio")]
    assert_eq!(status.to_string(), "Audio available");
    
    // Without audio feature, it should be fallback mode
    #[cfg(not(feature = "audio"))]
    assert_eq!(status.to_string(), "Visual-only mode");
}

#[test]
fn test_metronome_timing_accuracy() {
    // Test that metronome timing calculations are accurate
    let metronome = cli_metronome::metronome::Metronome::with_bpm(60).unwrap();
    assert_eq!(metronome.get_interval(), Duration::from_secs(1));
    
    let metronome = cli_metronome::metronome::Metronome::with_bpm(120).unwrap();
    assert_eq!(metronome.get_interval(), Duration::from_millis(500));
    
    let metronome = cli_metronome::metronome::Metronome::with_bpm(200).unwrap();
    assert_eq!(metronome.get_interval(), Duration::from_millis(300));
}

#[test]
fn test_cross_platform_compatibility() {
    // Test that basic functionality works on current platform
    let mut controller = MetronomeController::new(120).unwrap();
    let mut audio = CrossPlatformAudio::new();
    
    // These should work on any platform
    assert!(audio.initialize().is_ok());
    assert!(controller.start().is_ok());
    
    // Test audio system detection
    let _audio_available = audio.test_audio_system();
    // Note: We don't assert the result since it depends on the platform
    
    controller.stop();
}

#[test]
fn test_error_handling_integration() {
    // Test error propagation through the system
    use cli_metronome::MetronomeError;
    
    // Test invalid BPM error propagation
    let result = MetronomeController::new(300);
    assert!(result.is_err());
    
    if let Err(error) = result {
        match error {
            MetronomeError::InvalidBpm(bpm) => assert_eq!(bpm, 300),
            _ => panic!("Expected InvalidBpm error"),
        }
    } else {
        panic!("Expected error for invalid BPM");
    }
}

#[test]
fn test_display_functionality() {
    // Test display engine functionality
    let display = DisplayEngine::new();
    
    // These should not panic or fail
    display.show_startup_info(120, cli_metronome::models::TimeSignature::Four, &cli_metronome::audio::AudioStatus::FallbackMode);
    display.show_status(120, 1, Duration::from_secs(1), cli_metronome::models::TimeSignature::Four, 1);
    let beat = cli_metronome::models::Beat::new(1, cli_metronome::models::TimeSignature::Four, 120);
    display.show_visual_beat(&beat);
    display.clear_line();
}

#[cfg(target_os = "linux")]
#[test]
fn test_linux_specific_functionality() {
    // Test Linux-specific behavior
    let mut audio = CrossPlatformAudio::new();
    
    // Should work in fallback mode on Linux without ALSA
    assert!(audio.initialize().is_ok());
    
    // Should handle Ctrl+C setup (though we can't test the actual signal)
    let controller = MetronomeController::new(120).unwrap();
    // Note: We can't easily test the actual Ctrl+C handler in a unit test
    assert!(!controller.is_running());
}

#[cfg(target_os = "windows")]
#[test]
fn test_windows_specific_functionality() {
    // Test Windows-specific behavior
    let mut audio = CrossPlatformAudio::new();
    
    // Should work on Windows
    assert!(audio.initialize().is_ok());
    
    let controller = MetronomeController::new(120).unwrap();
    assert!(!controller.is_running());
}