use std::io::{self, Write};
use std::time::{Duration, Instant};
use crate::audio::AudioStatus;
use crate::models::{TimeSignature, Beat};

pub struct DisplayEngine {
    start_time: Option<Instant>,
}

impl DisplayEngine {
    pub fn new() -> Self {
        Self {
            start_time: None,
        }
    }
    
    pub fn set_start_time(&mut self, start_time: Instant) {
        self.start_time = Some(start_time);
    }
    
    pub fn show_startup_info(&self, bpm: u32, time_signature: TimeSignature, audio_status: &AudioStatus) {
        println!("CLI Metronome v0.1.0");
        println!("==================");
        println!("BPM: {}", bpm);
        println!("Time Signature: {}", time_signature.as_str());
        println!("Audio Status: {}", audio_status);
        println!("Press Ctrl+C to stop");
        println!();
        self.show_time_signature_legend(time_signature);
        println!();
    }
    
    pub fn show_status(&self, bpm: u32, beat_count: u64, elapsed: Duration, time_signature: TimeSignature, current_beat_in_measure: u32) {
        let elapsed_secs = elapsed.as_secs();
        let elapsed_mins = elapsed_secs / 60;
        let elapsed_secs = elapsed_secs % 60;
        
        // Clear the current line and move cursor to beginning
        print!("\r");
        
        // Show enhanced status information with time signature and beat position
        print!(
            "BPM: \x1b[1m{:3}\x1b[0m | Beat: \x1b[1m{:4}\x1b[0m | Time: \x1b[32m{:02}:{:02}\x1b[0m | \x1b[36m{}\x1b[0m: ",
            bpm, beat_count, elapsed_mins, elapsed_secs, time_signature.as_str()
        );
        
        // Show visual beat position within measure with enhanced indicators
        self.show_inline_beat_position(time_signature, current_beat_in_measure);
        
        print!(" | ");
        
        // Flush to ensure immediate display
        io::stdout().flush().unwrap();
    }
    
    /// Show inline beat position within the status line
    fn show_inline_beat_position(&self, time_signature: TimeSignature, current_beat: u32) {
        let beats_per_measure = time_signature.beats_per_measure();
        
        for i in 1..=beats_per_measure {
            if i == current_beat {
                // Highlight current beat based on its strength
                let temp_beat = crate::models::Beat::new(i as u64, time_signature, 120);
                if temp_beat.is_strong_beat() {
                    print!("\x1b[1;31m●\x1b[0m"); // Bold red for strong beats
                } else if temp_beat.is_medium_beat() {
                    print!("\x1b[1;33m◐\x1b[0m"); // Bold yellow for medium beats
                } else {
                    print!("\x1b[1;37m○\x1b[0m"); // Bold white for current weak beat
                }
            } else {
                // Show other beats as dim indicators
                let temp_beat = crate::models::Beat::new(i as u64, time_signature, 120);
                if temp_beat.is_strong_beat() {
                    print!("\x1b[2;31m●\x1b[0m"); // Dim red for non-current strong beats
                } else if temp_beat.is_medium_beat() {
                    print!("\x1b[2;33m◐\x1b[0m"); // Dim yellow for non-current medium beats
                } else {
                    print!("\x1b[2m○\x1b[0m"); // Dim for non-current weak beats
                }
            }
            
            if i < beats_per_measure {
                print!(" ");
            }
        }
        
        print!(" ({}/{})", current_beat, beats_per_measure);
    }
    
    pub fn clear_line(&self) {
        print!("\r");
        // Clear the entire line
        print!("\x1B[2K");
        io::stdout().flush().unwrap();
    }
    
    pub fn move_to_next_line(&self) {
        println!();
    }
    
    /// Show beat indicator with strong/weak beat distinction based on time signature
    pub fn show_beat_indicator(&self, beat: &Beat) {
        let indicator = self.get_beat_indicator_symbol(beat);
        print!("{}", indicator);
        io::stdout().flush().unwrap();
    }
    
    /// Show visual beat with enhanced time signature support
    pub fn show_visual_beat(&self, beat: &Beat) {
        let indicator = self.get_visual_beat_symbol(beat);
        print!(" {} ", indicator);
        io::stdout().flush().unwrap();
    }
    
    /// Get the appropriate beat indicator symbol based on beat strength
    fn get_beat_indicator_symbol(&self, beat: &Beat) -> &'static str {
        let strength = beat.get_accent_strength();
        
        if strength >= 1.0 {
            "♪♫♪"  // Strong beat (accent)
        } else if strength > 0.0 {
            "♪♫"   // Medium beat (partial accent)
        } else {
            "♪"    // Weak beat
        }
    }
    
    /// Get the appropriate visual beat symbol based on beat strength
    fn get_visual_beat_symbol(&self, beat: &Beat) -> &'static str {
        let strength = beat.get_accent_strength();
        
        if strength >= 1.0 {
            "●"    // Strong beat (solid circle)
        } else if strength > 0.0 {
            "◐"    // Medium beat (half-filled circle)
        } else {
            "○"    // Weak beat (empty circle)
        }
    }
    
    /// Show a comprehensive legend explaining the time signature pattern with enhanced visuals
    pub fn show_time_signature_legend(&self, time_signature: TimeSignature) {
        println!("\x1b[1mBeat Pattern Legend:\x1b[0m");
        println!("  \x1b[1;31m●\x1b[0m = Strong beat (accent)");
        println!("  \x1b[1;33m◐\x1b[0m = Medium beat (partial accent)");
        println!("  \x1b[2m○\x1b[0m = Weak beat (no accent)");
        println!();
        
        match time_signature {
            TimeSignature::Two => {
                println!("  \x1b[36m2/4 Time Signature:\x1b[0m");
                println!("    Beat positions: 1 2");
                println!("    Pattern: \x1b[1;31m●\x1b[0m \x1b[2m○\x1b[0m");
                println!("    Description: Strong-weak");
            }
            TimeSignature::Three => {
                println!("  \x1b[36m3/4 Time Signature:\x1b[0m");
                println!("    Beat positions: 1 2 3");
                println!("    Pattern: \x1b[1;31m●\x1b[0m \x1b[2m○\x1b[0m \x1b[2m○\x1b[0m");
                println!("    Description: Strong-weak-weak (waltz time)");
            }
            TimeSignature::Four => {
                println!("  \x1b[36m4/4 Time Signature:\x1b[0m");
                println!("    Beat positions: 1 2 3 4");
                println!("    Pattern: \x1b[1;31m●\x1b[0m \x1b[2m○\x1b[0m \x1b[1;33m◐\x1b[0m \x1b[2m○\x1b[0m");
                println!("    Description: Strong-weak-medium-weak (common time)");
            }
            TimeSignature::Six => {
                println!("  \x1b[36m6/8 Time Signature:\x1b[0m");
                println!("    Beat positions: 1 2 3 4 5 6");
                println!("    Pattern: \x1b[1;31m●\x1b[0m \x1b[2m○\x1b[0m \x1b[2m○\x1b[0m \x1b[1;33m◐\x1b[0m \x1b[2m○\x1b[0m \x1b[2m○\x1b[0m");
                println!("    Description: Strong-weak-weak-medium-weak-weak (compound time)");
            }
        }
        println!();
    }
    
    /// Show enhanced time signature information with current beat context
    pub fn show_enhanced_time_signature_info(&self, time_signature: TimeSignature, current_beat: u32) {
        let time_sig_str = time_signature.as_str();
        let beats_per_measure = time_signature.beats_per_measure();
        
        print!("\x1b[36m{}\x1b[0m time | Beat \x1b[1m{}\x1b[0m of \x1b[1m{}\x1b[0m | ", 
               time_sig_str, current_beat, beats_per_measure);
        
        // Show visual representation of current position in measure
        for i in 1..=beats_per_measure {
            if i == current_beat {
                print!("\x1b[1;37m[\x1b[0m");
            }
            
            // Determine beat strength for visual representation
            let temp_beat = crate::models::Beat::new(i as u64, time_signature, 120);
            if temp_beat.is_strong_beat() {
                print!("\x1b[1;31m●\x1b[0m");
            } else if temp_beat.is_medium_beat() {
                print!("\x1b[1;33m◐\x1b[0m");
            } else {
                print!("\x1b[2m○\x1b[0m");
            }
            
            if i == current_beat {
                print!("\x1b[1;37m]\x1b[0m");
            }
            
            if i < beats_per_measure {
                print!(" ");
            }
        }
        
        io::stdout().flush().unwrap();
    }
    
    /// Show enhanced beat indicator with measure position and strong/weak beat distinction
    pub fn show_enhanced_beat_indicator(&self, beat: &Beat) {
        // Show the visual beat symbol with enhanced distinction
        let visual_symbol = self.get_enhanced_visual_beat_symbol(beat);
        
        // Show beat position in measure with brackets for clarity
        print!(" [{}] {}/{} ", 
               visual_symbol, 
               beat.beat_in_measure, 
               beat.time_signature.beats_per_measure());
        
        // Add extra emphasis for strong beats with color coding if terminal supports it
        if beat.is_strong_beat() {
            print!("\x1b[1;31m!\x1b[0m"); // Bold red exclamation for strong beats
        } else if beat.is_medium_beat() {
            print!("\x1b[1;33m~\x1b[0m"); // Bold yellow tilde for medium beats
        } else {
            print!("\x1b[2m.\x1b[0m"); // Dim dot for weak beats
        }
        
        io::stdout().flush().unwrap();
    }
    
    /// Get enhanced visual beat symbol with better strong/weak distinction
    fn get_enhanced_visual_beat_symbol(&self, beat: &Beat) -> &'static str {
        let strength = beat.get_accent_strength();
        
        if strength >= 1.0 {
            "\x1b[1;31m●\x1b[0m"    // Bold red solid circle for strong beats
        } else if strength > 0.0 {
            "\x1b[1;33m◐\x1b[0m"    // Bold yellow half-filled circle for medium beats
        } else {
            "\x1b[2m○\x1b[0m"       // Dim empty circle for weak beats
        }
    }
    
    /// Show comprehensive beat indicator with time signature context
    pub fn show_comprehensive_beat_indicator(&self, beat: &Beat) {
        // Clear current line and show comprehensive beat information
        print!("\r");
        
        // Show time signature context
        let time_sig_display = self.get_time_signature_display(beat.time_signature);
        print!("{} ", time_sig_display);
        
        // Show current beat position with visual emphasis
        let beat_display = self.get_beat_position_display(beat);
        print!("{} ", beat_display);
        
        // Show beat strength indicator
        let strength_indicator = self.get_beat_strength_indicator(beat);
        print!("{} ", strength_indicator);
        
        // Show measure progress bar
        let progress_bar = self.get_measure_progress_bar(beat);
        print!("{}", progress_bar);
        
        io::stdout().flush().unwrap();
    }
    
    /// Get time signature display with current beat emphasis
    fn get_time_signature_display(&self, time_signature: TimeSignature) -> String {
        match time_signature {
            TimeSignature::Two => format!("\x1b[36m2/4\x1b[0m"),     // Cyan
            TimeSignature::Three => format!("\x1b[36m3/4\x1b[0m"),   // Cyan
            TimeSignature::Four => format!("\x1b[36m4/4\x1b[0m"),    // Cyan
            TimeSignature::Six => format!("\x1b[36m6/8\x1b[0m"),     // Cyan
        }
    }
    
    /// Get beat position display with visual emphasis
    fn get_beat_position_display(&self, beat: &Beat) -> String {
        let total_beats = beat.time_signature.beats_per_measure();
        let current_beat = beat.beat_in_measure;
        
        let mut display = String::new();
        display.push('[');
        
        for i in 1..=total_beats {
            if i == current_beat {
                // Highlight current beat based on strength
                if beat.is_strong_beat() {
                    display.push_str("\x1b[1;31m●\x1b[0m"); // Bold red
                } else if beat.is_medium_beat() {
                    display.push_str("\x1b[1;33m◐\x1b[0m"); // Bold yellow
                } else {
                    display.push_str("\x1b[1;37m○\x1b[0m"); // Bold white
                }
            } else {
                // Show other beats as dim dots
                display.push_str("\x1b[2m·\x1b[0m");
            }
            
            if i < total_beats {
                display.push(' ');
            }
        }
        
        display.push(']');
        display
    }
    
    /// Get beat strength indicator with descriptive text
    fn get_beat_strength_indicator(&self, beat: &Beat) -> String {
        let strength = beat.get_accent_strength();
        
        if strength >= 1.0 {
            format!("\x1b[1;31mSTRONG\x1b[0m")  // Bold red
        } else if strength > 0.0 {
            format!("\x1b[1;33mMEDIUM\x1b[0m")  // Bold yellow
        } else {
            format!("\x1b[2mweak\x1b[0m")       // Dim
        }
    }
    
    /// Get measure progress bar showing position within measure
    fn get_measure_progress_bar(&self, beat: &Beat) -> String {
        let total_beats = beat.time_signature.beats_per_measure();
        let current_beat = beat.beat_in_measure;
        let progress = current_beat as f32 / total_beats as f32;
        
        let bar_width = 20;
        let filled_width = (progress * bar_width as f32) as usize;
        
        let mut bar = String::new();
        bar.push('[');
        
        for i in 0..bar_width {
            if i < filled_width {
                bar.push_str("\x1b[32m█\x1b[0m"); // Green filled
            } else {
                bar.push_str("\x1b[2m░\x1b[0m"); // Dim empty
            }
        }
        
        bar.push(']');
        bar.push_str(&format!(" {}/{}", current_beat, total_beats));
        bar
    }
    
    pub fn show_help() {
        println!("\x1b[1mCLI Metronome - Usage Help\x1b[0m");
        println!("=========================");
        println!();
        println!("\x1b[1mUSAGE:\x1b[0m");
        println!("    cli-metronome [BPM]");
        println!();
        println!("\x1b[1mARGUMENTS:\x1b[0m");
        println!("    <BPM>    Beats per minute (60-200) - enables CLI mode");
        println!();
        println!("\x1b[1mOPTIONS:\x1b[0m");
        println!("    -h, --help    Show this help message");
        println!();
        println!("\x1b[1mMODES:\x1b[0m");
        println!("    \x1b[32mGUI Mode (default):\x1b[0m Run without arguments for graphical interface");
        println!("    \x1b[33mCLI Mode:\x1b[0m Provide BPM argument for command-line operation");
        println!();
        println!("\x1b[1mEXAMPLES:\x1b[0m");
        println!("    cli-metronome           Start GUI mode");
        println!("    cli-metronome 120       Start CLI mode at 120 BPM");
        println!("    cli-metronome 80        Start CLI mode at 80 BPM");
        println!("    cli-metronome 180       Start CLI mode at 180 BPM");
        println!();
        println!("\x1b[1mCLI MODE FEATURES:\x1b[0m");
        println!("    - Time signature support (\x1b[36m2/4, 3/4, 4/4, 6/8\x1b[0m)");
        println!("    - Strong/weak beat visual indicators:");
        println!("      \x1b[1;31m●\x1b[0m = Strong beat (accent)");
        println!("      \x1b[1;33m◐\x1b[0m = Medium beat (partial accent)");
        println!("      \x1b[2m○\x1b[0m = Weak beat (no accent)");
        println!("    - Beat position display within measures");
        println!("    - Real-time measure progress visualization");
        println!("    - Audio and visual feedback");
        println!("    - Color-coded beat strength indicators");
        println!();
        println!("\x1b[1mCONTROLS:\x1b[0m");
        println!("    \x1b[31mCtrl+C\x1b[0m    Stop the metronome");
        println!();
        println!("\x1b[1mNOTES:\x1b[0m");
        println!("    - The metronome supports both audio and visual indicators");
        println!("    - If audio is not available, visual-only mode will be used");
        println!("    - BPM must be between 60 and 200");
        println!("    - Visual indicators use colors when terminal supports them");
        println!("    - Time signatures follow standard musical conventions");
    }
    
    /// Show a detailed explanation of time signature patterns
    pub fn show_time_signature_help(&self) {
        println!("\x1b[1mTime Signature Guide\x1b[0m");
        println!("===================");
        println!();
        println!("\x1b[36m2/4 Time (Two-Four):\x1b[0m");
        println!("  - 2 beats per measure, quarter note gets the beat");
        println!("  - Pattern: \x1b[1;31mSTRONG\x1b[0m-weak");
        println!("  - Common in marches and polkas");
        println!();
        println!("\x1b[36m3/4 Time (Three-Four):\x1b[0m");
        println!("  - 3 beats per measure, quarter note gets the beat");
        println!("  - Pattern: \x1b[1;31mSTRONG\x1b[0m-weak-weak");
        println!("  - Common in waltzes and folk music");
        println!();
        println!("\x1b[36m4/4 Time (Four-Four):\x1b[0m");
        println!("  - 4 beats per measure, quarter note gets the beat");
        println!("  - Pattern: \x1b[1;31mSTRONG\x1b[0m-weak-\x1b[1;33mMEDIUM\x1b[0m-weak");
        println!("  - Most common time signature in popular music");
        println!();
        println!("\x1b[36m6/8 Time (Six-Eight):\x1b[0m");
        println!("  - 6 beats per measure, eighth note gets the beat");
        println!("  - Pattern: \x1b[1;31mSTRONG\x1b[0m-weak-weak-\x1b[1;33mMEDIUM\x1b[0m-weak-weak");
        println!("  - Compound time, feels like two groups of three");
        println!("  - Common in ballads and folk music");
        println!();
    }
    
    pub fn show_error(&self, error: &str) {
        eprintln!("Error: {}", error);
    }
    
    pub fn show_goodbye(&self) {
        println!("\n\x1b[32mMetronome stopped. Goodbye!\x1b[0m");
    }
    
    /// Show a real-time beat visualization with strong/weak beat emphasis
    pub fn show_realtime_beat_visualization(&self, beat: &Beat) {
        // Clear the line and show comprehensive beat information
        print!("\r\x1B[2K"); // Clear entire line
        
        // Show time signature
        print!("\x1b[36m{}\x1b[0m ", beat.time_signature.as_str());
        
        // Show beat sequence with current beat highlighted
        let beats_per_measure = beat.time_signature.beats_per_measure();
        print!("[");
        for i in 1..=beats_per_measure {
            if i == beat.beat_in_measure {
                // Current beat - show with emphasis based on strength
                if beat.is_strong_beat() {
                    print!("\x1b[1;31m●\x1b[0m"); // Bold red
                } else if beat.is_medium_beat() {
                    print!("\x1b[1;33m◐\x1b[0m"); // Bold yellow
                } else {
                    print!("\x1b[1;37m○\x1b[0m"); // Bold white
                }
            } else {
                // Other beats - show dimmed
                let temp_beat = crate::models::Beat::new(i as u64, beat.time_signature, beat.bpm);
                if temp_beat.is_strong_beat() {
                    print!("\x1b[2;31m●\x1b[0m"); // Dim red
                } else if temp_beat.is_medium_beat() {
                    print!("\x1b[2;33m◐\x1b[0m"); // Dim yellow
                } else {
                    print!("\x1b[2m○\x1b[0m"); // Dim
                }
            }
            
            if i < beats_per_measure {
                print!(" ");
            }
        }
        print!("] ");
        
        // Show beat strength description
        if beat.is_strong_beat() {
            print!("\x1b[1;31mSTRONG\x1b[0m ");
        } else if beat.is_medium_beat() {
            print!("\x1b[1;33mMEDIUM\x1b[0m ");
        } else {
            print!("\x1b[2mweak\x1b[0m ");
        }
        
        // Show beat count and measure info
        print!("Beat #{} (Measure {}, Beat {})", 
               beat.sequence_number,
               (beat.sequence_number - 1) / beats_per_measure as u64 + 1,
               beat.beat_in_measure);
        
        io::stdout().flush().unwrap();
    }
    
    /// Show a pulsing beat indicator that emphasizes strong/weak beats
    pub fn show_pulsing_beat_indicator(&self, beat: &Beat, pulse_phase: f32) {
        // Calculate pulse intensity based on beat strength and phase
        let base_strength = beat.get_accent_strength();
        let pulse_intensity = (pulse_phase * std::f32::consts::PI * 2.0).sin().abs();
        let combined_intensity = base_strength + (pulse_intensity * 0.3);
        
        // Choose symbol and color based on combined intensity
        let (symbol, color_code) = if combined_intensity >= 1.0 {
            ("●", "\x1b[1;31m") // Bold red
        } else if combined_intensity >= 0.7 {
            ("◐", "\x1b[1;33m") // Bold yellow
        } else if combined_intensity >= 0.3 {
            ("○", "\x1b[1;37m") // Bold white
        } else {
            ("·", "\x1b[2m")    // Dim
        };
        
        print!(" {}{}\x1b[0m ", color_code, symbol);
        io::stdout().flush().unwrap();
    }
    
    /// Show beat pattern demonstration for a given time signature
    pub fn demonstrate_beat_pattern(&self, time_signature: TimeSignature) {
        println!("\x1b[1mDemonstrating {} beat pattern:\x1b[0m", time_signature.as_str());
        
        let beats_per_measure = time_signature.beats_per_measure();
        
        // Show pattern multiple times for clarity
        for measure in 1..=2 {
            print!("Measure {}: ", measure);
            
            for beat_num in 1..=beats_per_measure {
                let temp_beat = crate::models::Beat::new(beat_num as u64, time_signature, 120);
                
                if temp_beat.is_strong_beat() {
                    print!("\x1b[1;31m●\x1b[0m(STRONG) ");
                } else if temp_beat.is_medium_beat() {
                    print!("\x1b[1;33m◐\x1b[0m(medium) ");
                } else {
                    print!("\x1b[2m○\x1b[0m(weak) ");
                }
            }
            println!();
        }
        println!();
    }
}

impl Default for DisplayEngine {
    fn default() -> Self {
        Self::new()
    }
}