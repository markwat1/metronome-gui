use cli_metronome::audio::{CrossPlatformAudio, AudioEngine, SoundData, AudioStatus};
use cli_metronome::models::{SoundType, Beat, TimeSignature};
use std::path::PathBuf;

#[test]
fn test_multiple_sound_data_creation() {
    // Test built-in sound creation
    let click_data = SoundData::from_builtin(SoundType::BuiltinClick);
    assert!(click_data.is_ok());
    let click = click_data.unwrap();
    assert_eq!(click.sound_type, SoundType::BuiltinClick);
    assert!(click.data.len() > 0);
    assert_eq!(click.channels, 1);
    assert_eq!(click.sample_rate, 44100);
    
    let wood_data = SoundData::from_builtin(SoundType::BuiltinWood);
    assert!(wood_data.is_ok());
    let wood = wood_data.unwrap();
    assert_eq!(wood.sound_type, SoundType::BuiltinWood);
    assert!(wood.data.len() > 0);
    
    let beep_data = SoundData::from_builtin(SoundType::BuiltinBeep);
    assert!(beep_data.is_ok());
    let beep = beep_data.unwrap();
    assert_eq!(beep.sound_type, SoundType::BuiltinBeep);
    assert!(beep.data.len() > 0);
}

#[test]
fn test_sound_data_different_characteristics() {
    let click = SoundData::from_builtin(SoundType::BuiltinClick).unwrap();
    let wood = SoundData::from_builtin(SoundType::BuiltinWood).unwrap();
    let beep = SoundData::from_builtin(SoundType::BuiltinBeep).unwrap();
    
    // Different sounds should have different data
    assert_ne!(click.data, wood.data);
    assert_ne!(click.data, beep.data);
    assert_ne!(wood.data, beep.data);
    
    // All should have same basic audio properties for consistency
    assert_eq!(click.sample_rate, wood.sample_rate);
    assert_eq!(wood.sample_rate, beep.sample_rate);
    assert_eq!(click.channels, wood.channels);
    assert_eq!(wood.channels, beep.channels);
}

#[test]
fn test_custom_sound_type_creation_fails_for_builtin() {
    let custom_path = PathBuf::from("test.wav");
    let result = SoundData::from_builtin(SoundType::Custom(custom_path));
    assert!(result.is_err());
}

#[test]
fn test_audio_cache_system() {
    let mut audio = CrossPlatformAudio::new();
    
    // Initially no sounds cached
    assert!(!audio.is_sound_cached(&SoundType::BuiltinClick));
    assert!(!audio.is_sound_cached(&SoundType::BuiltinWood));
    assert!(!audio.is_sound_cached(&SoundType::BuiltinBeep));
    
    // Load built-in sounds
    assert!(audio.load_builtin_sounds().is_ok());
    
    // Now sounds should be cached
    assert!(audio.is_sound_cached(&SoundType::BuiltinClick));
    assert!(audio.is_sound_cached(&SoundType::BuiltinWood));
    assert!(audio.is_sound_cached(&SoundType::BuiltinBeep));
    
    // Get cached sounds list
    let cached_sounds = audio.get_cached_sounds();
    assert_eq!(cached_sounds.len(), 3);
    assert!(cached_sounds.contains(&SoundType::BuiltinClick));
    assert!(cached_sounds.contains(&SoundType::BuiltinWood));
    assert!(cached_sounds.contains(&SoundType::BuiltinBeep));
}#[test]

fn test_audio_cache_retrieval() {
    let mut audio = CrossPlatformAudio::new();
    audio.load_builtin_sounds().unwrap();
    
    // Test getting sound data from cache
    let click_data = audio.get_sound_data(&SoundType::BuiltinClick);
    assert!(click_data.is_some());
    let click = click_data.unwrap();
    assert_eq!(click.sound_type, SoundType::BuiltinClick);
    
    // Test getting non-cached sound
    let custom_path = PathBuf::from("non_existent.wav");
    let custom_sound = SoundType::Custom(custom_path);
    let custom_data = audio.get_sound_data(&custom_sound);
    assert!(custom_data.is_none());
}

#[test]
fn test_audio_cache_clearing() {
    let mut audio = CrossPlatformAudio::new();
    audio.load_builtin_sounds().unwrap();
    
    // Verify sounds are cached
    assert!(audio.is_sound_cached(&SoundType::BuiltinClick));
    
    // Clear cache
    audio.clear_cache();
    
    // Verify cache is empty
    assert!(!audio.is_sound_cached(&SoundType::BuiltinClick));
    assert_eq!(audio.get_cached_sounds().len(), 0);
}

#[test]
fn test_custom_sound_loading_with_fallback() {
    let mut audio = CrossPlatformAudio::new();
    
    // Try to load non-existent custom sound - should fallback to built-in
    let non_existent_path = PathBuf::from("non_existent.wav");
    let result = audio.load_custom_sound(&non_existent_path);
    
    // Should succeed with fallback
    assert!(result.is_ok());
    let sound_type = result.unwrap();
    assert_eq!(sound_type, SoundType::BuiltinClick); // Should fallback to click
    assert!(audio.is_sound_cached(&sound_type));
}

#[test]
fn test_custom_sound_loading_strict() {
    let mut audio = CrossPlatformAudio::new();
    
    // Try to load non-existent custom sound strictly - should fail
    let non_existent_path = PathBuf::from("non_existent.wav");
    let result = audio.load_custom_sound_strict(&non_existent_path);
    
    // Should fail without fallback
    assert!(result.is_err());
}

#[test]
fn test_custom_sound_file_validation() {
    // Test non-existent file
    let non_existent = PathBuf::from("non_existent.wav");
    assert!(CrossPlatformAudio::validate_sound_file(&non_existent).is_err());
    
    // Test supported extensions
    let supported = CrossPlatformAudio::get_supported_extensions();
    assert!(supported.contains(&"wav"));
    assert!(supported.contains(&"mp3"));
    assert!(supported.contains(&"ogg"));
    assert_eq!(supported.len(), 3);
}

#[test]
fn test_preload_sounds_with_fallback() {
    let mut audio = CrossPlatformAudio::new();
    
    let sounds_to_preload = vec![
        SoundType::BuiltinClick,
        SoundType::BuiltinWood,
        SoundType::Custom(PathBuf::from("non_existent.wav")), // Should fallback
    ];
    
    // Should succeed even with invalid custom sound (fallback)
    assert!(audio.preload_sounds(&sounds_to_preload).is_ok());
    
    // Built-in sounds should be cached
    assert!(audio.is_sound_cached(&SoundType::BuiltinClick));
    assert!(audio.is_sound_cached(&SoundType::BuiltinWood));
    
    // The custom sound should have fallen back to built-in click
    // (The fallback sound should be cached)
    let cached_sounds = audio.get_cached_sounds();
    assert!(cached_sounds.len() >= 2); // At least the two built-in sounds
}

#[test]
fn test_strong_weak_beat_playback() {
    let mut audio = CrossPlatformAudio::new();
    audio.initialize().unwrap(); // Should succeed in fallback mode
    
    let beat_sound = SoundType::BuiltinClick;
    let accent_sound = SoundType::BuiltinWood;
    
    // Test playing regular beat
    assert!(audio.play_beat_with_accent(false, &beat_sound, &accent_sound).is_ok());
    
    // Test playing accent beat
    assert!(audio.play_beat_with_accent(true, &beat_sound, &accent_sound).is_ok());
    
    // Test individual beat and accent methods
    assert!(audio.play_beat(&beat_sound).is_ok());
    assert!(audio.play_accent(&accent_sound).is_ok());
}

#[test]
fn test_beat_playback_from_beat_info() {
    let audio = CrossPlatformAudio::new();
    
    // Create beats with different accent patterns
    let strong_beat = Beat::new(1, TimeSignature::Four, 120); // First beat - strong
    let weak_beat = Beat::new(2, TimeSignature::Four, 120);   // Second beat - weak
    let medium_beat = Beat::new(3, TimeSignature::Four, 120); // Third beat - medium
    
    // Test that beats have correct accent properties
    assert!(strong_beat.is_accent);
    assert!(!weak_beat.is_accent);
    assert!(medium_beat.is_accent); // In 4/4, beat 3 is accented
    
    // Test playing beats based on their accent properties
    let beat_sound = SoundType::BuiltinClick;
    let accent_sound = SoundType::BuiltinWood;
    
    assert!(audio.play_beat_with_accent(strong_beat.is_accent, &beat_sound, &accent_sound).is_ok());
    assert!(audio.play_beat_with_accent(weak_beat.is_accent, &beat_sound, &accent_sound).is_ok());
    assert!(audio.play_beat_with_accent(medium_beat.is_accent, &beat_sound, &accent_sound).is_ok());
}

#[test]
fn test_audio_status_reporting() {
    let audio = CrossPlatformAudio::new();
    
    // Test initial status
    let status = audio.get_audio_status();
    assert!(matches!(status, AudioStatus::FallbackMode | AudioStatus::Disabled));
    
    // Test status display
    let status_str = status.to_string();
    assert!(!status_str.is_empty());
    
    // Test audio availability
    assert!(!audio.is_audio_available()); // Should be false without initialization
}

#[test]
fn test_audio_initialization() {
    let mut audio = CrossPlatformAudio::new();
    
    // Test initialization (should succeed in fallback mode)
    assert!(audio.initialize().is_ok());
    
    // Test that basic playback works after initialization
    assert!(audio.play_beat_sound().is_ok());
    
    // Test status after initialization
    let status = audio.get_audio_status();
    assert!(matches!(status, AudioStatus::FallbackMode | AudioStatus::Available));
}

#[test]
fn test_audio_fallback_behavior() {
    let audio = CrossPlatformAudio::new().with_fallback(true);
    
    // Should be in fallback mode
    assert_eq!(audio.get_audio_status(), AudioStatus::FallbackMode);
    
    let audio_no_fallback = CrossPlatformAudio::new().with_fallback(false);
    // Without fallback, should be disabled initially
    assert_eq!(audio_no_fallback.get_audio_status(), AudioStatus::Disabled);
}

#[test]
fn test_audio_engine_high_level_interface() {
    let engine = AudioEngine::default();
    
    // Test default sound settings
    assert_eq!(engine.get_beat_sound(), &SoundType::BuiltinClick);
    assert_eq!(engine.get_accent_sound(), &SoundType::BuiltinWood);
    
    // Test basic playback
    assert!(engine.play_beat().is_ok());
    assert!(engine.play_accent().is_ok());
    assert!(engine.play_beat_with_accent(false).is_ok());
    assert!(engine.play_beat_with_accent(true).is_ok());
}#
[test]
fn test_audio_engine_sound_configuration() {
    let mut engine = AudioEngine::default();
    
    // Test setting beat sound
    let new_beat_sound = SoundType::BuiltinBeep;
    assert!(engine.set_beat_sound(new_beat_sound.clone()).is_ok());
    assert_eq!(engine.get_beat_sound(), &new_beat_sound);
    
    // Test setting accent sound
    let new_accent_sound = SoundType::BuiltinClick;
    assert!(engine.set_accent_sound(new_accent_sound.clone()).is_ok());
    assert_eq!(engine.get_accent_sound(), &new_accent_sound);
    
    // Test setting both sounds
    let beat = SoundType::BuiltinWood;
    let accent = SoundType::BuiltinBeep;
    assert!(engine.set_sounds(beat.clone(), accent.clone()).is_ok());
    assert_eq!(engine.get_beat_sound(), &beat);
    assert_eq!(engine.get_accent_sound(), &accent);
}

#[test]
fn test_audio_engine_beat_info_playback() {
    let engine = AudioEngine::default();
    
    // Test playing from beat info
    let regular_beat = Beat::new(2, TimeSignature::Four, 120); // Second beat in 4/4 (weak)
    let accent_beat = Beat::new(1, TimeSignature::Four, 120);  // First beat in 4/4 (strong)
    
    assert!(engine.play_beat_from_info(&regular_beat).is_ok());
    assert!(engine.play_beat_from_info(&accent_beat).is_ok());
}

#[test]
fn test_audio_engine_status_and_caching() {
    let engine = AudioEngine::default();
    
    // Test status reporting
    let status = engine.get_status();
    assert!(matches!(status, AudioStatus::FallbackMode | AudioStatus::Available));
    
    // Test cached sounds retrieval
    let cached_sounds = engine.get_cached_sounds();
    assert!(cached_sounds.len() >= 2); // Should have at least beat and accent sounds cached
    
    // Test file validation
    let non_existent = PathBuf::from("non_existent.wav");
    assert!(AudioEngine::validate_sound_file(&non_existent).is_err());
    
    // Test supported extensions
    let extensions = AudioEngine::get_supported_extensions();
    assert!(extensions.contains(&"wav"));
    assert!(extensions.contains(&"mp3"));
    assert!(extensions.contains(&"ogg"));
}

#[test]
fn test_time_signature_beat_patterns() {
    // Test different time signatures produce correct beat patterns
    
    // 2/4 time
    let beat1_2_4 = Beat::new(1, TimeSignature::Two, 120);
    let beat2_2_4 = Beat::new(2, TimeSignature::Two, 120);
    assert!(beat1_2_4.is_accent);  // Strong
    assert!(!beat2_2_4.is_accent); // Weak
    
    // 3/4 time
    let beat1_3_4 = Beat::new(1, TimeSignature::Three, 120);
    let beat2_3_4 = Beat::new(2, TimeSignature::Three, 120);
    let beat3_3_4 = Beat::new(3, TimeSignature::Three, 120);
    assert!(beat1_3_4.is_accent);  // Strong
    assert!(!beat2_3_4.is_accent); // Weak
    assert!(!beat3_3_4.is_accent); // Weak
    
    // 4/4 time
    let beat1_4_4 = Beat::new(1, TimeSignature::Four, 120);
    let beat2_4_4 = Beat::new(2, TimeSignature::Four, 120);
    let beat3_4_4 = Beat::new(3, TimeSignature::Four, 120);
    let beat4_4_4 = Beat::new(4, TimeSignature::Four, 120);
    assert!(beat1_4_4.is_accent);  // Strong
    assert!(!beat2_4_4.is_accent); // Weak
    assert!(beat3_4_4.is_accent);  // Medium (treated as accent)
    assert!(!beat4_4_4.is_accent); // Weak
    
    // 6/8 time
    let beat1_6_8 = Beat::new(1, TimeSignature::Six, 120);
    let beat4_6_8 = Beat::new(4, TimeSignature::Six, 120);
    assert!(beat1_6_8.is_accent);  // Strong
    assert!(beat4_6_8.is_accent);  // Medium (treated as accent)
}

#[test]
fn test_audio_playback_with_different_time_signatures() {
    let audio = CrossPlatformAudio::new();
    let beat_sound = SoundType::BuiltinClick;
    let accent_sound = SoundType::BuiltinWood;
    
    // Test playback for different time signature patterns
    let time_signatures = [TimeSignature::Two, TimeSignature::Three, TimeSignature::Four, TimeSignature::Six];
    
    for time_sig in &time_signatures {
        let beats_per_measure = time_sig.beats_per_measure();
        
        for beat_num in 1..=beats_per_measure {
            let beat = Beat::new(beat_num as u64, *time_sig, 120);
            let result = audio.play_beat_with_accent(beat.is_accent, &beat_sound, &accent_sound);
            assert!(result.is_ok(), "Failed to play beat {} in {:?} time", beat_num, time_sig);
        }
    }
}