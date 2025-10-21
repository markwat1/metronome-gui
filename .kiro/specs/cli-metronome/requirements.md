# Requirements Document

## Introduction

メトロノームアプリケーションは、コマンドラインとGUIの両方のインターフェースを提供するメトロノームアプリケーションです。引数なしで実行した場合はGUIモードで起動し、BPMや拍子の設定、サウンドデータの選択、スタート・ストップ操作をグラフィカルに行えます。引数ありで実行した場合は従来のCLIモードで動作します。WindowsとLinuxの両方のプラットフォームで動作します。

## Glossary

- **Metronome_App**: コマンドラインとGUIの両方のインターフェースを持つメトロノームアプリケーション
- **CLI_Mode**: コマンドライン引数でBPMを指定して実行するモード
- **GUI_Mode**: グラフィカルユーザーインターフェースで操作するモード
- **BPM**: Beats Per Minute（1分間あたりの拍数）
- **Time_Signature**: 拍子（例：4拍子、3拍子）
- **Beat_Sound**: メトロノームが生成する通常の拍音
- **Accent_Sound**: 拍子の最初の拍で再生される強調音
- **Sound_Data**: メトロノーム音として使用する音声ファイルまたは音声データ
- **GUI_Window**: グラフィカルユーザーインターフェースのメインウィンドウ
- **Volume_Level**: 音声の音量レベル（0.0から1.0の範囲）

## Requirements

### Requirement 1

**User Story:** ミュージシャンとして、練習時に正確なテンポを保ちたいので、コマンドラインからBPMを指定してメトロノームを起動したい

#### Acceptance Criteria

1. WHEN ユーザーがBPMパラメータと共にMetronome_Appを実行するとき、THE Metronome_App SHALL CLI_Modeで指定されたBPMでBeat_Soundを再生する
2. THE Metronome_App SHALL CLI_Modeで60から200の範囲内のBPM値を受け入れる
3. IF CLI_Modeで無効なBPM値が提供された場合、THEN THE Metronome_App SHALL エラーメッセージを表示して終了する
4. THE Metronome_App SHALL WindowsとLinuxの両方のオペレーティングシステムで実行可能である
5. WHEN CLI_ModeでユーザーがCtrl+Cを押すとき、THE Metronome_App SHALL 正常に終了する

### Requirement 2

**User Story:** ユーザーとして、アプリケーションの使用方法を理解したいので、ヘルプ情報を表示できるようにしたい

#### Acceptance Criteria

1. WHEN ユーザーが--helpまたは-hフラグでMetronome_Appを実行するとき、THE Metronome_App SHALL 使用方法の説明を表示する
2. THE Metronome_App SHALL BPMパラメータの有効範囲を使用方法に含める
3. THE Metronome_App SHALL CLI_ModeとGUI_Modeの両方の使用例をヘルプメッセージに含める

### Requirement 3

**User Story:** ユーザーとして、CLI使用時にメトロノームの動作状況を把握したいので、現在のBPMと経過時間を表示したい

#### Acceptance Criteria

1. WHILE CLI_Modeで動作している間、THE Metronome_App SHALL 現在のBPM値を表示する
2. WHILE CLI_Modeで動作している間、THE Metronome_App SHALL 経過時間を表示する
3. THE Metronome_App SHALL CLI_Modeで拍ごとに視覚的なインジケーターを表示する
4. THE Metronome_App SHALL 強拍、中拍、弱拍を区別する色分けされた視覚的インジケーターを表示する
5. THE Metronome_App SHALL 拍子記号の説明とビートパターンの凡例を表示する
6. THE Metronome_App SHALL 小節内での現在の拍位置を表示する

### Requirement 4

**User Story:** 開発者として、異なるプラットフォームで一貫した動作を保証したいので、クロスプラットフォーム対応の音声再生機能を実装したい

#### Acceptance Criteria

1. THE Metronome_App SHALL Windowsプラットフォームで音声を再生する
2. THE Metronome_App SHALL Linuxプラットフォームで音声を再生する
3. THE Metronome_App SHALL 各プラットフォームのネイティブ音声システムを使用する
4. IF 音声システムが利用できない場合、THEN THE Metronome_App SHALL 代替の視覚的インジケーターを提供する

### Requirement 5

**User Story:** ユーザーとして、直感的にメトロノームを操作したいので、引数なしで実行した場合にGUIウィンドウが表示されるようにしたい

#### Acceptance Criteria

1. WHEN ユーザーが引数なしでMetronome_Appを実行するとき、THE Metronome_App SHALL GUI_Windowを表示する
2. THE GUI_Window SHALL WindowsとLinuxの両方のプラットフォームで表示される
3. THE GUI_Window SHALL ユーザーが閉じるまで開いたままである
4. WHEN ユーザーがGUI_Windowを閉じるとき、THE Metronome_App SHALL 正常に終了する

### Requirement 6

**User Story:** ユーザーとして、GUIでBPMを設定したいので、BPM入力フィールドとコントロールを提供してほしい

#### Acceptance Criteria

1. THE GUI_Window SHALL BPM値を入力するためのテキストフィールドを表示する
2. THE GUI_Window SHALL BPM値を増減するためのボタンを提供する
3. THE Metronome_App SHALL 60から200の範囲内のBPM値のみを受け入れる
4. WHEN 無効なBPM値が入力されたとき、THE Metronome_App SHALL エラーメッセージを表示する
5. THE GUI_Window SHALL 現在設定されているBPM値を常に表示する

### Requirement 7

**User Story:** ユーザーとして、拍子を設定したいので、拍子選択機能を提供してほしい

#### Acceptance Criteria

1. THE GUI_Window SHALL Time_Signatureを選択するためのドロップダウンメニューまたは選択コントロールを表示する
2. THE Metronome_App SHALL 1拍子、2拍子、3拍子、4拍子、5拍子、6拍子、7拍子、8拍子の設定をサポートする
3. WHEN Time_Signatureが設定されているとき、THE Metronome_App SHALL 強拍でAccent_Soundを再生する
4. WHEN Time_Signatureが設定されているとき、THE Metronome_App SHALL 中拍と弱拍でBeat_Soundを再生する
5. THE GUI_Window SHALL 現在設定されているTime_Signatureを表示する
6. THE Metronome_App SHALL 拍子に基づく3段階のアクセントパターン（強拍、中拍、弱拍）をサポートする

### Requirement 8

**User Story:** ユーザーとして、好みの音を使いたいので、サウンドデータを選択できる機能を提供してほしい

#### Acceptance Criteria

1. THE GUI_Window SHALL Beat_Soundを選択するためのドロップダウンメニューまたは選択コントロールを表示する
2. THE GUI_Window SHALL Accent_Soundを選択するためのドロップダウンメニューまたは選択コントロールを表示する
3. THE Metronome_App SHALL 複数の内蔵Sound_Dataオプション（Click、Wood、Beep）を提供する
4. THE Metronome_App SHALL カスタム音声ファイル（WAV、MP3、OGG形式）の読み込みをサポートする
5. THE GUI_Window SHALL 現在選択されているSound_Dataを表示する
6. THE GUI_Window SHALL アクセント機能の有効/無効を切り替えるコントロールを提供する
7. THE Metronome_App SHALL 各サウンドタイプのテスト再生機能を提供する

### Requirement 9

**User Story:** ユーザーとして、GUIでメトロノームを制御したいので、スタート・ストップボタンを提供してほしい

#### Acceptance Criteria

1. THE GUI_Window SHALL メトロノームを開始するためのスタートボタンを表示する
2. THE GUI_Window SHALL メトロノームを停止するためのストップボタンを表示する
3. WHEN スタートボタンが押されたとき、THE Metronome_App SHALL 設定されたBPMとTime_Signatureでメトロノームを開始する
4. WHEN ストップボタンが押されたとき、THE Metronome_App SHALL メトロノームを停止する
5. WHILE メトロノームが動作している間、THE GUI_Window SHALL 視覚的なビートインジケーターを表示する

### Requirement 10

**User Story:** ユーザーとして、メトロノームの状態を把握したいので、GUI上で現在の状態と経過時間を確認したい

#### Acceptance Criteria

1. WHILE メトロノームが動作している間、THE GUI_Window SHALL 現在のBPM値を表示する
2. WHILE メトロノームが動作している間、THE GUI_Window SHALL 経過時間を表示する
3. WHILE メトロノームが動作している間、THE GUI_Window SHALL 現在の拍数を表示する
4. THE GUI_Window SHALL メトロノームの動作状態（停止中/動作中）を明確に表示する
5. THE GUI_Window SHALL 強拍、中拍、弱拍を区別する色分けされたビート可視化を提供する
6. THE GUI_Window SHALL 小節内での拍の進行状況を視覚的に表示する

### Requirement 11

**User Story:** ユーザーとして、より多様な音楽スタイルに対応したいので、拡張された拍子記号サポートを利用したい

#### Acceptance Criteria

1. THE Metronome_App SHALL 1/4、2/4、3/4、4/4、5/8、6/8、7/8、8/8の拍子記号をサポートする
2. THE Metronome_App SHALL 各拍子記号に適切なアクセントパターンを適用する
3. THE Metronome_App SHALL 複合拍子（6/8、7/8、8/8）での適切なビート間隔を計算する
4. THE GUI_Window SHALL 各拍子記号の小節あたりの拍数を表示する
5. THE CLI_Mode SHALL 各拍子記号のビートパターン説明を提供する

### Requirement 12

**User Story:** ユーザーとして、アクセントの使用を制御したいので、アクセント機能の有効/無効を切り替えたい

#### Acceptance Criteria

1. THE GUI_Window SHALL アクセント機能を有効/無効にするチェックボックスを提供する
2. WHEN アクセント機能が無効のとき、THE Metronome_App SHALL すべての拍で同じBeat_Soundを使用する
3. WHEN アクセント機能が有効のとき、THE Metronome_App SHALL 強拍でAccent_Soundを使用する
4. THE Metronome_App SHALL アクセント設定の変更を即座に反映する
5. THE GUI_Window SHALL 現在のアクセント設定状態を明確に表示する

### Requirement 13

**User Story:** ユーザーとして、練習環境に応じて音量を調節したいので、GUI上で音量を制御できるようにしたい

#### Acceptance Criteria

1. THE GUI_Window SHALL 音量を調節するためのスライダーコントロールを表示する
2. THE Metronome_App SHALL 0.0から1.0の範囲でVolume_Levelを受け入れる
3. WHEN Volume_Levelが変更されたとき、THE Metronome_App SHALL 新しい音量レベルを即座に適用する
4. THE GUI_Window SHALL 現在のVolume_Level値をパーセンテージで表示する
5. THE Metronome_App SHALL Volume_Level設定をBeat_SoundとAccent_Soundの両方に適用する
6. THE GUI_Window SHALL 音量テスト機能を提供する（現在の音量レベルでサウンドを再生）
7. THE Metronome_App SHALL 音量設定を設定ファイルに保存し、次回起動時に復元する