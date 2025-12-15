# MHF IELess Launcher

MHF default launcher requires IE to login. IE sucks.

This project reverse engineered the MHF launcher in order to make it possible to boot the game directly, without going through `mhf.exe` and `mhl.dll`.

## Why Use This?

If you're wondering 'Why use this instead of the original launcher?', here are some of the issues that are solved by using a custom launcher:

### Freedom from Internet Explorer
- **Not locked to IE** - Opens possibilities for modern launcher designs
- **Fast operations** - No more 10-second waits for each request
- **Linux/Steam Deck support** - Game can boot using Proton/Wine since IE was the main blocker

### Modern Network Stack
- **Flexible protocols** - Use HTTP(S), JSON, custom ports, etc.
- **No legacy constraints** - Implement operations the way you want

### Enhanced Features
- **New operations** - Add separate buttons for 'Sign Up' and 'Login'
- **Rich data** - Display extra information like character portraits in the launcher
- **No GameGuard modifications** - Since we're replacing the launcher, no need to patch `mhfo-hd.dll` to remove GameGuard checks

### Friends List Injection (NEW!)
- **In-game friends list** - Automatically populated from server data
- **HD and SD support** - Works seamlessly with both graphics modes
- **Character-specific** - Shows only friends for the active character
- **Zero configuration** - Automatically detects game version and injects at the correct memory addresses

## Features

### Core Functionality
- ✅ Direct game boot without `mhf.exe` / `mhl.dll`
- ✅ Support for both F5 and ZZ versions
- ✅ Custom server connection handling
- ✅ Character selection and management
- ✅ Notice/announcement system
- ✅ Mezfes event support

### Friends List System
- ✅ **Automatic injection** - Friends data injected directly into game memory
- ✅ **Version detection** - Auto-detects HD/SD mode from `mhf.ini`
- ✅ **Cross-platform** - Works on both graphics modes without configuration
- ✅ **Thread-safe** - Non-blocking async injection during game startup
- ✅ **Robust encoding** - Base32 ID conversion for proper friend identification

## Technical Details

### Friends List Implementation

The friends list system uses memory injection to populate the in-game friends list at runtime:

**Memory Offsets:**
- HD Mode (`mhfo-hd.dll`): `0x0ED7D6C0`
- SD Mode (`mhfo.dll`): `0x06142F20`

**Data Structure:**
```rust
const FRIEND_TABLE_SIZE: usize = 0x1000; // 4KB table
const FRIEND_ENTRY_SIZE: usize = 0x30; // 48 bytes per friend
const MAX_FRIENDS: usize = 50; // Maximum supported friends
```

**Process:**

Read `GRAPHICS_VER` from `mhf.ini` to determine HD/SD mode

Wait for game DLL to load in memory

Inject friends data at the correct offset

Friends appear in-game without server-side modifications

## Usage

### From Rust Projects

Make sure your project targets `nightly-i686-pc-windows-msvc`:

```rust
use mhf_iel::{run_mhf, MhfConfig, MhfVersion, FriendData};

let config = MhfConfig {
version: MhfVersion::ZZ,
char_id: 123,
char_name: "Hunter".to_string(),
friends: vec![
FriendData {
id: 1,
cid: 123,
name: "Friend1".to_string(),
},
// ... more friends
],
// ... other config
};

run_mhf(config)?;
```

### From Other Languages

se the [CLI interface](mhf-iel-cli/README.md) to run this project from any program without the `i686` limitation.

### Integration Options

Feel free to create a ticket if you need another way to integrate this lib into your app:
- `.dll` exports
- Static linking bindings
- IPC/socket communication
- Custom wrapper formats

## Compiling

### Prerequisites

Install the nightly toolchain and i686 target:

```bash
rustup toolchain install nightly
rustup target add i686-pc-windows-msvc
```

### Build Commands

```bash
# Build the library
cargo +nightly build --release --target i686-pc-windows-msvc

# Build the CLI tool
cargo +nightly build --release --target i686-pc-windows-msvc -p mhf-iel-cli
```

### For Linux Cross-Compilation

```bash
# Install MinGW toolchain
sudo apt install mingw-w64

# Add target
rustup target add i686-pc-windows-gnu

# Build
cargo +nightly build --release --target i686-pc-windows-gnu -p mhf-iel-cli
```

## Configuration

The launcher reads game settings from `mhf.ini` in the game directory:

```ini
[VIDEO]
GRAPHICS_VER=1 # 1 = HD, 0 = SD

[SCREEN]
FULLSCREEN_MODE=1
WINDOW_RESOLUTION_W=1920
WINDOW_RESOLUTION_H=1080

... other settings
```

## Server Integration

The launcher expects server responses with the following structure:

```json
{
"currentTs": 1234567890,
"expiryTs": 1234567890,
"userTokenId": 0,
"token": "abc123",
"rights": 1292,
"characters": [
{
"id": 1,
"name": "Hunter",
"isFemale": false,
"weapon": 0,
"hr": 999,
"gr": 100
}
],
"friends": [
{
"cid": 1,
"id": 2,
"name": "FriendName"
  }
    ]
      }
```

## Testing

```bash
# Run tests
cargo +nightly test --target i686-pc-windows-msvc

# Run with logging
RUST_LOG=debug cargo +nightly run --target i686-pc-windows-msvc
```

## Debugging

The friends injection system provides detailed logging:

```
🎮 [Main] Total friends in config: 22
🎯 [Main] Friends for char_id 3: 22

🔍 [Friends Injector] Starting...
DLL: mhfo-hd.dll
Base offset: 0x0ED7D6C0
Friends count: 22
ID:24 CID:3 Name:Wyxill
ID:12 CID:3 Name:Poe04​
...

✅ [Friends Injector] Module loaded at: 0x1ED7D6C0
⏱️ [Friends Injector] Waiting 2s for game init...
✅ [Friends Injector] Injection complete!
```

## Contributing

Contributions are welcome! Areas for improvement:
- Additional memory injection features
- Enhanced error handling
- Support for more game versions
- Performance optimizations
- Documentation improvements

## License

This project is provided as-is for Monster Hunter Frontier preservation efforts.

## Acknowledgments

- **Original reverse engineering work** - Foundation for this entire project
- **MHF preservation community** - Keeping the game alive
- **ButterClient project** - Inspiration for modern launcher features
- **Server developers** - Making private servers possible

## Project Status

✅ **Stable** - Ready for production use
- Core launcher functionality complete
- Friends list injection working in HD and SD modes
- Character selection and management functional
- Server communication stable

---

*"Things are either done right, or not done at all!"*
