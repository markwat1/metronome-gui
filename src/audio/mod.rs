use crate::error::{AudioError, Result};
use crate::models::SoundType;
use std::collections::HashMap;
use std::path::Path;

/// Sound data structure for caching audio samples
#[derive(Debug, Clone)]
pub struct SoundData {
    pub sound_type: SoundType,
    pub data: Vec<u8>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl SoundData {
    /// Create sound data from built-in sound type
    pub fn from_builtin(sound_type: SoundType) -> Result<Self> {
        match sound_type {
            SoundType::BuiltinClick => Ok(Self::generate_click_sound()),
            SoundType::BuiltinWood => Ok(Self::generate_wood_sound()),
            SoundType::BuiltinBeep => Ok(Self::generate_beep_sound()),
            SoundType::Custom(_) => Err(AudioError::UnsupportedFormat("Cannot create built-in sound from custom type".to_string()).into()),
        }
    }
    
    /// Create sound data from file
    pub fn from_file(path: &Path) -> Result<Self> {
        use std::fs;
        
        // Validate file exists
        if !path.exists() {
            return Err(AudioError::SoundLoadError(format!("File not found: {}", path.display())).into());
        }
        
        // Validate file size (limit to 10MB for safety)
        let metadata = fs::metadata(path)
            .map_err(|e| AudioError::SoundLoadError(format!("Cannot read file metadata: {}", e)))?;
        
        if metadata.len() > 10 * 1024 * 1024 {
            return Err(AudioError::SoundLoadError("File too large (max 10MB)".to_string()).into());
        }
        
        let sound_type = SoundType::Custom(path.to_path_buf());
        
        // Validate file format by extension
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase());
        
        match extension.as_deref() {
            Some("wav") => Self::load_wav_file(path, sound_type),
            Some("mp3") => Self::load_mp3_file(path, sound_type),
            Some("ogg") => Self::load_ogg_file(path, sound_type),
            Some(ext) => Err(AudioError::UnsupportedFormat(format!("Unsupported file format: {}", ext)).into()),
            None => Err(AudioError::UnsupportedFormat("No file extension found".to_string()).into()),
        }
    }
    
    /// Load WAV file (placeholder implementation)
    fn load_wav_file(path: &Path, sound_type: SoundType) -> Result<Self> {
        // For now, we'll generate a placeholder sound
        // In a real implementation, this would use a library like hound to parse WAV files
        
        // Try to read file to validate it exists and is readable
        std::fs::File::open(path)
            .map_err(|e| AudioError::SoundLoadError(format!("Cannot open WAV file: {}", e)))?;
        
        // Generate placeholder sound based on filename
        Ok(Self::generate_placeholder_custom_sound(sound_type))
    }
    
    /// Load MP3 file (placeholder implementation)
    fn load_mp3_file(path: &Path, sound_type: SoundType) -> Result<Self> {
        // For now, we'll generate a placeholder sound
        // In a real implementation, this would use a library like symphonia to parse MP3 files
        
        // Try to read file to validate it exists and is readable
        std::fs::File::open(path)
            .map_err(|e| AudioError::SoundLoadError(format!("Cannot open MP3 file: {}", e)))?;
        
        // Generate placeholder sound based on filename
        Ok(Self::generate_placeholder_custom_sound(sound_type))
    }
    
    /// Load OGG file (placeholder implementation)
    fn load_ogg_file(path: &Path, sound_type: SoundType) -> Result<Self> {
        // For now, we'll generate a placeholder sound
        // In a real implementation, this would use a library like lewton or symphonia to parse OGG files
        
        // Try to read file to validate it exists and is readable
        std::fs::File::open(path)
            .map_err(|e| AudioError::SoundLoadError(format!("Cannot open OGG file: {}", e)))?;
        
        // Generate placeholder sound based on filename
        Ok(Self::generate_placeholder_custom_sound(sound_type))
    }
    
    /// Generate click sound data
    fn generate_click_sound() -> Self {
        let sample_rate = 44100;
        let duration_ms = 50;
        let samples = (sample_rate * duration_ms / 1000) as usize;
        
        // Generate a sharp click sound (high frequency burst)
        let mut data = Vec::with_capacity(samples * 4); // 4 bytes per f32 sample
        for i in 0..samples {
            let t = i as f32 / sample_rate as f32;
            let frequency = 2000.0; // High frequency for click
            let amplitude = 0.5;
            
            // Sharp attack, quick decay
            let envelope = if i < samples / 10 {
                1.0
            } else {
                (1.0 - (i - samples / 10) as f32 / (samples - samples / 10) as f32).max(0.0)
            };
            
            let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin() * amplitude * envelope;
            data.extend_from_slice(&sample.to_le_bytes());
        }
        
        Self {
            sound_type: SoundType::BuiltinClick,
            data,
            sample_rate,
            channels: 1,
        }
    }
    
    /// Generate wood block sound data
    fn generate_wood_sound() -> Self {
        let sample_rate = 44100;
        let duration_ms = 80;
        let samples = (sample_rate * duration_ms / 1000) as usize;
        
        // Generate a wood block sound (lower frequency with harmonics)
        let mut data = Vec::with_capacity(samples * 4);
        for i in 0..samples {
            let t = i as f32 / sample_rate as f32;
            let fundamental = 800.0;
            let amplitude = 0.4;
            
            // Wood block has multiple harmonics
            let harmonic1 = (t * fundamental * 2.0 * std::f32::consts::PI).sin() * 0.6;
            let harmonic2 = (t * fundamental * 3.0 * 2.0 * std::f32::consts::PI).sin() * 0.3;
            let harmonic3 = (t * fundamental * 5.0 * 2.0 * std::f32::consts::PI).sin() * 0.1;
            
            // Exponential decay envelope
            let envelope = (-t * 8.0).exp();
            
            let sample = (harmonic1 + harmonic2 + harmonic3) * amplitude * envelope;
            data.extend_from_slice(&sample.to_le_bytes());
        }
        
        Self {
            sound_type: SoundType::BuiltinWood,
            data,
            sample_rate,
            channels: 1,
        }
    }
    
    /// Generate beep sound data
    fn generate_beep_sound() -> Self {
        let sample_rate = 44100;
        let duration_ms = 100;
        let samples = (sample_rate * duration_ms / 1000) as usize;
        
        // Generate a clean beep sound
        let mut data = Vec::with_capacity(samples * 4);
        for i in 0..samples {
            let t = i as f32 / sample_rate as f32;
            let frequency = 1000.0; // Mid frequency for beep
            let amplitude = 0.3;
            
            // Smooth envelope to avoid clicks
            let envelope = if i < 1000 {
                i as f32 / 1000.0
            } else if i > samples - 1000 {
                (samples - i) as f32 / 1000.0
            } else {
                1.0
            };
            
            let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin() * amplitude * envelope;
            data.extend_from_slice(&sample.to_le_bytes());
        }
        
        Self {
            sound_type: SoundType::BuiltinBeep,
            data,
            sample_rate,
            channels: 1,
        }
    }
    
    /// Generate placeholder custom sound (for demonstration)
    fn generate_placeholder_custom_sound(sound_type: SoundType) -> Self {
        // For now, generate a unique sound based on the filename
        let sample_rate = 44100;
        let duration_ms = 120;
        let samples = (sample_rate * duration_ms / 1000) as usize;
        
        // Use a different frequency based on the sound type
        let frequency = match &sound_type {
            SoundType::Custom(path) => {
                // Generate frequency based on filename hash for uniqueness
                let filename = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("default");
                let hash = filename.chars().fold(0u32, |acc, c| acc.wrapping_add(c as u32));
                600.0 + (hash % 800) as f32 // Frequency between 600-1400 Hz
            }
            _ => 800.0,
        };
        
        let mut data = Vec::with_capacity(samples * 4);
        for i in 0..samples {
            let t = i as f32 / sample_rate as f32;
            let amplitude = 0.35;
            
            // Smooth envelope
            let envelope = if i < 2000 {
                i as f32 / 2000.0
            } else if i > samples - 2000 {
                (samples - i) as f32 / 2000.0
            } else {
                1.0
            };
            
            let sample = (t * frequency * 2.0 * std::f32::consts::PI).sin() * amplitude * envelope;
            data.extend_from_slice(&sample.to_le_bytes());
        }
        
        Self {
            sound_type,
            data,
            sample_rate,
            channels: 1,
        }
    }
}

pub trait AudioPlayer {
    fn play_sound(&self, sound_type: &SoundType) -> Result<()>;
    fn play_sound_with_volume(&self, sound_type: &SoundType, volume: f32) -> Result<()>;
    fn is_available(&self) -> bool;
    fn initialize(&mut self) -> Result<()>;
    fn preload_sounds(&mut self, sounds: &[SoundType]) -> Result<()>;
    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct CrossPlatformAudio {
    player: Option<Box<dyn AudioPlayer>>,
    fallback_enabled: bool,
    sound_cache: HashMap<SoundType, SoundData>,
}

impl CrossPlatformAudio {
    pub fn new() -> Self {
        Self {
            player: None,
            fallback_enabled: true,
            sound_cache: HashMap::new(),
        }
    }
    
    pub fn initialize(&mut self) -> Result<()> {
        // Try to initialize the best available audio player for the platform
        #[cfg(feature = "audio")]
        {
            match self.create_platform_player() {
                Ok(mut player) => {
                    player.initialize()?;
                    
                    // Preload built-in sounds
                    let builtin_sounds = SoundType::builtin_sounds();
                    player.preload_sounds(builtin_sounds)?;
                    
                    self.player = Some(player);
                    self.load_builtin_sounds()?;
                    Ok(())
                }
                Err(e) => {
                    if self.fallback_enabled {
                        eprintln!("Warning: Audio initialization failed ({}), using visual-only mode", e);
                        // Still load sounds for potential future use
                        let _ = self.load_builtin_sounds();
                        Ok(())
                    } else {
                        Err(e)
                    }
                }
            }
        }
        
        #[cfg(not(feature = "audio"))]
        {
            if self.fallback_enabled {
                eprintln!("Audio support not compiled in, using visual-only mode");
                // Still load sound data for potential future use
                let _ = self.load_builtin_sounds();
                Ok(())
            } else {
                Err(AudioError::DeviceNotAvailable.into())
            }
        }
    }
    
    #[cfg(feature = "audio")]
    fn create_platform_player(&self) -> Result<Box<dyn AudioPlayer>> {
        Ok(Box::new(RodioAudioPlayer::new()?))
    }
    
    pub fn play_sound(&self, sound_type: &SoundType) -> Result<()> {
        match &self.player {
            Some(player) => player.play_sound(sound_type),
            None => {
                // Fallback to visual indication only
                Ok(())
            }
        }
    }
    
    pub fn play_sound_with_volume(&self, sound_type: &SoundType, volume: f32) -> Result<()> {
        match &self.player {
            Some(player) => player.play_sound_with_volume(sound_type, volume),
            None => {
                // Fallback to visual indication only
                Ok(())
            }
        }
    }
    
    pub fn play_beat_sound(&self) -> Result<()> {
        self.play_sound(&SoundType::BuiltinClick)
    }
    
    /// Play a beat sound (regular beat)
    pub fn play_beat(&self, beat_sound: &SoundType) -> Result<()> {
        self.play_sound(beat_sound)
    }
    
    /// Play an accent sound (strong beat)
    pub fn play_accent(&self, accent_sound: &SoundType) -> Result<()> {
        self.play_sound(accent_sound)
    }
    
    /// Play the appropriate sound based on beat type
    pub fn play_beat_with_accent(&self, is_accent: bool, beat_sound: &SoundType, accent_sound: &SoundType) -> Result<()> {
        if is_accent {
            self.play_accent(accent_sound)
        } else {
            self.play_beat(beat_sound)
        }
    }
    
    pub fn is_audio_available(&self) -> bool {
        self.player.as_ref().map_or(false, |p| p.is_available())
    }
    
    pub fn set_fallback_enabled(&mut self, enabled: bool) {
        self.fallback_enabled = enabled;
    }
    
    pub fn with_fallback(mut self, enabled: bool) -> Self {
        self.fallback_enabled = enabled;
        self
    }
    
    #[cfg(feature = "audio")]
    pub fn test_audio_system(&self) -> bool {
        // Simple test that doesn't require thread safety
        RodioAudioPlayer::new().is_ok()
    }
    
    #[cfg(not(feature = "audio"))]
    pub fn test_audio_system(&self) -> bool {
        false
    }
    
    pub fn get_audio_status(&self) -> AudioStatus {
        match &self.player {
            Some(player) if player.is_available() => AudioStatus::Available,
            Some(_) => AudioStatus::Unavailable,
            None => {
                if self.fallback_enabled {
                    AudioStatus::FallbackMode
                } else {
                    AudioStatus::Disabled
                }
            }
        }
    }
    
    /// Load all built-in sounds into cache
    pub fn load_builtin_sounds(&mut self) -> Result<()> {
        for sound_type in SoundType::builtin_sounds() {
            let sound_data = SoundData::from_builtin(sound_type.clone())?;
            self.sound_cache.insert(sound_type.clone(), sound_data);
        }
        Ok(())
    }
    
    /// Load a custom sound file into cache with fallback
    pub fn load_custom_sound(&mut self, path: &Path) -> Result<SoundType> {
        match SoundData::from_file(path) {
            Ok(sound_data) => {
                let sound_type = sound_data.sound_type.clone();
                self.sound_cache.insert(sound_type.clone(), sound_data);
                
                // If we have an active player, preload this sound
                if let Some(player) = &mut self.player {
                    if let Err(e) = player.preload_sounds(&[sound_type.clone()]) {
                        eprintln!("Warning: Failed to preload custom sound in player: {}", e);
                        // Continue anyway, sound is cached
                    }
                }
                
                Ok(sound_type)
            }
            Err(e) => {
                eprintln!("Warning: Failed to load custom sound file '{}': {}", path.display(), e);
                eprintln!("Falling back to built-in click sound");
                
                // Fallback to built-in click sound
                let fallback_sound = SoundType::BuiltinClick;
                if !self.is_sound_cached(&fallback_sound) {
                    let sound_data = SoundData::from_builtin(fallback_sound.clone())?;
                    self.sound_cache.insert(fallback_sound.clone(), sound_data);
                }
                
                Ok(fallback_sound)
            }
        }
    }
    
    /// Load a custom sound file into cache without fallback (for testing)
    pub fn load_custom_sound_strict(&mut self, path: &Path) -> Result<SoundType> {
        let sound_data = SoundData::from_file(path)?;
        let sound_type = sound_data.sound_type.clone();
        self.sound_cache.insert(sound_type.clone(), sound_data);
        
        // If we have an active player, preload this sound
        if let Some(player) = &mut self.player {
            player.preload_sounds(&[sound_type.clone()])?;
        }
        
        Ok(sound_type)
    }
    
    /// Get cached sound data
    pub fn get_sound_data(&self, sound_type: &SoundType) -> Option<&SoundData> {
        self.sound_cache.get(sound_type)
    }
    
    /// Check if a sound is cached
    pub fn is_sound_cached(&self, sound_type: &SoundType) -> bool {
        self.sound_cache.contains_key(sound_type)
    }
    
    /// Get all cached sound types
    pub fn get_cached_sounds(&self) -> Vec<SoundType> {
        self.sound_cache.keys().cloned().collect()
    }
    
    /// Clear sound cache
    pub fn clear_cache(&mut self) {
        self.sound_cache.clear();
    }
    
    /// Preload sounds for better performance with fallback for custom files
    pub fn preload_sounds(&mut self, sounds: &[SoundType]) -> Result<()> {
        let mut successful_sounds = Vec::new();
        
        // Load into cache first
        for sound_type in sounds {
            if !self.is_sound_cached(sound_type) {
                match sound_type {
                    SoundType::BuiltinClick | SoundType::BuiltinWood | SoundType::BuiltinBeep => {
                        match SoundData::from_builtin(sound_type.clone()) {
                            Ok(sound_data) => {
                                self.sound_cache.insert(sound_type.clone(), sound_data);
                                successful_sounds.push(sound_type.clone());
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to load built-in sound {:?}: {}", sound_type, e);
                            }
                        }
                    }
                    SoundType::Custom(path) => {
                        match SoundData::from_file(path) {
                            Ok(sound_data) => {
                                self.sound_cache.insert(sound_type.clone(), sound_data);
                                successful_sounds.push(sound_type.clone());
                            }
                            Err(e) => {
                                eprintln!("Warning: Failed to load custom sound '{}': {}", path.display(), e);
                                eprintln!("Using built-in click sound as fallback");
                                
                                // Use built-in click as fallback
                                let fallback = SoundType::BuiltinClick;
                                if !self.is_sound_cached(&fallback) {
                                    if let Ok(sound_data) = SoundData::from_builtin(fallback.clone()) {
                                        self.sound_cache.insert(fallback.clone(), sound_data);
                                    }
                                }
                                successful_sounds.push(fallback);
                            }
                        }
                    }
                }
            } else {
                successful_sounds.push(sound_type.clone());
            }
        }
        
        // Preload in player if available
        if let Some(player) = &mut self.player {
            if let Err(e) = player.preload_sounds(&successful_sounds) {
                eprintln!("Warning: Failed to preload sounds in player: {}", e);
                // Continue anyway, sounds are cached
            }
        }
        
        Ok(())
    }
    
    /// Validate that a sound file can be loaded
    pub fn validate_sound_file(path: &Path) -> Result<()> {
        use std::fs;
        
        // Check if file exists
        if !path.exists() {
            return Err(AudioError::SoundLoadError(format!("File not found: {}", path.display())).into());
        }
        
        // Check if it's a file (not a directory)
        let metadata = fs::metadata(path)
            .map_err(|e| AudioError::SoundLoadError(format!("Cannot read file metadata: {}", e)))?;
        
        if !metadata.is_file() {
            return Err(AudioError::SoundLoadError(format!("Path is not a file: {}", path.display())).into());
        }
        
        // Check file size (limit to 10MB)
        if metadata.len() > 10 * 1024 * 1024 {
            return Err(AudioError::SoundLoadError("File too large (max 10MB)".to_string()).into());
        }
        
        // Check file extension
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase());
        
        match extension.as_deref() {
            Some("wav") | Some("mp3") | Some("ogg") => Ok(()),
            Some(ext) => Err(AudioError::UnsupportedFormat(format!("Unsupported file format: {}", ext)).into()),
            None => Err(AudioError::UnsupportedFormat("No file extension found".to_string()).into()),
        }
    }
    
    /// Get supported file extensions
    pub fn get_supported_extensions() -> &'static [&'static str] {
        &["wav", "mp3", "ogg"]
    }
}

impl Default for CrossPlatformAudio {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioStatus {
    Available,
    Unavailable,
    FallbackMode,
    Disabled,
}

impl std::fmt::Display for AudioStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AudioStatus::Available => write!(f, "Audio available"),
            AudioStatus::Unavailable => write!(f, "Audio unavailable"),
            AudioStatus::FallbackMode => write!(f, "Visual-only mode"),
            AudioStatus::Disabled => write!(f, "Audio disabled"),
        }
    }
}

// Rodio-based audio player (only compiled when audio feature is enabled)
#[cfg(feature = "audio")]
mod rodio_player {
    use super::*;
    use rodio::{OutputStream, OutputStreamHandle, Sink};
    use std::time::Duration;

    pub struct RodioAudioPlayer {
        _stream: OutputStream,
        stream_handle: OutputStreamHandle,
        sink: Option<Sink>,
        sound_cache: HashMap<SoundType, Vec<f32>>,
    }

    impl RodioAudioPlayer {
        pub fn new() -> Result<Self> {
            let (stream, stream_handle) = OutputStream::try_default()
                .map_err(|e| AudioError::InitializationFailed(e.to_string()))?;
            
            Ok(Self {
                _stream: stream,
                stream_handle,
                sink: None,
                sound_cache: HashMap::new(),
            })
        }
        
        fn generate_beep_sound(&self, frequency: f32, duration: Duration) -> Vec<f32> {
            let sample_rate = 44100;
            let samples = (sample_rate as f32 * duration.as_secs_f32()) as usize;
            
            (0..samples)
                .map(move |i| {
                    let t = i as f32 / sample_rate as f32;
                    let amplitude = 0.3; // Volume level
                    let wave = (t * frequency * 2.0 * std::f32::consts::PI).sin();
                    
                    // Apply envelope to avoid clicks
                    let envelope = if i < 1000 {
                        i as f32 / 1000.0
                    } else if i > samples - 1000 {
                        (samples - i) as f32 / 1000.0
                    } else {
                        1.0
                    };
                    
                    wave * amplitude * envelope
                })
                .collect()
        }
        
        fn generate_sound_samples(&self, sound_type: &SoundType) -> Result<Vec<f32>> {
            match sound_type {
                SoundType::BuiltinClick => Ok(self.generate_click_samples()),
                SoundType::BuiltinWood => Ok(self.generate_wood_samples()),
                SoundType::BuiltinBeep => Ok(self.generate_beep_sound(1000.0, Duration::from_millis(100))),
                SoundType::Custom(path) => self.load_custom_samples(path),
            }
        }
        
        fn generate_click_samples(&self) -> Vec<f32> {
            let sample_rate = 44100;
            let duration_ms = 50;
            let samples = (sample_rate * duration_ms / 1000) as usize;
            
            (0..samples)
                .map(|i| {
                    let t = i as f32 / sample_rate as f32;
                    let frequency = 2000.0; // High frequency for click
                    let amplitude = 0.5;
                    
                    // Sharp attack, quick decay
                    let envelope = if i < samples / 10 {
                        1.0
                    } else {
                        (1.0 - (i - samples / 10) as f32 / (samples - samples / 10) as f32).max(0.0)
                    };
                    
                    (t * frequency * 2.0 * std::f32::consts::PI).sin() * amplitude * envelope
                })
                .collect()
        }
        
        fn generate_wood_samples(&self) -> Vec<f32> {
            let sample_rate = 44100;
            let duration_ms = 80;
            let samples = (sample_rate * duration_ms / 1000) as usize;
            
            (0..samples)
                .map(|i| {
                    let t = i as f32 / sample_rate as f32;
                    let fundamental = 800.0;
                    let amplitude = 0.4;
                    
                    // Wood block has multiple harmonics
                    let harmonic1 = (t * fundamental * 2.0 * std::f32::consts::PI).sin() * 0.6;
                    let harmonic2 = (t * fundamental * 3.0 * 2.0 * std::f32::consts::PI).sin() * 0.3;
                    let harmonic3 = (t * fundamental * 5.0 * 2.0 * std::f32::consts::PI).sin() * 0.1;
                    
                    // Exponential decay envelope
                    let envelope = (-t * 8.0).exp();
                    
                    (harmonic1 + harmonic2 + harmonic3) * amplitude * envelope
                })
                .collect()
        }
        
        fn load_custom_samples(&self, path: &Path) -> Result<Vec<f32>> {
            // For now, generate a placeholder sound based on filename
            // In a real implementation, this would use a library like symphonia or hound
            let filename = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("default");
            let hash = filename.chars().fold(0u32, |acc, c| acc.wrapping_add(c as u32));
            let frequency = 600.0 + (hash % 800) as f32; // Frequency between 600-1400 Hz
            
            Ok(self.generate_beep_sound(frequency, Duration::from_millis(120)))
        }
    }

    impl AudioPlayer for RodioAudioPlayer {
        fn play_sound(&self, sound_type: &SoundType) -> Result<()> {
            use rodio::buffer::SamplesBuffer;
            
            // Create a new sink for each sound to avoid blocking
            let sink = Sink::try_new(&self.stream_handle)
                .map_err(|e| AudioError::PlaybackFailed(e.to_string()))?;
            
            // Get sound samples from cache or generate them
            let samples = if let Some(cached_samples) = self.sound_cache.get(sound_type) {
                cached_samples.clone()
            } else {
                // Generate sound on-the-fly if not cached
                self.generate_sound_samples(sound_type)?
            };
            
            let sound_source = SamplesBuffer::new(1, 44100, samples);
            sink.append(sound_source);
            sink.detach(); // Let it play independently
            
            Ok(())
        }
        
        fn is_available(&self) -> bool {
            // Try to create a test sink to check if audio is available
            Sink::try_new(&self.stream_handle).is_ok()
        }
        
        fn initialize(&mut self) -> Result<()> {
            // Test audio system by creating a sink
            let test_sink = Sink::try_new(&self.stream_handle)
                .map_err(|e| AudioError::InitializationFailed(e.to_string()))?;
            
            self.sink = Some(test_sink);
            Ok(())
        }
        
        fn preload_sounds(&mut self, sounds: &[SoundType]) -> Result<()> {
            for sound_type in sounds {
                if !self.sound_cache.contains_key(sound_type) {
                    let samples = self.generate_sound_samples(sound_type)?;
                    self.sound_cache.insert(sound_type.clone(), samples);
                }
            }
            Ok(())
        }
        
        fn play_sound_with_volume(&self, sound_type: &SoundType, volume: f32) -> Result<()> {
            use rodio::buffer::SamplesBuffer;
            
            // Clamp volume to valid range
            let volume = volume.clamp(0.0, 1.0);
            
            // Create a new sink for each sound to avoid blocking
            let sink = Sink::try_new(&self.stream_handle)
                .map_err(|e| AudioError::PlaybackFailed(e.to_string()))?;
            
            // Set volume on the sink
            sink.set_volume(volume);
            
            // Get sound samples from cache or generate them
            let samples = if let Some(cached_samples) = self.sound_cache.get(sound_type) {
                cached_samples.clone()
            } else {
                // Generate sound on-the-fly if not cached
                self.generate_sound_samples(sound_type)?
            };
            
            let sound_source = SamplesBuffer::new(1, 44100, samples);
            sink.append(sound_source);
            sink.detach(); // Let it play independently
            
            Ok(())
        }
        
        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }
}

#[cfg(feature = "audio")]
pub use rodio_player::RodioAudioPlayer;

/// High-level audio engine for metronome sounds
pub struct AudioEngine {
    audio_system: CrossPlatformAudio,
    beat_sound: SoundType,
    accent_sound: SoundType,
}

impl AudioEngine {
    /// Create a new audio engine
    pub fn new() -> Result<Self> {
        let mut audio_system = CrossPlatformAudio::new();
        audio_system.initialize()?;
        
        Ok(Self {
            audio_system,
            beat_sound: SoundType::BuiltinClick,
            accent_sound: SoundType::BuiltinWood,
        })
    }
    
    /// Set the beat sound (regular beats)
    pub fn set_beat_sound(&mut self, sound_type: SoundType) -> Result<()> {
        // Preload the sound if it's not cached
        if !self.audio_system.is_sound_cached(&sound_type) {
            self.audio_system.preload_sounds(&[sound_type.clone()])?;
        }
        self.beat_sound = sound_type;
        Ok(())
    }
    
    /// Set the accent sound (strong beats)
    pub fn set_accent_sound(&mut self, sound_type: SoundType) -> Result<()> {
        // Preload the sound if it's not cached
        if !self.audio_system.is_sound_cached(&sound_type) {
            self.audio_system.preload_sounds(&[sound_type.clone()])?;
        }
        self.accent_sound = sound_type;
        Ok(())
    }
    
    /// Set both beat and accent sounds
    pub fn set_sounds(&mut self, beat_sound: SoundType, accent_sound: SoundType) -> Result<()> {
        // Preload both sounds
        let sounds = vec![beat_sound.clone(), accent_sound.clone()];
        self.audio_system.preload_sounds(&sounds)?;
        
        self.beat_sound = beat_sound;
        self.accent_sound = accent_sound;
        Ok(())
    }
    
    /// Play a regular beat
    pub fn play_beat(&self) -> Result<()> {
        self.audio_system.play_beat(&self.beat_sound)
    }
    
    /// Play an accent beat
    pub fn play_accent(&self) -> Result<()> {
        self.audio_system.play_accent(&self.accent_sound)
    }
    
    /// Play the appropriate sound based on whether it's an accent beat
    pub fn play_beat_with_accent(&self, is_accent: bool) -> Result<()> {
        self.audio_system.play_beat_with_accent(is_accent, &self.beat_sound, &self.accent_sound)
    }
    
    /// Play sound based on beat information
    pub fn play_beat_from_info(&self, beat: &crate::models::Beat) -> Result<()> {
        self.play_beat_with_accent(beat.is_accent)
    }
    
    /// Load a custom sound file and return the sound type
    pub fn load_custom_sound(&mut self, path: &Path) -> Result<SoundType> {
        self.audio_system.load_custom_sound(path)
    }
    
    /// Check if audio is available
    pub fn is_available(&self) -> bool {
        self.audio_system.is_audio_available()
    }
    
    /// Get audio status
    pub fn get_status(&self) -> AudioStatus {
        self.audio_system.get_audio_status()
    }
    
    /// Get current beat sound
    pub fn get_beat_sound(&self) -> &SoundType {
        &self.beat_sound
    }
    
    /// Get current accent sound
    pub fn get_accent_sound(&self) -> &SoundType {
        &self.accent_sound
    }
    
    /// Get all cached sounds
    pub fn get_cached_sounds(&self) -> Vec<SoundType> {
        self.audio_system.get_cached_sounds()
    }
    
    /// Validate a sound file before loading
    pub fn validate_sound_file(path: &Path) -> Result<()> {
        CrossPlatformAudio::validate_sound_file(path)
    }
    
    /// Get supported file extensions
    pub fn get_supported_extensions() -> &'static [&'static str] {
        CrossPlatformAudio::get_supported_extensions()
    }
}

impl Default for AudioEngine {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Fallback to a minimal audio engine if initialization fails
            Self {
                audio_system: CrossPlatformAudio::new(),
                beat_sound: SoundType::BuiltinClick,
                accent_sound: SoundType::BuiltinWood,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_creation() {
        let audio = CrossPlatformAudio::new();
        assert!(!audio.is_audio_available());
    }
    
    #[test]
    fn test_audio_fallback() {
        let audio = CrossPlatformAudio::new().with_fallback(true);
        assert_eq!(audio.get_audio_status(), AudioStatus::FallbackMode);
    }
    
    #[test]
    fn test_audio_status_display() {
        assert_eq!(AudioStatus::Available.to_string(), "Audio available");
        assert_eq!(AudioStatus::Unavailable.to_string(), "Audio unavailable");
        assert_eq!(AudioStatus::FallbackMode.to_string(), "Visual-only mode");
        assert_eq!(AudioStatus::Disabled.to_string(), "Audio disabled");
    }
    
    #[test]
    fn test_audio_initialization_without_feature() {
        let mut audio = CrossPlatformAudio::new();
        // Should succeed with fallback enabled
        assert!(audio.initialize().is_ok());
    }
    
    #[test]
    fn test_play_beat_without_audio() {
        let audio = CrossPlatformAudio::new();
        // Should not fail even without audio
        assert!(audio.play_beat_sound().is_ok());
    }
    
    #[test]
    fn test_sound_data_creation() {
        // Test built-in sound creation
        let click_data = SoundData::from_builtin(SoundType::BuiltinClick);
        assert!(click_data.is_ok());
        
        let wood_data = SoundData::from_builtin(SoundType::BuiltinWood);
        assert!(wood_data.is_ok());
        
        let beep_data = SoundData::from_builtin(SoundType::BuiltinBeep);
        assert!(beep_data.is_ok());
        
        // Test that custom type fails for built-in creation
        let custom_path = std::path::PathBuf::from("test.wav");
        let custom_data = SoundData::from_builtin(SoundType::Custom(custom_path));
        assert!(custom_data.is_err());
    }
    
    #[test]
    fn test_sound_cache_operations() {
        let mut audio = CrossPlatformAudio::new();
        
        // Test loading built-in sounds
        assert!(audio.load_builtin_sounds().is_ok());
        
        // Check that sounds are cached
        assert!(audio.is_sound_cached(&SoundType::BuiltinClick));
        assert!(audio.is_sound_cached(&SoundType::BuiltinWood));
        assert!(audio.is_sound_cached(&SoundType::BuiltinBeep));
        
        // Test getting cached sounds
        let cached_sounds = audio.get_cached_sounds();
        assert_eq!(cached_sounds.len(), 3);
        
        // Test getting sound data
        let click_data = audio.get_sound_data(&SoundType::BuiltinClick);
        assert!(click_data.is_some());
        
        // Test clearing cache
        audio.clear_cache();
        assert!(!audio.is_sound_cached(&SoundType::BuiltinClick));
    }
    
    #[test]
    fn test_preload_sounds() {
        let mut audio = CrossPlatformAudio::new();
        
        let sounds_to_preload = vec![
            SoundType::BuiltinClick,
            SoundType::BuiltinWood,
        ];
        
        assert!(audio.preload_sounds(&sounds_to_preload).is_ok());
        
        // Check that sounds are cached
        for sound in &sounds_to_preload {
            assert!(audio.is_sound_cached(sound));
        }
    }
    
    #[test]
    fn test_custom_sound_validation() {
        use std::path::PathBuf;
        
        // Test non-existent file
        let non_existent = PathBuf::from("non_existent.wav");
        assert!(CrossPlatformAudio::validate_sound_file(&non_existent).is_err());
        
        // Test supported extensions
        let supported = CrossPlatformAudio::get_supported_extensions();
        assert!(supported.contains(&"wav"));
        assert!(supported.contains(&"mp3"));
        assert!(supported.contains(&"ogg"));
    }
    
    #[test]
    fn test_custom_sound_fallback() {
        let mut audio = CrossPlatformAudio::new();
        
        // Try to load a non-existent custom sound - should fallback to built-in
        let non_existent_path = std::path::PathBuf::from("non_existent.wav");
        let result = audio.load_custom_sound(&non_existent_path);
        
        // Should succeed with fallback
        assert!(result.is_ok());
        
        // Should have fallen back to built-in click
        let sound_type = result.unwrap();
        assert_eq!(sound_type, SoundType::BuiltinClick);
        assert!(audio.is_sound_cached(&sound_type));
    }
    
    #[test]
    fn test_custom_sound_strict_loading() {
        let mut audio = CrossPlatformAudio::new();
        
        // Try to load a non-existent custom sound strictly - should fail
        let non_existent_path = std::path::PathBuf::from("non_existent.wav");
        let result = audio.load_custom_sound_strict(&non_existent_path);
        
        // Should fail without fallback
        assert!(result.is_err());
    }
    
    #[test]
    fn test_preload_with_custom_fallback() {
        let mut audio = CrossPlatformAudio::new();
        
        let sounds_to_preload = vec![
            SoundType::BuiltinClick,
            SoundType::Custom(std::path::PathBuf::from("non_existent.wav")),
        ];
        
        // Should succeed even with invalid custom sound (fallback)
        assert!(audio.preload_sounds(&sounds_to_preload).is_ok());
        
        // Built-in click should be cached (both original and fallback)
        assert!(audio.is_sound_cached(&SoundType::BuiltinClick));
    }
    
    #[test]
    fn test_audio_engine_creation() {
        // Test default creation
        let engine = AudioEngine::default();
        assert_eq!(engine.get_beat_sound(), &SoundType::BuiltinClick);
        assert_eq!(engine.get_accent_sound(), &SoundType::BuiltinWood);
    }
    
    #[test]
    fn test_audio_engine_sound_setting() {
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
    fn test_audio_engine_playback() {
        let engine = AudioEngine::default();
        
        // Test basic playback (should not fail even without audio)
        assert!(engine.play_beat().is_ok());
        assert!(engine.play_accent().is_ok());
        assert!(engine.play_beat_with_accent(false).is_ok());
        assert!(engine.play_beat_with_accent(true).is_ok());
    }
    
    #[test]
    fn test_audio_engine_beat_info_playback() {
        use crate::models::{Beat, TimeSignature};
        
        let engine = AudioEngine::default();
        
        // Test playing from beat info
        let regular_beat = Beat::new(2, TimeSignature::Four, 120); // Second beat in 4/4 (weak)
        let accent_beat = Beat::new(1, TimeSignature::Four, 120);  // First beat in 4/4 (strong)
        
        assert!(engine.play_beat_from_info(&regular_beat).is_ok());
        assert!(engine.play_beat_from_info(&accent_beat).is_ok());
    }
    
    #[test]
    fn test_audio_engine_file_operations() {
        let mut engine = AudioEngine::default();
        
        // Test supported extensions
        let extensions = AudioEngine::get_supported_extensions();
        assert!(extensions.contains(&"wav"));
        assert!(extensions.contains(&"mp3"));
        assert!(extensions.contains(&"ogg"));
        
        // Test file validation
        let non_existent = std::path::PathBuf::from("non_existent.wav");
        assert!(AudioEngine::validate_sound_file(&non_existent).is_err());
        
        // Test loading custom sound (should fallback)
        let result = engine.load_custom_sound(&non_existent);
        assert!(result.is_ok()); // Should succeed with fallback
    }
}