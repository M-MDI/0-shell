# Development Notes

## Architecture Overview

- `main.rs` - Entry point and shell initialization
- `shell.rs` - Main shell loop and user interaction
- `parser.rs` - Command line parsing logic
- `commands/` - Individual command implementations
- `error.rs` - Error handling and custom error types
- `utils.rs` - Shared utility functions

## Command Implementation Guidelines

Each command should:

1. Parse its arguments correctly
2. Handle errors gracefully
3. Follow Unix conventions
4. Return appropriate exit codes
