use std::time::{Duration, Instant};
use std::path::PathBuf;
use crate::error::{MetronomeError, Result};

#[cfg(feature = "gui")]
use serde::{Deserialize, Serialize};

/// Time signature enumeration supporting common time signatures
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "gui", derive(Serialize, Deserialize))]
pub enum TimeSignature {
    // no time signature
    One,
    /// 2/4 time signature
    Two,
    /// 3/4 time signature  
    Three,
    /// 4/4 time signature
    Four,
    /// 5/4 time signature
    Five,
    /// 6/8 time signature
    Six,
    /// 7/8 time signature
    Seven,
    /// 8/8 time signature
    Eight,
}

impl TimeSignature {
    /// Get the number of beats per measure for this time signature
    pub fn beats_per_measure(&self) -> u32 {
        match self {
            TimeSignature::One => 1,
            TimeSignature::Two => 2,
            TimeSignature::Three => 3,
            TimeSignature::Four => 4,
            TimeSignature::Five => 5,
            TimeSignature::Six => 6,
            TimeSignature::Seven => 7,
            TimeSignature::Eight => 8,
        }
    }
    
    /// Get a human-readable string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeSignature::One => "None",
            TimeSignature::Two => "2/4",
            TimeSignature::Three => "3/4",
            TimeSignature::Four => "4/4",
            TimeSignature::Five => "5/8",
            TimeSignature::Six => "6/8",
            TimeSignature::Seven => "7/8",
            TimeSignature::Eight => "8/8",
        }
    }
    
    /// Get all available time signatures
    pub fn all() -> &'static [TimeSignature] {
        &[
            TimeSignature::One,
            TimeSignature::Two,
            TimeSignature::Three,
            TimeSignature::Four,
            TimeSignature::Five,
            TimeSignature::Six,
            TimeSignature::Seven,
            TimeSignature::Eight,
        ]
    }
}

impl Default for TimeSignature {
    fn default() -> Self {
        TimeSignature::Four
    }
}

/// Sound type enumeration for different metronome sounds
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "gui", derive(Serialize, Deserialize))]
pub enum SoundType {
    /// Built-in click sound
    BuiltinClick,
    /// Built-in wood block sound
    BuiltinWood,
    /// Built-in beep sound
    BuiltinBeep,
    /// Custom sound from file
    Custom(PathBuf),
}

impl SoundType {
    /// Get a human-readable string representation
    pub fn as_str(&self) -> String {
        match self {
            SoundType::BuiltinClick => "Click".to_string(),
            SoundType::BuiltinWood => "Wood".to_string(),
            SoundType::BuiltinBeep => "Beep".to_string(),
            SoundType::Custom(path) => {
                format!("Custom: {}", path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("Unknown"))
            }
        }
    }
    
    /// Get all built-in sound types
    pub fn builtin_sounds() -> &'static [SoundType] {
        &[
            SoundType::BuiltinClick,
            SoundType::BuiltinWood,
            SoundType::BuiltinBeep,
        ]
    }
    
    /// Check if this is a built-in sound
    pub fn is_builtin(&self) -> bool {
        matches!(self, SoundType::BuiltinClick | SoundType::BuiltinWood | SoundType::BuiltinBeep)
    }
}

impl Default for SoundType {
    fn default() -> Self {
        SoundType::BuiltinClick
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "gui", derive(Serialize, Deserialize))]
pub struct MetronomeConfig {
    pub bpm: u32,
    pub time_signature: TimeSignature,
    pub beat_sound: SoundType,
    pub accent_sound: SoundType,
    pub sound_enabled: bool,
    pub visual_enabled: bool,
    pub accent_enabled: bool,
    pub volume: f32,
}

impl MetronomeConfig {
    pub fn new(bpm: u32) -> Self {
        Self {
            bpm,
            time_signature: TimeSignature::default(),
            beat_sound: SoundType::default(),
            accent_sound: SoundType::BuiltinWood, // Different sound for accent
            sound_enabled: true,
            visual_enabled: true,
            accent_enabled: true, // Accents enabled by default
            volume: 0.7, // Default volume at 70%
        }
    }
    
    pub fn default() -> Self {
        Self::new(120)
    }
    
    pub fn validate(&self) -> Result<()> {
        if self.bpm < 60 || self.bpm > 200 {
            return Err(MetronomeError::InvalidBpm(self.bpm));
        }
        Ok(())
    }
    
    pub fn with_sound(mut self, enabled: bool) -> Self {
        self.sound_enabled = enabled;
        self
    }
    
    pub fn with_visual(mut self, enabled: bool) -> Self {
        self.visual_enabled = enabled;
        self
    }
    
    pub fn with_time_signature(mut self, time_signature: TimeSignature) -> Self {
        self.time_signature = time_signature;
        self
    }
    
    pub fn with_sounds(mut self, beat_sound: SoundType, accent_sound: SoundType) -> Self {
        self.beat_sound = beat_sound;
        self.accent_sound = accent_sound;
        self
    }
    
    pub fn with_accent_enabled(mut self, enabled: bool) -> Self {
        self.accent_enabled = enabled;
        self
    }
    
    pub fn with_volume(mut self, volume: f32) -> Self {
        self.volume = volume.clamp(0.0, 1.0);
        self
    }
    
    #[cfg(feature = "gui")]
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        use std::fs;
        use crate::error::ConfigError;
        
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| ConfigError::ParseError(format!("Failed to serialize config: {}", e)))?;
        fs::write(path, json)
            .map_err(|e| ConfigError::WriteError(format!("Failed to write config file: {}", e)))?;
        Ok(())
    }
    
    #[cfg(feature = "gui")]
    pub fn load_from_file(path: &std::path::Path) -> Result<Self> {
        use std::fs;
        use crate::error::ConfigError;
        
        let json = fs::read_to_string(path)
            .map_err(|e| ConfigError::ReadError(format!("Failed to read config file: {}", e)))?;
        let config = serde_json::from_str(&json)
            .map_err(|e| ConfigError::ParseError(format!("Failed to parse config: {}", e)))?;
        Ok(config)
    }
}

#[derive(Debug, Clone)]
pub struct Beat {
    pub timestamp: Instant,
    pub sequence_number: u64,
    pub beat_in_measure: u32,
    pub is_accent: bool,
    pub bpm: u32,
    pub time_signature: TimeSignature,
    pub accent_enabled: bool,
}

impl Beat {
    pub fn new(sequence_number: u64, time_signature: TimeSignature, bpm: u32) -> Self {
        Self::new_with_accent_setting(sequence_number, time_signature, bpm, true)
    }
    
    pub fn new_with_accent_setting(sequence_number: u64, time_signature: TimeSignature, bpm: u32, accent_enabled: bool) -> Self {
        let beats_per_measure = time_signature.beats_per_measure();
        // Handle the case where sequence_number is 0 by treating it as beat 1
        let effective_sequence = if sequence_number == 0 { 1 } else { sequence_number };
        let beat_in_measure = ((effective_sequence - 1) % beats_per_measure as u64) as u32 + 1;
        
        // Create a temporary state to calculate accent
        let temp_state = MetronomeState {
            bpm,
            time_signature,
            beat_sound: SoundType::default(),
            accent_sound: SoundType::default(),
            is_running: true,
            start_time: Some(Instant::now()),
            beat_count: sequence_number,
            current_beat_in_measure: beat_in_measure,
            accent_enabled,
            volume: 0.7,
        };
        
        let is_accent = temp_state.is_accent_beat();
        
        Self {
            timestamp: Instant::now(),
            sequence_number: effective_sequence,
            beat_in_measure,
            is_accent,
            bpm,
            time_signature,
            accent_enabled,
        }
    }
    
    pub fn is_first_beat(&self) -> bool {
        self.beat_in_measure == 1
    }
    
    pub fn get_accent_strength(&self) -> f32 {
        if !self.accent_enabled {
            return 0.0; // No accents when disabled
        }
        
        match self.time_signature {
            TimeSignature::One => {
                match self.beat_in_measure {
                    _ => 0.0, // Weak beat
                }
            }
            TimeSignature::Two => {
                match self.beat_in_measure {
                    1 => 1.0, // Strong beat
                    _ => 0.0, // Weak beat
                }
            }
            TimeSignature::Three => {
                match self.beat_in_measure {
                    1 => 1.0, // Strong beat
                    _ => 0.0, // Weak beats
                }
            }
            TimeSignature::Four => {
                match self.beat_in_measure {
                    1 => 1.0,   // Strong beat
                    3 => 0.5,   // Medium beat
                    _ => 0.0,   // Weak beats
                }
            }
            TimeSignature::Five => {
                match self.beat_in_measure {
                    1 => 1.0, // Strong beat
                    _ => 0.0, // Weak beats
                }
            }
            TimeSignature::Six => {
                match self.beat_in_measure {
                    1 => 1.0,   // Strong beat
                    4 => 0.5,   // Medium beat
                    _ => 0.0,   // Weak beats
                }
            }
            TimeSignature::Seven => {
                match self.beat_in_measure {
                    1 => 1.0, // Strong beat
                    _ => 0.0, // Weak beats
                }
            }
            TimeSignature::Eight=> {
                match self.beat_in_measure {
                    1 => 1.0,   // Strong beat
                    5 => 0.5,   // Medium beat
                    _ => 0.0,   // Weak beats
                }
            }
        }
    }
    
    pub fn is_strong_beat(&self) -> bool {
        self.get_accent_strength() >= 1.0
    }
    
    pub fn is_medium_beat(&self) -> bool {
        let strength = self.get_accent_strength();
        strength > 0.0 && strength < 1.0
    }
    
    pub fn is_weak_beat(&self) -> bool {
        self.get_accent_strength() == 0.0
    }
}

/// Metronome state structure for thread-safe access
#[derive(Debug, Clone)]
pub struct MetronomeState {
    pub bpm: u32,
    pub time_signature: TimeSignature,
    pub beat_sound: SoundType,
    pub accent_sound: SoundType,
    pub is_running: bool,
    pub start_time: Option<Instant>,
    pub beat_count: u64,
    pub current_beat_in_measure: u32,
    pub accent_enabled: bool,
    pub volume: f32,
}

impl MetronomeState {
    pub fn new(config: &MetronomeConfig) -> Self {
        Self {
            bpm: config.bpm,
            time_signature: config.time_signature,
            beat_sound: config.beat_sound.clone(),
            accent_sound: config.accent_sound.clone(),
            is_running: false,
            start_time: None,
            beat_count: 0,
            current_beat_in_measure: 1,
            accent_enabled: config.accent_enabled,
            volume: config.volume,
        }
    }
    
    pub fn get_elapsed_time(&self) -> Duration {
        match self.start_time {
            Some(start) => start.elapsed(),
            None => Duration::from_secs(0),
        }
    }
    
    pub fn get_interval(&self) -> Duration {
        self.calculate_beat_interval()
    }
    
    /// Calculate beat interval based on time signature and BPM
    pub fn calculate_beat_interval(&self) -> Duration {
        let base_seconds_per_beat = 60.0 / self.bpm as f64;
        
        // Adjust timing based on time signature
        match self.time_signature {
            TimeSignature::One => {
                // None
                Duration::from_secs_f64(base_seconds_per_beat)
            }
            TimeSignature::Two => {
                // 2/4 time - quarter note gets the beat
                Duration::from_secs_f64(base_seconds_per_beat)
            }
            TimeSignature::Three => {
                // 3/4 time - quarter note gets the beat
                Duration::from_secs_f64(base_seconds_per_beat)
            }
            TimeSignature::Four => {
                // 4/4 time - quarter note gets the beat
                Duration::from_secs_f64(base_seconds_per_beat)
            }
            TimeSignature::Five => {
                // 6/8 time - eighth note gets the beat, but we count in compound time
                // BPM refers to dotted quarter notes (3 eighth notes)
                // So each eighth note is 1/3 of the dotted quarter
                Duration::from_secs_f64(base_seconds_per_beat)
            }
            TimeSignature::Six => {
                // 6/8 time - eighth note gets the beat, but we count in compound time
                // BPM refers to dotted quarter notes (3 eighth notes)
                // So each eighth note is 1/3 of the dotted quarter
                Duration::from_secs_f64(base_seconds_per_beat)
            }
            TimeSignature::Seven => {
                // 6/8 time - eighth note gets the beat, but we count in compound time
                // BPM refers to dotted quarter notes (3 eighth notes)
                // So each eighth note is 1/3 of the dotted quarter
                Duration::from_secs_f64(base_seconds_per_beat)
            }
            TimeSignature::Eight => {
                // 6/8 time - eighth note gets the beat, but we count in compound time
                // BPM refers to dotted quarter notes (3 eighth notes)
                // So each eighth note is 1/3 of the dotted quarter
                Duration::from_secs_f64(base_seconds_per_beat)
            }
        }
    }
    
    /// Get the accent pattern for the current time signature
    pub fn get_accent_pattern(&self) -> Vec<bool> {
        match self.time_signature {
            TimeSignature::One => vec![false], // weak
            TimeSignature::Two => vec![true, false], // Strong-weak
            TimeSignature::Three => vec![true, false, false], // Strong-weak-weak
            TimeSignature::Four => vec![true, false, true, false], // Strong-weak-medium-weak
            TimeSignature::Five => vec![true, false, false,false,false], // Strong-weak-weak-weak-weak
            TimeSignature::Six => vec![true, false, false, true, false, false], // Strong-weak-weak-medium-weak-weak
            TimeSignature::Seven => vec![true, false, false, false, false, false], // Strong-weak-weak-weak-weak-weak-weak
            TimeSignature::Eight => vec![true, false, false, false, true, false, false, false], // Strong-weak-weak-weak-medium-weak-weak-weak
        }
    }
    
    /// Check if the current beat should be accented based on time signature
    pub fn is_accent_beat(&self) -> bool {
        if !self.accent_enabled {
            return false; // No accents when disabled
        }
        let pattern = self.get_accent_pattern();
        let beat_index = (self.current_beat_in_measure - 1) as usize;
        pattern.get(beat_index).copied().unwrap_or(false)
    }
    
    /// Get the accent strength (0.0 = no accent, 1.0 = strongest accent)
    pub fn get_accent_strength(&self) -> f32 {
        if !self.accent_enabled {
            return 0.0; // No accents when disabled
        }
        match self.time_signature {
            TimeSignature::One => {
                match self.current_beat_in_measure {
                    _ => 0.0, // Weak beat
                }
            }
            TimeSignature::Two => {
                match self.current_beat_in_measure {
                    1 => 1.0, // Strong beat
                    _ => 0.0, // Weak beat
                }
            }
            TimeSignature::Three => {
                match self.current_beat_in_measure {
                    1 => 1.0, // Strong beat
                    _ => 0.0, // Weak beats
                }
            }
            TimeSignature::Four => {
                match self.current_beat_in_measure {
                    1 => 1.0,   // Strong beat
                    3 => 0.5,   // Medium beat
                    _ => 0.0,   // Weak beats
                }
            }
            TimeSignature::Five => {
                match self.current_beat_in_measure {
                    1 => 1.0, // Strong beat
                    _ => 0.0, // Weak beats
                }
            }
            TimeSignature::Six => {
                match self.current_beat_in_measure {
                    1 => 1.0,   // Strong beat
                    4 => 0.5,   // Medium beat
                    _ => 0.0,   // Weak beats
                }
            }
            TimeSignature::Seven => {
                match self.current_beat_in_measure {
                    1 => 1.0, // Strong beat
                    _ => 0.0, // Weak beats
                }
            }
            TimeSignature::Eight => {
                match self.current_beat_in_measure {
                    1 => 1.0,   // Strong beat
                    5 => 0.5,   // Medium beat
                    _ => 0.0,   // Weak beats
                }
            }
        }
    }
    
    /// Calculate the expected time for the next beat
    pub fn get_next_beat_time(&self) -> Option<Instant> {
        if let Some(start_time) = self.start_time {
            let interval = self.calculate_beat_interval();
            let next_beat_time = start_time + interval * (self.beat_count + 1) as u32;
            Some(next_beat_time)
        } else {
            None
        }
    }
    
    /// Get timing accuracy (how close we are to the expected beat time)
    pub fn get_timing_accuracy(&self, actual_beat_time: Instant) -> Option<Duration> {
        if let Some(expected_time) = self.get_next_beat_time() {
            if actual_beat_time >= expected_time {
                Some(actual_beat_time - expected_time)
            } else {
                Some(expected_time - actual_beat_time)
            }
        } else {
            None
        }
    }
    
    pub fn increment_beat(&mut self) -> Beat {
        self.beat_count += 1;
        let beats_per_measure = self.time_signature.beats_per_measure();
        self.current_beat_in_measure = ((self.beat_count - 1) % beats_per_measure as u64) as u32 + 1;
        
        Beat::new(self.beat_count, self.time_signature, self.bpm)
    }
    
    pub fn start(&mut self) {
        if !self.is_running {
            self.start_time = Some(Instant::now());
            self.beat_count = 0;
            self.current_beat_in_measure = 1;
            self.is_running = true;
        }
    }
    
    pub fn stop(&mut self) {
        self.is_running = false;
        self.start_time = None;
    }
    
    pub fn update_bpm(&mut self, bpm: u32) -> Result<()> {
        if bpm < 60 || bpm > 200 {
            return Err(MetronomeError::InvalidBpm(bpm));
        }
        self.bpm = bpm;
        Ok(())
    }
    
    pub fn update_time_signature(&mut self, time_signature: TimeSignature) {
        self.time_signature = time_signature;
        // Reset beat position when changing time signature
        if self.is_running {
            self.current_beat_in_measure = 1;
        }
    }
    
    pub fn update_sounds(&mut self, beat_sound: SoundType, accent_sound: SoundType) {
        self.beat_sound = beat_sound;
        self.accent_sound = accent_sound;
    }
    
    pub fn update_accent_enabled(&mut self, accent_enabled: bool) {
        self.accent_enabled = accent_enabled;
    }
    
    pub fn update_volume(&mut self, volume: f32) -> Result<()> {
        if volume < 0.0 || volume > 1.0 {
            return Err(MetronomeError::InvalidVolume(volume));
        }
        self.volume = volume;
        Ok(())
    }
}

/// GUI state structure for managing UI state
#[derive(Debug, Clone)]
pub struct GuiState {
    pub bpm_input: String,
    pub bpm_valid: bool,
    pub selected_time_signature: TimeSignature,
    pub selected_beat_sound: SoundType,
    pub selected_accent_sound: SoundType,
    pub is_running: bool,
    pub error_message: Option<String>,
    pub last_beat_visual: Option<Instant>,
    pub accent_enabled: bool,
    pub volume: f32,
}

impl GuiState {
    pub fn new() -> Self {
        Self {
            bpm_input: "120".to_string(),
            bpm_valid: true,
            selected_time_signature: TimeSignature::default(),
            selected_beat_sound: SoundType::default(),
            selected_accent_sound: SoundType::BuiltinWood,
            is_running: false,
            error_message: None,
            last_beat_visual: None,
            accent_enabled: true, // Accents enabled by default
            volume: 0.7, // Default volume at 70%
        }
    }
    
    pub fn validate_bpm(&mut self, input: &str) -> Result<u32> {
        match input.trim().parse::<u32>() {
            Ok(bpm) if bpm >= 60 && bpm <= 200 => {
                self.bpm_valid = true;
                self.error_message = None;
                Ok(bpm)
            }
            Ok(bpm) => {
                self.bpm_valid = false;
                self.error_message = Some(format!("BPM must be between 60 and 200 (got {})", bpm));
                Err(MetronomeError::InvalidBpm(bpm))
            }
            Err(_) => {
                self.bpm_valid = false;
                self.error_message = Some("Invalid BPM value. Please enter a number.".to_string());
                Err(MetronomeError::InvalidBpm(0))
            }
        }
    }
    
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
    }
    
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
    
    pub fn update_beat_visual(&mut self) {
        self.last_beat_visual = Some(Instant::now());
    }
    
    pub fn should_show_beat_visual(&self, duration: Duration) -> bool {
        match self.last_beat_visual {
            Some(last_beat) => last_beat.elapsed() < duration,
            None => false,
        }
    }
    
    pub fn set_gui_error(&mut self, error: crate::error::GuiError) {
        self.error_message = Some(error.to_string());
    }
    
    pub fn handle_result<T>(&mut self, result: Result<T>) -> Option<T> {
        match result {
            Ok(value) => {
                self.clear_error();
                Some(value)
            }
            Err(error) => {
                self.set_error(error.to_string());
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_metronome_config_creation() {
        let config = MetronomeConfig::new(120);
        assert_eq!(config.bpm, 120);
        assert!(config.sound_enabled);
        assert!(config.visual_enabled);
    }
    
    #[test]
    fn test_metronome_config_validation() {
        let config = MetronomeConfig::new(120);
        assert!(config.validate().is_ok());
        
        let config = MetronomeConfig::new(59);
        assert!(config.validate().is_err());
        
        let config = MetronomeConfig::new(201);
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_metronome_config_builder() {
        let config = MetronomeConfig::new(120)
            .with_sound(false)
            .with_visual(true)
            .with_accent_enabled(false);
        
        assert!(!config.sound_enabled);
        assert!(config.visual_enabled);
        assert!(!config.accent_enabled);
    }
    
    #[test]
    fn test_beat_creation() {
        let beat = Beat::new(1, TimeSignature::Four, 120);
        assert_eq!(beat.sequence_number, 1);
        assert_eq!(beat.bpm, 120);
        assert_eq!(beat.beat_in_measure, 1);
        assert!(beat.is_accent);
        assert!(beat.is_first_beat());
    }
    
    #[test]
    fn test_beat_creation_with_zero_sequence() {
        // Test that Beat::new handles sequence_number 0 without panicking
        let beat = Beat::new(0, TimeSignature::Four, 120);
        assert_eq!(beat.sequence_number, 1); // Should be normalized to 1
        assert_eq!(beat.bpm, 120);
        assert_eq!(beat.beat_in_measure, 1);
        assert!(beat.is_accent);
        assert!(beat.is_first_beat());
    }
    
    #[test]
    fn test_beat_accent_enabled_disabled() {
        // Test Beat with accents enabled (default)
        let beat_enabled = Beat::new_with_accent_setting(1, TimeSignature::Four, 120, true);
        assert!(beat_enabled.accent_enabled);
        assert_eq!(beat_enabled.get_accent_strength(), 1.0); // Strong beat
        
        let beat_enabled_weak = Beat::new_with_accent_setting(2, TimeSignature::Four, 120, true);
        assert!(beat_enabled_weak.accent_enabled);
        assert_eq!(beat_enabled_weak.get_accent_strength(), 0.0); // Weak beat
        
        // Test Beat with accents disabled
        let beat_disabled = Beat::new_with_accent_setting(1, TimeSignature::Four, 120, false);
        assert!(!beat_disabled.accent_enabled);
        assert_eq!(beat_disabled.get_accent_strength(), 0.0); // No accent when disabled
        
        let beat_disabled_weak = Beat::new_with_accent_setting(2, TimeSignature::Four, 120, false);
        assert!(!beat_disabled_weak.accent_enabled);
        assert_eq!(beat_disabled_weak.get_accent_strength(), 0.0); // Still no accent
    }
    
    #[test]
    fn test_time_signature() {
        assert_eq!(TimeSignature::Four.beats_per_measure(), 4);
        assert_eq!(TimeSignature::Three.beats_per_measure(), 3);
        assert_eq!(TimeSignature::Two.beats_per_measure(), 2);
        assert_eq!(TimeSignature::Six.beats_per_measure(), 6);
        
        assert_eq!(TimeSignature::Four.as_str(), "4/4");
        assert_eq!(TimeSignature::Three.as_str(), "3/4");
    }
    
    #[test]
    fn test_sound_type() {
        assert_eq!(SoundType::BuiltinClick.as_str(), "Click");
        assert_eq!(SoundType::BuiltinWood.as_str(), "Wood");
        assert_eq!(SoundType::BuiltinBeep.as_str(), "Beep");
        
        assert!(SoundType::BuiltinClick.is_builtin());
        assert!(!SoundType::Custom(std::path::PathBuf::from("test.wav")).is_builtin());
    }
    
    #[test]
    fn test_beat_sequence_in_measure() {
        // Test 4/4 time signature
        let beat1 = Beat::new(1, TimeSignature::Four, 120);
        let beat2 = Beat::new(2, TimeSignature::Four, 120);
        let beat3 = Beat::new(3, TimeSignature::Four, 120);
        let beat4 = Beat::new(4, TimeSignature::Four, 120);
        let beat5 = Beat::new(5, TimeSignature::Four, 120); // Next measure
        
        assert_eq!(beat1.beat_in_measure, 1);
        assert!(beat1.is_accent);
        
        assert_eq!(beat2.beat_in_measure, 2);
        assert!(!beat2.is_accent);
        
        assert_eq!(beat3.beat_in_measure, 3);
        // In 4/4 time, beat 3 is a medium accent (not a strong accent but not weak either)
        assert!(beat3.is_accent); // This is now true due to our enhanced accent logic
        
        assert_eq!(beat4.beat_in_measure, 4);
        assert!(!beat4.is_accent);
        
        assert_eq!(beat5.beat_in_measure, 1); // Back to first beat of next measure
        assert!(beat5.is_accent);
    }
    
    #[test]
    fn test_metronome_state_creation() {
        let config = MetronomeConfig::new(120);
        let state = MetronomeState::new(&config);
        
        assert_eq!(state.bpm, 120);
        assert_eq!(state.time_signature, TimeSignature::Four);
        assert!(!state.is_running);
        assert_eq!(state.beat_count, 0);
        assert_eq!(state.current_beat_in_measure, 1);
    }
    
    #[test]
    fn test_metronome_state_start_stop() {
        let config = MetronomeConfig::new(120);
        let mut state = MetronomeState::new(&config);
        
        assert!(!state.is_running);
        state.start();
        assert!(state.is_running);
        assert!(state.start_time.is_some());
        
        state.stop();
        assert!(!state.is_running);
        assert!(state.start_time.is_none());
    }
    
    #[test]
    fn test_metronome_state_bpm_update() {
        let config = MetronomeConfig::new(120);
        let mut state = MetronomeState::new(&config);
        
        assert!(state.update_bpm(140).is_ok());
        assert_eq!(state.bpm, 140);
        
        assert!(state.update_bpm(59).is_err());
        assert!(state.update_bpm(201).is_err());
    }
    
    #[test]
    fn test_gui_state_creation() {
        let gui_state = GuiState::new();
        
        assert_eq!(gui_state.bpm_input, "120");
        assert!(gui_state.bpm_valid);
        assert_eq!(gui_state.selected_time_signature, TimeSignature::Four);
        assert!(!gui_state.is_running);
        assert!(gui_state.error_message.is_none());
    }
    
    #[test]
    fn test_gui_state_bpm_validation() {
        let mut gui_state = GuiState::new();
        
        assert!(gui_state.validate_bpm("120").is_ok());
        assert!(gui_state.bpm_valid);
        assert!(gui_state.error_message.is_none());
        
        assert!(gui_state.validate_bpm("59").is_err());
        assert!(!gui_state.bpm_valid);
        assert!(gui_state.error_message.is_some());
        
        assert!(gui_state.validate_bpm("invalid").is_err());
        assert!(!gui_state.bpm_valid);
        assert!(gui_state.error_message.is_some());
    }
    
    #[test]
    fn test_gui_state_error_handling() {
        let mut gui_state = GuiState::new();
        
        // Test setting and clearing errors
        gui_state.set_error("Test error".to_string());
        assert!(gui_state.error_message.is_some());
        
        gui_state.clear_error();
        assert!(gui_state.error_message.is_none());
        
        // Test handling results
        let ok_result: Result<u32> = Ok(120);
        let value = gui_state.handle_result(ok_result);
        assert_eq!(value, Some(120));
        assert!(gui_state.error_message.is_none());
        
        let err_result: Result<u32> = Err(MetronomeError::InvalidBpm(59));
        let value = gui_state.handle_result(err_result);
        assert_eq!(value, None);
        assert!(gui_state.error_message.is_some());
    }
    
    #[test]
    fn test_time_signature_intervals() {
        let config = MetronomeConfig::new(120); // 120 BPM
        let mut state = MetronomeState::new(&config);
        
        // Test 4/4 time (default)
        let interval_4_4 = state.calculate_beat_interval();
        assert_eq!(interval_4_4, Duration::from_millis(500)); // 120 BPM = 500ms per beat
        
        // Test 6/8 time (compound time)
        state.time_signature = TimeSignature::Six;
        let interval_6_8 = state.calculate_beat_interval();
        // In our implementation, 6/8 time uses the same interval as 4/4 (500ms per beat)
        assert_eq!(interval_6_8, Duration::from_millis(500));
        
        // Test other time signatures
        state.time_signature = TimeSignature::Three;
        let interval_3_4 = state.calculate_beat_interval();
        assert_eq!(interval_3_4, Duration::from_millis(500)); // Same as 4/4
    }
    
    #[test]
    fn test_accent_patterns() {
        let config = MetronomeConfig::new(120);
        let mut state = MetronomeState::new(&config);
        
        // Test 4/4 accent pattern
        state.time_signature = TimeSignature::Four;
        let pattern_4_4 = state.get_accent_pattern();
        assert_eq!(pattern_4_4, vec![true, false, true, false]); // Strong-weak-medium-weak
        
        // Test 3/4 accent pattern
        state.time_signature = TimeSignature::Three;
        let pattern_3_4 = state.get_accent_pattern();
        assert_eq!(pattern_3_4, vec![true, false, false]); // Strong-weak-weak
        
        // Test 6/8 accent pattern
        state.time_signature = TimeSignature::Six;
        let pattern_6_8 = state.get_accent_pattern();
        assert_eq!(pattern_6_8, vec![true, false, false, true, false, false]); // Strong-weak-weak-medium-weak-weak
    }
    
    #[test]
    fn test_accent_strength_calculation() {
        let config = MetronomeConfig::new(120);
        let mut state = MetronomeState::new(&config);
        
        // Test 4/4 accent strengths
        state.time_signature = TimeSignature::Four;
        
        state.current_beat_in_measure = 1;
        assert_eq!(state.get_accent_strength(), 1.0); // Strong beat
        
        state.current_beat_in_measure = 2;
        assert_eq!(state.get_accent_strength(), 0.0); // Weak beat
        
        state.current_beat_in_measure = 3;
        assert_eq!(state.get_accent_strength(), 0.5); // Medium beat
        
        state.current_beat_in_measure = 4;
        assert_eq!(state.get_accent_strength(), 0.0); // Weak beat
    }
    
    #[test]
    fn test_beat_accent_methods() {
        // Test 4/4 time signature beats
        let beat1 = Beat::new(1, TimeSignature::Four, 120);
        assert!(beat1.is_strong_beat());
        assert!(!beat1.is_medium_beat());
        assert!(!beat1.is_weak_beat());
        assert_eq!(beat1.get_accent_strength(), 1.0);
        
        let beat2 = Beat::new(2, TimeSignature::Four, 120);
        assert!(!beat2.is_strong_beat());
        assert!(!beat2.is_medium_beat());
        assert!(beat2.is_weak_beat());
        assert_eq!(beat2.get_accent_strength(), 0.0);
        
        let beat3 = Beat::new(3, TimeSignature::Four, 120);
        assert!(!beat3.is_strong_beat());
        assert!(beat3.is_medium_beat());
        assert!(!beat3.is_weak_beat());
        assert_eq!(beat3.get_accent_strength(), 0.5);
    }
    
    #[test]
    fn test_timing_accuracy_calculation() {
        let config = MetronomeConfig::new(120);
        let mut state = MetronomeState::new(&config);
        state.start();
        
        // Test that timing accuracy calculation works
        // We'll test with a known time offset
        let start_time = state.start_time.unwrap();
        let expected_first_beat = start_time + state.calculate_beat_interval();
        
        // Simulate checking accuracy at the expected time
        let accuracy = state.get_timing_accuracy(expected_first_beat);
        assert!(accuracy.is_some());
        
        // The accuracy should be very small (close to 0) when timing is perfect
        if let Some(acc) = accuracy {
            assert!(acc.as_millis() <= 1); // Should be very close to expected time
        }
    }
    
    #[test]
    fn test_volume_functionality() {
        // Test MetronomeConfig with volume
        let config = MetronomeConfig::new(120).with_volume(0.5);
        assert_eq!(config.volume, 0.5);
        
        // Test volume clamping
        let config_high = MetronomeConfig::new(120).with_volume(1.5);
        assert_eq!(config_high.volume, 1.0); // Should be clamped to 1.0
        
        let config_low = MetronomeConfig::new(120).with_volume(-0.5);
        assert_eq!(config_low.volume, 0.0); // Should be clamped to 0.0
        
        // Test MetronomeState volume update
        let config = MetronomeConfig::new(120);
        let mut state = MetronomeState::new(&config);
        
        assert_eq!(state.volume, 0.7); // Default volume
        
        // Test valid volume update
        assert!(state.update_volume(0.8).is_ok());
        assert_eq!(state.volume, 0.8);
        
        // Test invalid volume updates
        assert!(state.update_volume(-0.1).is_err());
        assert!(state.update_volume(1.1).is_err());
        assert_eq!(state.volume, 0.8); // Should remain unchanged after invalid update
    }
    
    #[test]
    fn test_gui_state_volume() {
        let gui_state = GuiState::new();
        assert_eq!(gui_state.volume, 0.7); // Default volume
    }
}