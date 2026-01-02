# Changelog

## [0.2.1] - 2026-01-02

### Added
- **Font Management System** (LilButter's modification)
  - Automatic MS Gothic.ttf registration for Wine compatibility
  - Multi-path font resolution (backend/Font/ and Font/)
  - Custom font name configuration support

### Changed
- **Friends Injector**: Enhanced logging with emoji indicators
  - Display first 5 friends during injection
  - Verbose memory polling and timing information
  - Visual indicators for each phase

### Fixed
- `init_data!` macro reduced from 16 to 14 parameters
- `AddFontResourceExW` API call compatibility (None vs null_mut)
- All `GetPrivateProfile*` calls use correct PCSTR parameter
- Missing closing brace in `register_game_font()` unsafe block

### Build
- Clean release compilation without errors
- 11 warnings (10 fixable with cargo fix)
