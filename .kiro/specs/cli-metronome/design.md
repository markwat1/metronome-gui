# Design Document

## Overview

メトロノームアプリケーションは、Rustで実装されるクロスプラットフォーム対応のデュアルモードアプリケーションです。コマンドライン引数の有無により、CLI モードまたは GUI モードで動作します。正確なタイミング制御、クロスプラットフォーム音声再生、拍子設定、カスタムサウンド選択機能を提供し、直感的なユーザーインターフェースを両モードで実現します。

## Architecture

### High-Level Architecture

```
                    ┌─────────────────┐
                    │   App Launcher  │
                    └─────────┬───────┘
                              │
                    ┌─────────▼───────┐
                    │  Mode Detector  │
                    └─────┬─────┬─────┘
                          │     │
              ┌───────────▼─┐ ┌─▼──────────┐
              │  CLI Mode   │ │  GUI Mode  │
              └─────┬───────┘ └─┬──────────┘
                    │           │
    ┌───────────────▼───────────▼──────────────┐
    │           Metronome Core                 │
    └─────┬─────────────────────────────┬─────┘
          │                             │
    ┌─────▼─────┐                 ┌─────▼─────┐
    │Audio Engine│                 │GUI Engine │
    └───────────┘                 └───────────┘
```

### Technology Stack

- **言語**: Rust（クロスプラットフォーム対応、メモリ安全性、高性能）
- **CLI解析**: clap crate（引数解析とヘルプ生成）
- **GUI フレームワーク**: egui + eframe（軽量、クロスプラットフォーム GUI）
- **音声再生**: rodio crate（クロスプラットフォーム音声）
- **音声ファイル**: 内蔵音声データ（Click、Wood、Beep）+ カスタムファイル読み込み（WAV、MP3、OGG）
- **タイミング**: std::time（高精度タイマー）
- **信号処理**: ctrlc crate（CLI モード用 Ctrl+C 処理）
- **非同期処理**: tokio（GUI と音声の並行処理）
- **シリアライゼーション**: serde + serde_json（設定ファイル保存用）

## Components and Interfaces

### 1. App Launcher Module

**責任**: アプリケーションの起動とモード判定

```rust
pub enum AppMode {
    Cli(CliArgs),
    Gui,
}

pub fn determine_mode() -> AppMode
pub fn launch_app(mode: AppMode) -> Result<(), AppError>
```

**機能**:
- コマンドライン引数の有無によるモード判定
- 適切なモードでのアプリケーション起動

### 2. CLI Parser Module

**責任**: コマンドライン引数の解析とバリデーション

```rust
pub struct CliArgs {
    pub bpm: u32,
    pub help: bool,
}

pub fn parse_args() -> Result<Option<CliArgs>, CliError>
```

**機能**:
- BPM値のバリデーション（60-200範囲）
- ヘルプメッセージの生成
- 引数なしの場合の None 返却

### 3. Metronome Core Module

**責任**: メトロノームのメインロジックとタイミング制御

```rust
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

pub struct Metronome {
    state: Arc<Mutex<MetronomeState>>,
    audio_engine: Arc<AudioEngine>,
}

impl Metronome {
    pub fn new() -> Self
    pub fn set_bpm(&mut self, bpm: u32) -> Result<(), MetronomeError>
    pub fn set_time_signature(&mut self, sig: TimeSignature)
    pub fn set_sounds(&mut self, beat: SoundType, accent: SoundType)
    pub fn set_accent_enabled(&mut self, enabled: bool)
    pub fn set_volume(&mut self, volume: f32) -> Result<(), MetronomeError>
    pub fn start(&mut self) -> Result<(), MetronomeError>
    pub fn stop(&mut self)
    pub fn get_state(&self) -> MetronomeState
}
```

**機能**:
- BPMから拍間隔の計算
- 拍子に基づく強拍・弱拍の管理
- 正確なタイミング制御
- 状態管理とスレッドセーフなアクセス

### 4. Audio Engine Module

**責任**: クロスプラットフォーム音声再生と音声データ管理

```rust
#[derive(Debug, Clone)]
pub enum SoundType {
    BuiltinClick,
    BuiltinWood,
    BuiltinBeep,
    Custom(PathBuf),
}

#[derive(Debug, Clone)]
pub enum TimeSignature {
    One,    // 1/4 (no time signature)
    Two,    // 2/4
    Three,  // 3/4
    Four,   // 4/4
    Five,   // 5/8
    Six,    // 6/8
    Seven,  // 7/8
    Eight,  // 8/8
}

pub struct AudioEngine {
    sound_cache: HashMap<SoundType, Vec<u8>>,
    output_stream: Option<OutputStream>,
}

impl AudioEngine {
    pub fn new() -> Result<Self, AudioError>
    pub fn load_builtin_sounds(&mut self) -> Result<(), AudioError>
    pub fn load_custom_sound(&mut self, path: &Path) -> Result<SoundType, AudioError>
    pub fn play_sound(&self, sound_type: &SoundType) -> Result<(), AudioError>
    pub fn play_sound_with_volume(&self, sound_type: &SoundType, volume: f32) -> Result<(), AudioError>
    pub fn set_volume(&mut self, volume: f32)
    pub fn get_volume(&self) -> f32
    pub fn is_available(&self) -> bool
}
```

**機能**:
- 複数の内蔵音声データ管理
- カスタム音声ファイルの読み込み
- 音声キャッシュシステム
- プラットフォーム固有の音声システム対応

### 5. GUI Engine Module

**責任**: グラフィカルユーザーインターフェース

```rust
pub struct MetronomeApp {
    metronome: Arc<Mutex<Metronome>>,
    bpm_input: String,
    selected_time_signature: TimeSignature,
    selected_beat_sound: SoundType,
    selected_accent_sound: SoundType,
    error_message: Option<String>,
    last_beat_time: Option<Instant>,
}

impl eframe::App for MetronomeApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame)
}

impl MetronomeApp {
    pub fn new() -> Self
    fn render_controls(&mut self, ui: &mut egui::Ui)
    fn render_status(&mut self, ui: &mut egui::Ui)
    fn render_beat_indicator(&mut self, ui: &mut egui::Ui)
    fn render_accent_controls(&mut self, ui: &mut egui::Ui)
    fn render_volume_controls(&mut self, ui: &mut egui::Ui)
    fn handle_start_stop(&mut self)
    fn validate_bpm(&self, input: &str) -> Result<u32, String>
    fn test_sound(&mut self, sound_type: &SoundType)
    fn test_sound_with_volume(&mut self, sound_type: &SoundType, volume: f32)
}
```

**機能**:
- BPM入力フィールドと増減ボタン
- 拍子選択ドロップダウン
- 音声選択コントロール
- スタート・ストップボタン
- リアルタイム状態表示
- 視覚的ビートインジケーター

### 6. Display Engine Module (CLI Mode)

**責任**: コンソール出力とCLIユーザーインターフェース

```rust
pub struct DisplayEngine {
    start_time: Instant,
}

impl DisplayEngine {
    pub fn new() -> Self
    pub fn show_status(&self, state: &MetronomeState)
    pub fn show_beat_indicator(&self, beat: &Beat)
    pub fn show_enhanced_beat_indicator(&self, beat: &Beat)
    pub fn show_time_signature_legend(&self, time_signature: TimeSignature)
    pub fn show_realtime_beat_visualization(&self, beat: &Beat)
    pub fn clear_line(&self)
    pub fn show_help(&self)
}
```

**機能**:
- リアルタイム状態表示
- 視覚的拍インジケーター（強拍・弱拍区別）
- ヘルプメッセージ表示

## Data Models

### Configuration Model

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn default() -> Self
    pub fn validate(&self) -> Result<(), ConfigError>
    pub fn save_to_file(&self, path: &Path) -> Result<(), ConfigError>
    pub fn load_from_file(path: &Path) -> Result<Self, ConfigError>
}
```

### Beat Model

```rust
#[derive(Debug, Clone)]
pub struct Beat {
    pub timestamp: Instant,
    pub sequence_number: u64,
    pub beat_in_measure: u32,
    pub is_accent: bool,
    pub bpm: u32,
    pub time_signature: TimeSignature,
}

impl Beat {
    pub fn new(sequence: u64, time_sig: TimeSignature, bpm: u32) -> Self
    pub fn is_first_beat(&self) -> bool
}
```

### Sound Data Model

```rust
#[derive(Debug, Clone)]
pub struct SoundData {
    pub sound_type: SoundType,
    pub data: Vec<u8>,
    pub sample_rate: u32,
    pub channels: u16,
}

impl SoundData {
    pub fn from_builtin(sound_type: SoundType) -> Result<Self, AudioError>
    pub fn from_file(path: &Path) -> Result<Self, AudioError>
}
```

### GUI State Model

```rust
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
```

### Error Models

```rust
#[derive(Debug, thiserror::Error)]
pub enum MetronomeError {
    #[error("Invalid BPM value: {0}. Must be between 60 and 200")]
    InvalidBpm(u32),
    
    #[error("Audio system error: {0}")]
    AudioError(#[from] AudioError),
    
    #[error("CLI parsing error: {0}")]
    CliError(#[from] CliError),
    
    #[error("GUI error: {0}")]
    GuiError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),
}

#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("Failed to initialize audio system")]
    InitializationFailed,
    
    #[error("Failed to load sound file: {0}")]
    SoundLoadError(String),
    
    #[error("Failed to play sound")]
    PlaybackError,
    
    #[error("Unsupported audio format")]
    UnsupportedFormat,
}
```

## Error Handling

### Error Strategy

1. **入力バリデーション**: 早期エラー検出とユーザーフレンドリーなメッセージ
2. **音声エラー**: 音声が利用できない場合の視覚的フォールバック
3. **GUI エラー**: 非破壊的エラー表示とユーザー操作継続
4. **ファイルエラー**: カスタム音声ファイル読み込み失敗時の内蔵音声フォールバック
5. **システムエラー**: 適切なクリーンアップと終了処理
6. **信号処理**: CLI モードでの Ctrl+C 正常終了

### Error Recovery

- 音声システムエラー → 視覚的インジケーターのみで継続
- 無効なBPM → GUI: エラーメッセージ表示、CLI: 終了
- カスタム音声読み込み失敗 → 内蔵音声に自動フォールバック
- GUI 初期化失敗 → CLI モードで継続
- システム割り込み → リソースクリーンアップ後に正常終了

### GUI Error Handling

- **非破壊的エラー**: エラーメッセージを GUI 内に表示し、操作継続可能
- **入力検証**: リアルタイム入力検証とビジュアルフィードバック
- **状態保持**: エラー発生時も設定値を保持

## Testing Strategy

### Unit Testing

- **CLI Parser**: 引数解析とバリデーション、モード判定のテスト
- **Metronome Core**: タイミング計算、拍子管理、状態管理のテスト
- **Audio Engine**: モック音声システム、音声キャッシュ、ファイル読み込みのテスト
- **GUI Components**: UI コンポーネントの状態管理とイベント処理のテスト
- **Display Engine**: CLI 出力フォーマットのテスト

### Integration Testing

- **Dual Mode**: CLI/GUI 両モードでの動作テスト
- **Audio Integration**: 実際の音声再生とタイミング精度のテスト
- **GUI Integration**: ユーザー操作からメトロノーム制御までの統合テスト
- **Platform Testing**: Windows/Linux での動作確認
- **Error Scenarios**: 各種エラー条件でのテスト

### Performance Testing

- **Timing Accuracy**: 長時間実行での精度測定（CLI/GUI 両モード）
- **Resource Usage**: CPU/メモリ使用量の監視
- **Audio Latency**: 音声再生の遅延測定
- **GUI Responsiveness**: GUI の応答性とフレームレート測定

### Manual Testing

- **User Experience**: GUI の使いやすさとワークフローのテスト
- **Sound Quality**: 各種音声データの品質確認
- **Cross-Platform**: 実機での動作確認

## Implementation Notes

### Timing Precision

- `std::time::Instant`を使用した高精度タイミング
- スリープドリフト補正アルゴリズムの実装
- システムクロックの変更に対する耐性
- GUI モードでの非同期タイマー処理

### Cross-Platform Considerations

- **Windows**: WASAPI経由での音声再生、ネイティブ GUI
- **Linux**: ALSA/PulseAudio経由での音声再生、X11/Wayland 対応
- **共通**: rodio crate による音声抽象化、egui による GUI 抽象化

### GUI Architecture

- **Event-Driven**: egui のイベントループによる応答性確保
- **State Management**: Arc<Mutex<>> による安全な状態共有
- **Real-time Updates**: 60fps での GUI 更新とビートインジケーター
- **Resource Management**: 音声データのメモリ効率的なキャッシュ

### Audio System Design

- **Sound Caching**: 起動時の音声データプリロード
- **Low Latency**: 音声再生の遅延最小化
- **Format Support**: WAV, MP3, OGG 等の一般的フォーマット対応
- **Fallback Strategy**: 音声システム障害時の視覚的代替

### User Experience

#### CLI Mode
- 明確なエラーメッセージ
- リアルタイム状態表示
- 直感的なコマンドライン引数
- 適切な終了処理

#### GUI Mode
- 直感的なコントロール配置
- リアルタイムフィードバック
- 視覚的ビートインジケーター
- 設定の永続化（オプション）
- アクセシビリティ対応

### Performance Considerations

- **Memory Usage**: 音声データの効率的管理
- **CPU Usage**: タイマー処理の最適化
- **Battery Life**: モバイル環境での省電力動作
- **Startup Time**: アプリケーション起動の高速化