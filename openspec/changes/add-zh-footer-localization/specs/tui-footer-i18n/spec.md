## ADDED Requirements

### Requirement: Footer hints support Chinese localization
The TUI footer SHALL render translated hint text, mode labels, and context usage text when Chinese locale is active.

#### Scenario: Chinese locale shows localized footer strings
- **GIVEN** Chinese locale is active in the TUI session
- **WHEN** the footer renders shortcut hints and context indicators
- **THEN** footer copy is shown in Chinese

#### Scenario: English locale preserves existing footer strings
- **GIVEN** English locale is active in the TUI session
- **WHEN** the footer renders shortcut hints and context indicators
- **THEN** existing English footer copy remains unchanged

### Requirement: Slash popup and startup hints support Chinese localization
The TUI SHALL render translated descriptions for slash-command popup entries and key startup hint text when Chinese locale is active.

#### Scenario: Chinese locale shows translated slash-command descriptions
- **GIVEN** Chinese locale is active in the TUI session
- **WHEN** the user opens the slash-command popup with `/`
- **THEN** slash-command descriptions are shown in Chinese

#### Scenario: Chinese locale shows translated startup hints
- **GIVEN** Chinese locale is active in the TUI session
- **WHEN** the startup tooltip and session header are rendered
- **THEN** key hint text (including model-change hint) is shown in Chinese

### Requirement: Selection popups and trust onboarding support Chinese localization
The TUI SHALL render translated copy for key secondary selection menus, shared popup hints, and trust onboarding text when Chinese locale is active.

#### Scenario: Chinese locale shows localized secondary menu copy
- **GIVEN** Chinese locale is active in the TUI session
- **WHEN** the user opens model, reasoning, collaboration, review, or skills secondary menus
- **THEN** popup titles, subtitles, search placeholders, and hint text are shown in Chinese

#### Scenario: Trust onboarding resolves translated strings instead of raw keys
- **GIVEN** trust onboarding is rendered in the TUI
- **WHEN** locale files are loaded
- **THEN** trust prompt text shows translated strings (not unresolved locale key names)

### Requirement: Composer placeholder and running status hints support Chinese localization
The TUI SHALL render translated placeholder suggestions and running status interrupt hints when Chinese locale is active.

#### Scenario: Chinese locale shows translated placeholder suggestions
- **GIVEN** Chinese locale is active in the TUI session
- **WHEN** the composer displays a random placeholder suggestion
- **THEN** placeholder copy is shown in Chinese

#### Scenario: Chinese locale shows translated running status hints
- **GIVEN** Chinese locale is active while a task is running
- **WHEN** the status indicator row is rendered
- **THEN** the running header and interrupt hint copy are shown in Chinese

### Requirement: Session picker, feedback, and onboarding auth flows support Chinese localization
The TUI SHALL render translated copy in agent/session picker flows, feedback-related popups, request-user-input overlay hints, and onboarding auth/provider selection prompts when Chinese locale is active.

#### Scenario: Chinese locale shows translated picker and popup empty states
- **GIVEN** Chinese locale is active in the TUI session
- **WHEN** agent/session pickers and related popups render search placeholders, empty states, or footer hints
- **THEN** those hints and empty-state strings are shown in Chinese

#### Scenario: Chinese locale shows translated onboarding auth and OSS selector prompts
- **GIVEN** Chinese locale is active during onboarding
- **WHEN** auth mode selection, device-code login prompts, and OSS provider selection UI are rendered
- **THEN** prompt copy and key action hints are shown in Chinese

#### Scenario: English locale preserves existing auth/picker/popup copy
- **GIVEN** English locale is active in the TUI session
- **WHEN** the same picker, popup, and onboarding auth flows are rendered
- **THEN** existing English strings remain unchanged
