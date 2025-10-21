# Implementation Plan

- [x] 1. プロジェクト構造と GUI 依存関係の追加

  - GUI 関連クレート（egui、eframe、tokio）を Cargo.toml に追加
  - モジュール構造の拡張（gui モジュールの追加）
  - 既存の CLI 機能との互換性確保
  - _Requirements: 5.1, 5.2_

- [x] 2. データモデルの拡張と GUI エラーハンドリング

  - [x] 2.1 拍子とサウンドタイプの実装

    - TimeSignature enum（2/4, 3/4, 4/4, 6/8）の定義
    - SoundType enum（内蔵音声とカスタム音声）の実装
    - _Requirements: 7.2, 7.3, 8.2, 8.3_

  - [x] 2.2 MetronomeState と GUI 状態モデルの実装

    - MetronomeState 構造体の拡張（拍子、音声設定を含む）
    - GuiState 構造体の実装
    - スレッドセーフな状態管理（Arc<Mutex<>>）の実装
    - _Requirements: 6.5, 7.5, 9.5, 10.4_

  - [x] 2.3 GUI 用エラー型の追加
    - GuiError、ConfigError の定義
    - 既存エラー型の拡張
    - _Requirements: 6.4, 8.4_

- [x] 3. アプリケーションランチャーとモード判定の実装

  - [x] 3.1 AppMode とモード判定ロジック

    - AppMode enum（Cli、Gui）の定義
    - コマンドライン引数の有無によるモード判定
    - _Requirements: 5.1, 5.3_

  - [x] 3.2 CLI 解析の更新
    - 既存の CLI 解析機能の保持
    - 引数なしの場合の None 返却対応
    - ヘルプメッセージに GUI モードの説明追加
    - _Requirements: 1.1, 1.2, 2.3_

- [x] 4. メトロノームコアロジックの拡張

  - [x] 4.1 拍子対応の Metronome 構造体の実装

    - 既存の Metronome 構造体を拡張
    - 拍子に基づく強拍・弱拍の管理
    - current_beat_in_measure の追跡
    - _Requirements: 7.3, 7.4, 10.3_

  - [x] 4.2 音声設定とスレッドセーフな制御

    - beat_sound と accent_sound の設定機能
    - Arc<Mutex<MetronomeState>>による状態管理
    - 非同期での start/stop 制御
    - _Requirements: 8.2, 8.3, 9.3, 9.4_

  - [x] 4.3 拍子別タイミング計算
    - 各拍子での拍間隔計算
    - 強拍判定ロジック
    - _Requirements: 7.3, 7.4_

- [x] 5. 音声エンジンの拡張

  - [x] 5.1 複数音声データとキャッシュシステム

    - 内蔵音声データ（Click、Wood、Beep）の実装
    - SoundData モデルとキャッシュシステム
    - 音声データのプリロード機能
    - _Requirements: 8.3, 8.5_

  - [x] 5.2 カスタム音声ファイル読み込み

    - WAV、MP3、OGG 形式のサポート
    - ファイル読み込みとバリデーション
    - エラー時の内蔵音声フォールバック
    - _Requirements: 8.4_

  - [x] 5.3 強拍・弱拍音声再生
    - beat_sound と accent_sound の区別再生
    - 音声タイプに基づく再生制御
    - _Requirements: 7.4, 8.2, 8.3_

- [x] 6. GUI エンジンの実装

  - [x] 6.1 MetronomeApp と eframe 統合

    - MetronomeApp 構造体の実装
    - eframe::App トレイトの実装
    - 基本的な GUI ウィンドウの表示
    - _Requirements: 5.1, 5.2, 5.4_

  - [x] 6.2 BPM 設定コントロール

    - BPM 入力フィールドの実装
    - BPM 増減ボタンの実装
    - リアルタイム入力バリデーション
    - _Requirements: 6.1, 6.2, 6.3, 6.4, 6.5_

  - [x] 6.3 拍子選択コントロール

    - 拍子選択ドロップダウンの実装
    - 現在の拍子表示
    - _Requirements: 7.1, 7.2, 7.5_

  - [x] 6.4 音声選択コントロール

    - beat_sound 選択ドロップダウン
    - accent_sound 選択ドロップダウン
    - カスタム音声ファイル選択ダイアログ
    - _Requirements: 8.1, 8.2, 8.5_

  - [x] 6.5 スタート・ストップ制御

    - スタート・ストップボタンの実装
    - メトロノーム状態の視覚的表示
    - _Requirements: 9.1, 9.2, 9.3, 9.4_

  - [x] 6.6 状態表示とビートインジケーター

    - 現在の BPM、経過時間、拍数表示
    - 視覚的ビートインジケーター
    - 動作状態の表示
    - _Requirements: 9.5, 10.1, 10.2, 10.3, 10.4_

- [ ] 7. CLI ディスプレイエンジンの拡張

  - [x] 7.1 強拍・弱拍対応の視覚的インジケーター

    - 既存の DisplayEngine を拡張
    - 強拍・弱拍の区別表示
    - 拍子情報の表示
    - _Requirements: 3.3, 7.4, 7.5_

- [x] 8. デュアルモードアプリケーションの統合

  - [x] 8.1 main 関数のモード分岐実装

    - モード判定に基づく CLI/GUI 起動
    - 共通初期化処理の実装
    - _Requirements: 5.1, 5.3_

  - [x] 8.2 CLI モードの実行フロー

    - 既存の CLI 機能の保持
    - 拡張されたメトロノームコアとの統合
    - _Requirements: 1.1, 1.5, 3.3_

  - [x] 8.3 GUI モードの実行フロー

    - eframe アプリケーションの起動
    - GUI 初期化エラーハンドリング
    - _Requirements: 5.1, 5.2, 5.4_

- [x] 9. テストとドキュメント

  - [x] 9.1 GUI 機能の単体テスト

    - GUI 状態管理のテスト
    - コントロール機能のテスト
    - 入力バリデーションのテスト
    - _Requirements: 6.4, 7.2, 8.4_

  - [x] 9.2 拡張された音声エンジンのテスト

    - 複数音声データのテスト
    - カスタム音声読み込みのテスト
    - 強拍・弱拍再生のテスト
    - _Requirements: 7.4, 8.2, 8.3, 8.4_

  - [x] 9.3 デュアルモード統合テスト

    - CLI/GUI モード切り替えのテスト
    - 両モードでの機能一貫性テスト
    - _Requirements: 5.1, 5.3_

  - [x] 9.4 ドキュメント更新

    - README に GUI 機能の説明追加
    - 使用方法とスクリーンショット
    - 新機能（拍子、カスタム音声）の説明
    - _Requirements: 2.3, 5.1, 7.1, 8.1_
