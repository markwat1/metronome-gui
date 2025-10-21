// GUI application implementation using egui/eframe

use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::metronome::Metronome;
use crate::models::{GuiState, TimeSignature, SoundType};

/// Main GUI application structure
pub struct MetronomeApp {
    metronome: Arc<Mutex<Metronome>>,
    gui_state: GuiState,
    audio_engine: Option<Arc<crate::audio::CrossPlatformAudio>>,
    last_beat_time: Option<Instant>,
}

impl MetronomeApp {
    pub fn new() -> Self {
        let metronome = Arc::new(Mutex::new(Metronome::new()));
        let gui_state = GuiState::new();
        
        // Initialize audio engine
        let audio_engine = {
            let mut audio = crate::audio::CrossPlatformAudio::new();
            match audio.initialize() {
                Ok(()) => {
                    Some(Arc::new(audio))
                }
                Err(e) => {
                    eprintln!("Warning: Audio initialization failed: {}", e);
                    None
                }
            }
        };
        
        Self {
            metronome,
            gui_state,
            audio_engine,
            last_beat_time: None,
        }
    }
    
    /// Adjust BPM by the given delta and update the input field
    fn adjust_bpm(&mut self, delta: i32) {
        if let Ok(metronome) = self.metronome.lock() {
            let current_bpm = metronome.get_bpm() as i32;
            let new_bpm = (current_bpm + delta).max(60).min(200) as u32;
            
            // Update the input field
            self.gui_state.bpm_input = new_bpm.to_string();
            
            // Apply the new BPM
            if let Err(e) = metronome.set_bpm(new_bpm) {
                self.gui_state.set_error(e.to_string());
            } else {
                self.gui_state.clear_error();
                self.gui_state.bpm_valid = true;
            }
        }
    }
    
    /// Test a sound by playing it once
    fn test_sound(&mut self, sound_type: &SoundType) {
        if let Some(audio_engine) = &self.audio_engine {
            if let Err(e) = audio_engine.play_sound(sound_type) {
                self.gui_state.set_error(format!("Failed to play sound: {}", e));
            }
        } else {
            // Visual feedback when audio is not available
            self.gui_state.update_beat_visual();
        }
    }
    
    /// Start the metronome
    fn start_metronome(&mut self) {
        if let Ok(metronome) = self.metronome.lock() {
            if let Err(e) = metronome.start() {
                self.gui_state.set_error(format!("Failed to start metronome: {}", e));
            } else {
                self.gui_state.clear_error();
                self.gui_state.is_running = true;
                self.last_beat_time = Some(Instant::now());
            }
        }
    }
    
    /// Stop the metronome
    fn stop_metronome(&mut self) {
        if let Ok(metronome) = self.metronome.lock() {
            metronome.stop();
            self.gui_state.is_running = false;
            self.last_beat_time = None;
        }
    }
    
    /// Reset the metronome (stop and reset beat count)
    fn reset_metronome(&mut self) {
        if let Ok(metronome) = self.metronome.lock() {
            metronome.stop();
            metronome.reset_beat_position();
            self.gui_state.is_running = false;
            self.last_beat_time = None;
            self.gui_state.clear_error();
        }
    }
    
    /// Handle metronome beat timing and audio playback
    fn handle_metronome_beats(&mut self) {
        if let Ok(metronome) = self.metronome.lock() {
            if !metronome.is_running() {
                return;
            }
            
            if let Some(last_beat) = self.last_beat_time {
                if metronome.should_play_beat(last_beat) {
                    // Play the beat
                    let beat = metronome.increment_beat();
                    // Use accent sound for strong beats (strength >= 1.0) only
                    let sound_type = if beat.get_accent_strength() >= 1.0 {
                        &self.gui_state.selected_accent_sound
                    } else {
                        &self.gui_state.selected_beat_sound
                    };
                    
                    // Play audio if available
                    if let Some(audio_engine) = &self.audio_engine {
                        if let Err(e) = audio_engine.play_sound(sound_type) {
                            eprintln!("Audio playback error: {}", e);
                        }
                    }
                    
                    // Update visual beat indicator
                    self.gui_state.update_beat_visual();
                    self.last_beat_time = Some(Instant::now());
                }
            }
        }
    }
}

#[cfg(feature = "gui")]
impl eframe::App for MetronomeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle metronome beats
        self.handle_metronome_beats();
        
        // Set up the main window
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Metronome");
            
            ui.separator();
            
            // BPM Controls Section
            ui.group(|ui| {
                ui.label("BPM Settings");
                
                ui.horizontal(|ui| {
                    // BPM input field
                    ui.label("BPM:");
                    let response = ui.text_edit_singleline(&mut self.gui_state.bpm_input);
                    
                    // Validate BPM input on change
                    if response.changed() {
                        let input_clone = self.gui_state.bpm_input.clone();
                        let _ = self.gui_state.validate_bpm(&input_clone);
                    }
                    
                    // Apply BPM button
                    if ui.button("Apply").clicked() {
                        let input_clone = self.gui_state.bpm_input.clone();
                        let validation_result = self.gui_state.validate_bpm(&input_clone);
                        if let Some(bpm) = self.gui_state.handle_result(validation_result) {
                            if let Ok(metronome) = self.metronome.lock() {
                                if let Err(e) = metronome.set_bpm(bpm) {
                                    self.gui_state.set_error(e.to_string());
                                }
                            }
                        }
                    }
                });
                
                ui.horizontal(|ui| {
                    // BPM decrease button
                    if ui.button("-10").clicked() {
                        self.adjust_bpm(-10);
                    }
                    if ui.button("-1").clicked() {
                        self.adjust_bpm(-1);
                    }
                    
                    // Current BPM display
                    if let Ok(metronome) = self.metronome.lock() {
                        ui.label(format!("Current: {}", metronome.get_bpm()));
                    }
                    
                    // BPM increase buttons
                    if ui.button("+1").clicked() {
                        self.adjust_bpm(1);
                    }
                    if ui.button("+10").clicked() {
                        self.adjust_bpm(10);
                    }
                });
                
                // BPM validation indicator
                if !self.gui_state.bpm_valid {
                    ui.colored_label(egui::Color32::RED, "Invalid BPM (must be 60-200)");
                }
            });
            
            ui.separator();
            
            // Time Signature Controls Section
            ui.group(|ui| {
                ui.label("Time Signature");
                
                ui.horizontal(|ui| {
                    ui.label("Time Signature:");
                    
                    // Time signature dropdown
                    egui::ComboBox::from_label("")
                        .selected_text(self.gui_state.selected_time_signature.as_str())
                        .show_ui(ui, |ui| {
                            for &time_sig in TimeSignature::all() {
                                let selected = ui.selectable_value(
                                    &mut self.gui_state.selected_time_signature,
                                    time_sig,
                                    time_sig.as_str()
                                );
                                
                                // Apply time signature change immediately
                                if selected.clicked() {
                                    if let Ok(metronome) = self.metronome.lock() {
                                        metronome.set_time_signature(time_sig);
                                        // Reset beat position when changing time signature
                                        metronome.reset_beat_position();
                                    }
                                }
                            }
                        });
                    
                    // Current time signature display
                    if let Ok(metronome) = self.metronome.lock() {
                        let current_time_sig = metronome.get_time_signature();
                        ui.label(format!("Current: {}", current_time_sig.as_str()));
                        ui.label(format!("({} beats per measure)", current_time_sig.beats_per_measure()));
                    }
                });
            });
            
            ui.separator();
            
            // Sound Selection Controls Section
            ui.group(|ui| {
                ui.label("Sound Settings");
                
                ui.horizontal(|ui| {
                    ui.label("Beat Sound:");
                    
                    // Beat sound dropdown
                    egui::ComboBox::from_label("beat_sound")
                        .selected_text(self.gui_state.selected_beat_sound.as_str())
                        .show_ui(ui, |ui| {
                            for sound_type in SoundType::builtin_sounds() {
                                let selected = ui.selectable_value(
                                    &mut self.gui_state.selected_beat_sound,
                                    sound_type.clone(),
                                    sound_type.as_str()
                                );
                                
                                // Apply sound change immediately
                                if selected.clicked() {
                                    if let Ok(metronome) = self.metronome.lock() {
                                        metronome.set_sounds(
                                            self.gui_state.selected_beat_sound.clone(),
                                            self.gui_state.selected_accent_sound.clone()
                                        );
                                    }
                                }
                            }
                        });
                    
                    // Test beat sound button
                    if ui.button("Test").clicked() {
                        self.test_sound(&self.gui_state.selected_beat_sound.clone());
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Accent Sound:");
                    
                    // Accent sound dropdown
                    egui::ComboBox::from_label("accent_sound")
                        .selected_text(self.gui_state.selected_accent_sound.as_str())
                        .show_ui(ui, |ui| {
                            for sound_type in SoundType::builtin_sounds() {
                                let selected = ui.selectable_value(
                                    &mut self.gui_state.selected_accent_sound,
                                    sound_type.clone(),
                                    sound_type.as_str()
                                );
                                
                                // Apply sound change immediately
                                if selected.clicked() {
                                    if let Ok(metronome) = self.metronome.lock() {
                                        metronome.set_sounds(
                                            self.gui_state.selected_beat_sound.clone(),
                                            self.gui_state.selected_accent_sound.clone()
                                        );
                                    }
                                }
                            }
                        });
                    
                    // Test accent sound button
                    if ui.button("Test").clicked() {
                        self.test_sound(&self.gui_state.selected_accent_sound.clone());
                    }
                });
                
                // Custom sound file selection (placeholder for now)
                ui.horizontal(|ui| {
                    ui.label("Custom Sound:");
                    if ui.button("Load Custom Sound...").clicked() {
                        // TODO: Implement file dialog for custom sound selection
                        // This would require a file dialog crate like rfd
                        self.gui_state.set_error("Custom sound loading not yet implemented".to_string());
                    }
                });
                
                // Audio status display
                if let Some(_audio_engine) = &self.audio_engine {
                    ui.label("Audio Status: Available");
                } else {
                    ui.colored_label(egui::Color32::YELLOW, "Audio Status: Visual-only mode");
                }
            });
            
            ui.separator();
            
            // Start/Stop Controls Section
            ui.group(|ui| {
                ui.label("Metronome Control");
                
                ui.horizontal(|ui| {
                    let is_running = if let Ok(metronome) = self.metronome.lock() {
                        metronome.is_running()
                    } else {
                        false
                    };
                    
                    if is_running {
                        // Stop button
                        if ui.button("⏹ Stop").clicked() {
                            self.stop_metronome();
                        }
                        
                        // Visual status indicator
                        ui.colored_label(egui::Color32::GREEN, "● Running");
                    } else {
                        // Start button
                        if ui.button("▶ Start").clicked() {
                            self.start_metronome();
                        }
                        
                        // Visual status indicator
                        ui.colored_label(egui::Color32::RED, "● Stopped");
                    }
                });
                
                // Reset button
                if ui.button("🔄 Reset").clicked() {
                    self.reset_metronome();
                }
            });
            
            ui.separator();
            
            // Status Display and Beat Indicator Section
            ui.group(|ui| {
                ui.label("Status Display");
                
                if let Ok(metronome) = self.metronome.lock() {
                    let state = metronome.get_state();
                    
                    // Current settings display
                    ui.horizontal(|ui| {
                        ui.label(format!("BPM: {}", state.bpm));
                        ui.separator();
                        ui.label(format!("Time Signature: {}", state.time_signature.as_str()));
                        ui.separator();
                        ui.label(format!("Status: {}", if state.is_running { "Running" } else { "Stopped" }));
                    });
                    
                    // Beat count and timing display
                    ui.horizontal(|ui| {
                        ui.label(format!("Total Beats: {}", state.beat_count));
                        ui.separator();
                        ui.label(format!("Current Beat: {}/{}", 
                            state.current_beat_in_measure, 
                            state.time_signature.beats_per_measure()));
                        ui.separator();
                        
                        // Elapsed time display
                        let elapsed = state.get_elapsed_time();
                        let minutes = elapsed.as_secs() / 60;
                        let seconds = elapsed.as_secs() % 60;
                        ui.label(format!("Time: {:02}:{:02}", minutes, seconds));
                    });
                    
                    // Visual beat indicator
                    ui.horizontal(|ui| {
                        ui.label("Beat Indicator:");
                        
                        // Show beat pattern for current time signature
                        let beats_per_measure = state.time_signature.beats_per_measure();
                        for beat_num in 1..=beats_per_measure {
                            let is_current_beat = beat_num == state.current_beat_in_measure;
                            // Create a temporary beat to check accent strength
                            let temp_beat = crate::models::Beat::new(beat_num as u64, state.time_signature, state.bpm);
                            let is_strong_accent = temp_beat.get_accent_strength() >= 1.0;
                            let is_medium_accent = temp_beat.get_accent_strength() > 0.0 && temp_beat.get_accent_strength() < 1.0;
                            
                            // Visual beat indicator with different colors
                            let (color, symbol) = if is_current_beat {
                                if self.gui_state.should_show_beat_visual(std::time::Duration::from_millis(100)) {
                                    if is_strong_accent {
                                        (egui::Color32::RED, "●")  // Current strong accent beat - red
                                    } else if is_medium_accent {
                                        (egui::Color32::from_rgb(255, 165, 0), "●") // Current medium accent beat - orange
                                    } else {
                                        (egui::Color32::GREEN, "●") // Current regular beat - green
                                    }
                                } else if is_strong_accent {
                                    (egui::Color32::DARK_RED, "○") // Strong accent position - dark red
                                } else if is_medium_accent {
                                    (egui::Color32::from_rgb(200, 100, 0), "○") // Medium accent position - dark orange
                                } else {
                                    (egui::Color32::GRAY, "○") // Regular position - gray
                                }
                            } else if is_strong_accent {
                                (egui::Color32::DARK_RED, "○") // Strong accent position - dark red
                            } else if is_medium_accent {
                                (egui::Color32::from_rgb(200, 100, 0), "○") // Medium accent position - dark orange
                            } else {
                                (egui::Color32::GRAY, "○") // Regular position - gray
                            };
                            
                            ui.colored_label(color, symbol);
                        }
                        
                        // Beat strength indicator
                        if state.is_running && state.beat_count > 0 {
                            let current_beat = crate::models::Beat::new(
                                state.beat_count, 
                                state.time_signature, 
                                state.bpm
                            );
                            let strength = current_beat.get_accent_strength();
                            
                            ui.separator();
                            if strength >= 1.0 {
                                ui.colored_label(egui::Color32::RED, "Strong Beat");
                            } else if strength > 0.0 {
                                ui.colored_label(egui::Color32::YELLOW, "Medium Beat");
                            } else {
                                ui.colored_label(egui::Color32::GRAY, "Weak Beat");
                            }
                        }
                    });
                    
                    // Tempo indicator (visual metronome)
                    if state.is_running {
                        ui.horizontal(|ui| {
                            ui.label("Tempo:");
                            
                            // Large visual beat indicator
                            let beat_visual_active = self.gui_state.should_show_beat_visual(
                                std::time::Duration::from_millis(150)
                            );
                            
                            if beat_visual_active {
                                // Flash effect for beat
                                // Create a temporary beat to check accent strength
                                let temp_beat = crate::models::Beat::new(state.current_beat_in_measure as u64, state.time_signature, state.bpm);
                                let accent_strength = temp_beat.get_accent_strength();
                                let color = if accent_strength >= 1.0 {
                                    egui::Color32::from_rgb(255, 100, 100) // Light red for strong accent
                                } else if accent_strength > 0.0 {
                                    egui::Color32::from_rgb(255, 165, 100) // Light orange for medium accent
                                } else {
                                    egui::Color32::from_rgb(100, 255, 100) // Light green for regular
                                };
                                
                                ui.colored_label(color, "⬤");
                            } else {
                                ui.colored_label(egui::Color32::DARK_GRAY, "○");
                            }
                            
                            // BPM indicator bars
                            let interval_ms = (60000.0 / state.bpm as f32) as u64;
                            let progress = if let Some(start_time) = state.start_time {
                                let elapsed_ms = start_time.elapsed().as_millis() as u64;
                                (elapsed_ms % interval_ms) as f32 / interval_ms as f32
                            } else {
                                0.0
                            };
                            
                            ui.add(egui::ProgressBar::new(progress).desired_width(100.0));
                        });
                    }
                }
            });
            
            ui.separator();
            
            // Show error message if any
            if let Some(error) = &self.gui_state.error_message {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            }
        });
        
        // Request repaint for smooth updates
        ctx.request_repaint();
    }
}