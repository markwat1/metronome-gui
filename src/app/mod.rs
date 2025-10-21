// Application launcher and mode detection module

pub mod launcher;

pub use launcher::{AppMode, determine_mode, launch_app};