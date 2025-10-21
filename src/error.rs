use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetronomeError {
    #[error("Invalid BPM value: {0}. Must be between 60 and 200")]
    InvalidBpm(u32),
    
    #[error("Invalid volume value: {0}. Must be between 0.0 and 1.0")]
    InvalidVolume(f32),
    
    #[error("Audio system error: {0}")]
    AudioError(#[from] AudioError),
    
    #[error("CLI parsing error: {0}")]
    CliError(#[from] CliError),
    
    #[error("GUI error: {0}")]
    GuiError(#[from] GuiError),
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),
    
    #[error("System error: {0}")]
    SystemError(String),
}

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("Audio device not available")]
    DeviceNotAvailable,
    
    #[error("Audio initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("Audio playback failed: {0}")]
    PlaybackFailed(String),
    
    #[error("Audio format not supported: {0}")]
    UnsupportedFormat(String),
    
    #[error("Failed to load sound file: {0}")]
    SoundLoadError(String),
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Missing required argument: {0}")]
    MissingArgument(String),
    
    #[error("Invalid argument value: {0}")]
    InvalidArgument(String),
    
    #[error("Argument parsing failed: {0}")]
    ParsingFailed(String),
}

#[derive(Debug, Error)]
pub enum GuiError {
    #[error("GUI initialization failed: {0}")]
    InitializationFailed(String),
    
    #[error("GUI rendering error: {0}")]
    RenderingError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("File dialog error: {0}")]
    FileDialogError(String),
    
    #[error("GUI state error: {0}")]
    StateError(String),
    
    #[error("Widget error: {0}")]
    WidgetError(String),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Configuration file not found: {0}")]
    FileNotFound(String),
    
    #[error("Configuration file read error: {0}")]
    ReadError(String),
    
    #[error("Configuration file write error: {0}")]
    WriteError(String),
    
    #[error("Configuration parsing error: {0}")]
    ParseError(String),
    
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
    
    #[error("Configuration validation error: {0}")]
    ValidationError(String),
}

pub type Result<T> = std::result::Result<T, MetronomeError>;