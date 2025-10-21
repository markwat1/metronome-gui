// GUI module for the metronome application
// Provides graphical user interface using egui/eframe

#[cfg(feature = "gui")]
pub mod app;

#[cfg(feature = "gui")]
pub use app::MetronomeApp;