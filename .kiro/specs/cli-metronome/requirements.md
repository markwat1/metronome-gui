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
2. THE Metronome_App SHALL 2拍子、3拍子、4拍子、6拍子の設定をサポートする
3. WHEN Time_Signatureが設定されているとき、THE Metronome_App SHALL 拍子の最初の拍でAccent_Soundを再生する
4. WHEN Time_Signatureが設定されているとき、THE Metronome_App SHALL 他の拍でBeat_Soundを再生する
5. THE GUI_Window SHALL 現在設定されているTime_Signatureを表示する

### Requirement 8

**User Story:** ユーザーとして、好みの音を使いたいので、サウンドデータを選択できる機能を提供してほしい

#### Acceptance Criteria

1. THE GUI_Window SHALL Beat_Soundを選択するためのドロップダウンメニューまたは選択コントロールを表示する
2. THE GUI_Window SHALL Accent_Soundを選択するためのドロップダウンメニューまたは選択コントロールを表示する
3. THE Metronome_App SHALL 複数の内蔵Sound_Dataオプションを提供する
4. THE Metronome_App SHALL カスタム音声ファイルの読み込みをサポートする
5. THE GUI_Window SHALL 現在選択されているSound_Dataを表示する

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