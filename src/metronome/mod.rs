use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::error::{MetronomeError, Result};
use crate::models::{Beat, MetronomeConfig, MetronomeState};

#[derive(Debug)]
pub struct Metronome {
    state: Arc<Mutex<MetronomeState>>,
}

impl Metronome {
    pub fn new() -> Self {
        let config = MetronomeConfig::default();
        let state = MetronomeState::new(&config);
        
        Self {
            state: Arc::new(Mutex::new(state)),
        }
    }
    
    pub fn from_config(config: MetronomeConfig) -> Result<Self> {
        config.validate()?;
        let state = MetronomeState::new(&config);
        
        Ok(Self {
            state: Arc::new(Mutex::new(state)),
        })
    }
    
    pub fn with_bpm(bpm: u32) -> Result<Self> {
        let config = MetronomeConfig::new(bpm);
        config.validate()?;
        Self::from_config(config)
    }
    
    pub fn set_bpm(&self, bpm: u32) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        state.update_bpm(bpm)
    }
    
    pub fn set_time_signature(&self, time_signature: crate::models::TimeSignature) {
        let mut state = self.state.lock().unwrap();
        state.update_time_signature(time_signature);
    }
    
    pub fn set_sounds(&self, beat_sound: crate::models::SoundType, accent_sound: crate::models::SoundType) {
        let mut state = self.state.lock().unwrap();
        state.update_sounds(beat_sound, accent_sound);
    }
    
    pub fn set_accent_enabled(&self, accent_enabled: bool) {
        let mut state = self.state.lock().unwrap();
        state.update_accent_enabled(accent_enabled);
    }
    
    pub fn set_volume(&self, volume: f32) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        state.update_volume(volume)
    }
    
    pub fn get_volume(&self) -> f32 {
        let state = self.state.lock().unwrap();
        state.volume
    }
    
    pub fn get_time_signature(&self) -> crate::models::TimeSignature {
        let state = self.state.lock().unwrap();
        state.time_signature
    }
    
    pub fn get_current_beat_in_measure(&self) -> u32 {
        let state = self.state.lock().unwrap();
        state.current_beat_in_measure
    }
    
    pub fn get_beats_per_measure(&self) -> u32 {
        let state = self.state.lock().unwrap();
        state.time_signature.beats_per_measure()
    }
    
    pub fn is_accent_beat(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.current_beat_in_measure == 1
    }
    
    pub fn get_beat_sound(&self) -> crate::models::SoundType {
        let state = self.state.lock().unwrap();
        if state.current_beat_in_measure == 1 {
            state.accent_sound.clone()
        } else {
            state.beat_sound.clone()
        }
    }
    
    pub fn reset_beat_position(&self) {
        let mut state = self.state.lock().unwrap();
        state.current_beat_in_measure = 1;
    }
    
    /// Get a clone of the state Arc for sharing across threads
    pub fn get_state_arc(&self) -> Arc<Mutex<MetronomeState>> {
        Arc::clone(&self.state)
    }
    
    /// Update multiple settings atomically
    pub fn update_settings(&self, bpm: Option<u32>, time_signature: Option<crate::models::TimeSignature>, 
                          beat_sound: Option<crate::models::SoundType>, accent_sound: Option<crate::models::SoundType>,
                          accent_enabled: Option<bool>, volume: Option<f32>) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        
        if let Some(bpm) = bpm {
            if bpm < 60 || bpm > 200 {
                return Err(MetronomeError::InvalidBpm(bpm));
            }
            state.bpm = bpm;
        }
        
        if let Some(time_sig) = time_signature {
            state.update_time_signature(time_sig);
        }
        
        if let Some(beat) = beat_sound {
            state.beat_sound = beat;
        }
        
        if let Some(accent) = accent_sound {
            state.accent_sound = accent;
        }
        
        if let Some(enabled) = accent_enabled {
            state.update_accent_enabled(enabled);
        }
        
        if let Some(vol) = volume {
            state.update_volume(vol)?;
        }
        
        Ok(())
    }
    
    /// Thread-safe method to check if a beat should be played
    pub fn should_play_beat_safe(&self, last_beat_time: std::time::Instant) -> bool {
        let state = self.state.lock().unwrap();
        if !state.is_running {
            return false;
        }
        last_beat_time.elapsed() >= state.get_interval()
    }
    
    /// Get current sound type for the current beat position (thread-safe)
    pub fn get_current_sound_type(&self) -> crate::models::SoundType {
        let state = self.state.lock().unwrap();
        if state.current_beat_in_measure == 1 {
            state.accent_sound.clone()
        } else {
            state.beat_sound.clone()
        }
    }
    
    pub fn start(&self) -> Result<()> {
        let mut state = self.state.lock().unwrap();
        state.start();
        Ok(())
    }
    
    pub fn stop(&self) {
        let mut state = self.state.lock().unwrap();
        state.stop();
    }
    
    pub fn is_running(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.is_running
    }
    
    pub fn get_bpm(&self) -> u32 {
        let state = self.state.lock().unwrap();
        state.bpm
    }
    
    pub fn get_interval(&self) -> Duration {
        let state = self.state.lock().unwrap();
        state.get_interval()
    }
    
    pub fn get_elapsed_time(&self) -> Duration {
        let state = self.state.lock().unwrap();
        state.get_elapsed_time()
    }
    
    pub fn get_beat_count(&self) -> u64 {
        let state = self.state.lock().unwrap();
        state.beat_count
    }
    
    pub fn increment_beat(&self) -> Beat {
        let mut state = self.state.lock().unwrap();
        state.increment_beat()
    }
    
    pub fn get_state(&self) -> MetronomeState {
        let state = self.state.lock().unwrap();
        state.clone()
    }
    
    pub fn should_play_beat(&self, last_beat_time: Instant) -> bool {
        self.should_play_beat_safe(last_beat_time)
    }
    
    /// Create a future-compatible metronome runner
    #[cfg(feature = "gui")]
    pub async fn run_async<F>(&self, mut beat_callback: F) -> Result<()>
    where
        F: FnMut(crate::models::Beat, crate::models::SoundType) + Send + 'static,
    {
        use tokio::time::{sleep, Duration as TokioDuration};
        
        let mut last_beat_time = Instant::now();
        
        while self.is_running() {
            if self.should_play_beat_safe(last_beat_time) {
                let beat = self.increment_beat();
                let sound_type = self.get_current_sound_type();
                
                beat_callback(beat, sound_type);
                last_beat_time = Instant::now();
            }
            
            // Small async sleep to prevent excessive CPU usage
            sleep(TokioDuration::from_millis(1)).await;
        }
        
        Ok(())
    }
    
    /// Get a snapshot of the current state for display purposes
    pub fn get_display_state(&self) -> (u32, u64, Duration, crate::models::TimeSignature, u32, bool) {
        let state = self.state.lock().unwrap();
        (
            state.bpm,
            state.beat_count,
            state.get_elapsed_time(),
            state.time_signature,
            state.current_beat_in_measure,
            state.is_running,
        )
    }
}
use std::sync::atomic::{AtomicBool, Ordering};


pub struct MetronomeController {
    metronome: Metronome,
    running: Arc<AtomicBool>,
}

impl MetronomeController {
    pub fn new(bpm: u32) -> Result<Self> {
        let metronome = Metronome::with_bpm(bpm)?;
        let running = Arc::new(AtomicBool::new(false));
        
        Ok(Self {
            metronome,
            running,
        })
    }
    
    pub fn setup_ctrl_c_handler(&self) -> Result<()> {
        let running = Arc::clone(&self.running);
        
        ctrlc::set_handler(move || {
            println!("\nReceived Ctrl+C, stopping metronome...");
            running.store(false, Ordering::SeqCst);
        }).map_err(|e| MetronomeError::SystemError(format!("Failed to set Ctrl+C handler: {}", e)))?;
        
        Ok(())
    }
    
    pub fn start(&mut self) -> Result<()> {
        self.running.store(true, Ordering::SeqCst);
        self.metronome.start()
    }
    
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        self.metronome.stop();
    }
    
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst) && self.metronome.is_running()
    }
    
    pub fn should_continue(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
    
    pub fn get_metronome(&self) -> &Metronome {
        &self.metronome
    }
    
    pub fn get_metronome_mut(&mut self) -> &mut Metronome {
        &mut self.metronome
    }
    
    /// Start the metronome with thread-safe control
    pub fn start_safe(&self) -> Result<()> {
        self.running.store(true, Ordering::SeqCst);
        self.metronome.start()
    }
    
    /// Stop the metronome with thread-safe control
    pub fn stop_safe(&self) {
        self.running.store(false, Ordering::SeqCst);
        self.metronome.stop();
    }
    
    /// Check if the controller should continue running (thread-safe)
    pub fn should_continue_safe(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
    
    /// Get the metronome state Arc for sharing across threads
    pub fn get_metronome_state_arc(&self) -> Arc<Mutex<MetronomeState>> {
        self.metronome.get_state_arc()
    }
    
    /// Update metronome settings atomically
    pub fn update_metronome_settings(&self, bpm: Option<u32>, time_signature: Option<crate::models::TimeSignature>, 
                                   beat_sound: Option<crate::models::SoundType>, accent_sound: Option<crate::models::SoundType>,
                                   accent_enabled: Option<bool>, volume: Option<f32>) -> Result<()> {
        self.metronome.update_settings(bpm, time_signature, beat_sound, accent_sound, accent_enabled, volume)
    }
}#[cfg
(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    
    #[test]
    fn test_metronome_creation() {
        let metronome = Metronome::with_bpm(120).unwrap();
        assert_eq!(metronome.get_bpm(), 120);
        assert!(!metronome.is_running());
        assert_eq!(metronome.get_beat_count(), 0);
    }
    
    #[test]
    fn test_metronome_invalid_bpm() {
        assert!(Metronome::with_bpm(59).is_err());
        assert!(Metronome::with_bpm(201).is_err());
    }
    
    #[test]
    fn test_interval_calculation() {
        let metronome = Metronome::with_bpm(60).unwrap();
        assert_eq!(metronome.get_interval(), Duration::from_secs(1));
        
        let metronome = Metronome::with_bpm(120).unwrap();
        assert_eq!(metronome.get_interval(), Duration::from_millis(500));
    }
    
    #[test]
    fn test_metronome_start_stop() {
        let metronome = Metronome::with_bpm(120).unwrap();
        
        assert!(!metronome.is_running());
        metronome.start().unwrap();
        assert!(metronome.is_running());
        
        metronome.stop();
        assert!(!metronome.is_running());
    }
    
    #[test]
    fn test_beat_increment() {
        let metronome = Metronome::with_bpm(120).unwrap();
        metronome.start().unwrap();
        
        let beat1 = metronome.increment_beat();
        assert_eq!(beat1.sequence_number, 1);
        assert_eq!(beat1.bpm, 120);
        assert_eq!(beat1.beat_in_measure, 1);
        assert!(beat1.is_accent);
        
        let beat2 = metronome.increment_beat();
        assert_eq!(beat2.sequence_number, 2);
        assert_eq!(beat2.beat_in_measure, 2);
        assert!(!beat2.is_accent);
        assert_eq!(metronome.get_beat_count(), 2);
    }
    
    #[test]
    fn test_metronome_controller() {
        let controller = MetronomeController::new(120).unwrap();
        assert!(!controller.is_running());
        assert_eq!(controller.get_metronome().get_bpm(), 120);
    }
    
    #[test]
    fn test_time_signature_functionality() {
        use crate::models::TimeSignature;
        
        let metronome = Metronome::with_bpm(120).unwrap();
        
        // Test default time signature (4/4)
        assert_eq!(metronome.get_time_signature(), TimeSignature::Four);
        assert_eq!(metronome.get_beats_per_measure(), 4);
        assert_eq!(metronome.get_current_beat_in_measure(), 1);
        assert!(metronome.is_accent_beat());
        
        // Test changing time signature
        metronome.set_time_signature(TimeSignature::Three);
        assert_eq!(metronome.get_time_signature(), TimeSignature::Three);
        assert_eq!(metronome.get_beats_per_measure(), 3);
    }
    
    #[test]
    fn test_beat_progression_in_measure() {
        use crate::models::TimeSignature;
        
        let metronome = Metronome::with_bpm(120).unwrap();
        metronome.set_time_signature(TimeSignature::Three); // 3/4 time
        metronome.start().unwrap();
        
        // First beat - accent
        let beat1 = metronome.increment_beat();
        assert_eq!(beat1.beat_in_measure, 1);
        assert!(beat1.is_accent);
        assert!(metronome.is_accent_beat());
        
        // Second beat - regular
        let beat2 = metronome.increment_beat();
        assert_eq!(beat2.beat_in_measure, 2);
        assert!(!beat2.is_accent);
        assert!(!metronome.is_accent_beat());
        
        // Third beat - regular
        let beat3 = metronome.increment_beat();
        assert_eq!(beat3.beat_in_measure, 3);
        assert!(!beat3.is_accent);
        assert!(!metronome.is_accent_beat());
        
        // Fourth beat - back to accent (new measure)
        let beat4 = metronome.increment_beat();
        assert_eq!(beat4.beat_in_measure, 1);
        assert!(beat4.is_accent);
        assert!(metronome.is_accent_beat());
    }
    
    #[test]
    fn test_sound_selection_based_on_beat() {
        use crate::models::{SoundType, TimeSignature};
        
        let metronome = Metronome::with_bpm(120).unwrap();
        metronome.set_sounds(SoundType::BuiltinClick, SoundType::BuiltinWood);
        metronome.set_time_signature(TimeSignature::Four);
        metronome.start().unwrap();
        
        // First beat should use accent sound
        metronome.increment_beat();
        let sound = metronome.get_beat_sound();
        assert_eq!(sound, SoundType::BuiltinWood); // accent sound
        
        // Second beat should use regular sound
        metronome.increment_beat();
        let sound = metronome.get_beat_sound();
        assert_eq!(sound, SoundType::BuiltinClick); // beat sound
    }
    
    #[test]
    fn test_beat_position_reset() {
        let metronome = Metronome::with_bpm(120).unwrap();
        metronome.start().unwrap();
        
        // Advance a few beats
        metronome.increment_beat();
        metronome.increment_beat();
        metronome.increment_beat();
        assert_eq!(metronome.get_current_beat_in_measure(), 3);
        
        // Reset position
        metronome.reset_beat_position();
        assert_eq!(metronome.get_current_beat_in_measure(), 1);
        assert!(metronome.is_accent_beat());
    }
    
    #[test]
    fn test_atomic_settings_update() {
        use crate::models::{SoundType, TimeSignature};
        
        let metronome = Metronome::with_bpm(120).unwrap();
        
        // Update multiple settings atomically
        let result = metronome.update_settings(
            Some(140),
            Some(TimeSignature::Three),
            Some(SoundType::BuiltinWood),
            Some(SoundType::BuiltinBeep),
            None,
            None
        );
        
        assert!(result.is_ok());
        assert_eq!(metronome.get_bpm(), 140);
        assert_eq!(metronome.get_time_signature(), TimeSignature::Three);
        
        // Test invalid BPM in atomic update
        let result = metronome.update_settings(Some(300), None, None, None, None, None);
        assert!(result.is_err());
        // BPM should remain unchanged after failed update
        assert_eq!(metronome.get_bpm(), 140);
    }
    
    #[test]
    fn test_thread_safe_beat_checking() {
        let metronome = Metronome::with_bpm(120).unwrap();
        metronome.start().unwrap();
        
        let last_beat = std::time::Instant::now();
        
        // Should not play beat immediately
        assert!(!metronome.should_play_beat_safe(last_beat));
        
        // Stop metronome
        metronome.stop();
        
        // Should not play beat when stopped, even after time passes
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert!(!metronome.should_play_beat_safe(last_beat));
    }
    
    #[test]
    fn test_display_state_snapshot() {
        use crate::models::TimeSignature;
        
        let metronome = Metronome::with_bpm(120).unwrap();
        metronome.set_time_signature(TimeSignature::Three);
        metronome.start().unwrap();
        
        let (bpm, beat_count, _elapsed, time_sig, current_beat, is_running) = metronome.get_display_state();
        
        assert_eq!(bpm, 120);
        assert_eq!(beat_count, 0);
        assert_eq!(time_sig, TimeSignature::Three);
        assert_eq!(current_beat, 1);
        assert!(is_running);
    }
    
    #[test]
    fn test_controller_thread_safe_operations() {
        let controller = MetronomeController::new(120).unwrap();
        
        // Test thread-safe start/stop
        assert!(controller.start_safe().is_ok());
        assert!(controller.should_continue_safe());
        assert!(controller.is_running());
        
        controller.stop_safe();
        assert!(!controller.should_continue_safe());
        assert!(!controller.is_running());
        
        // Test atomic settings update through controller
        let result = controller.update_metronome_settings(
            Some(140), 
            Some(crate::models::TimeSignature::Four), 
            None, 
            None,
            None,
            None
        );
        assert!(result.is_ok());
        assert_eq!(controller.get_metronome().get_bpm(), 140);
    }
    
    #[test]
    fn test_volume_control() {
        let metronome = Metronome::with_bpm(120).unwrap();
        
        // Test default volume
        assert_eq!(metronome.get_volume(), 0.7);
        
        // Test setting valid volume
        assert!(metronome.set_volume(0.5).is_ok());
        assert_eq!(metronome.get_volume(), 0.5);
        
        // Test setting invalid volumes
        assert!(metronome.set_volume(-0.1).is_err());
        assert!(metronome.set_volume(1.1).is_err());
        assert_eq!(metronome.get_volume(), 0.5); // Should remain unchanged
        
        // Test volume in atomic settings update
        let result = metronome.update_settings(None, None, None, None, None, Some(0.8));
        assert!(result.is_ok());
        assert_eq!(metronome.get_volume(), 0.8);
        
        // Test invalid volume in atomic update
        let result = metronome.update_settings(None, None, None, None, None, Some(1.5));
        assert!(result.is_err());
        assert_eq!(metronome.get_volume(), 0.8); // Should remain unchanged after failed update
    }
}