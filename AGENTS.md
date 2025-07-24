# AI Agent Instructions for Pinniped Development

## Project Overview

Pinniped is a professional note-taking application with a focus on:
- Typora-style WYSIWYG markdown editing
- Excellent table support with calculations
- Cross-platform (iOS, Android, Desktop) via shared Rust core
- Natural language search capabilities
- Zero-friction capture and effortless recall

## Architecture

The project uses a Rust core for shared logic with native UI shells:
- **Rust Core**: Document model, markdown parsing, table engine, sync logic
- **Native Bindings**: iOS (Swift/FFI), Android (Kotlin/JNI), Web (WASM)
- **UI Layer**: React Native for mobile, Electron/Web for desktop

## Current Development Phase

We are building a web-based test harness to validate the Rust markdown parser before implementing mobile bindings.

## Key Technical Decisions

1. **Markdown Parsing**: Build custom parser for Typora-like WYSIWYG experience
2. **Tables**: First-class support with navigation, calculations, and formulas
3. **Document Model**: CRDT-based for eventual sync support
4. **Operations**: All edits as operations for undo/redo and sync

## Code Style Guidelines

### Rust
- Use descriptive names for public APIs
- Keep modules focused and small
- Extensive unit tests for parser
- Document all public interfaces
- Error handling with `Result<T, Error>` types

### JavaScript/TypeScript
- Modern ES6+ syntax
- Functional style where appropriate
- Clear variable names
- Comment complex logic

## Testing Requirements

1. Parser must round-trip perfectly (markdown → AST → markdown)
2. Table operations must handle edge cases
3. Performance benchmarks for large documents
4. Cross-browser testing for web interface

## AI Assistant Guidelines

When implementing features:
1. Start with tests that define expected behavior
2. Implement the simplest solution first
3. Optimize only after profiling
4. Maintain clean separation between Rust core and UI
5. Document any non-obvious design decisions

When reviewing code:
1. Check for edge cases in parser
2. Ensure operations are reversible
3. Verify WASM bindings are minimal
4. Look for performance bottlenecks

## Common Pitfalls to Avoid

1. Don't put UI logic in Rust core
2. Don't expose complex Rust types through FFI
3. Keep WASM bundle size minimal
4. Avoid premature optimization
5. Don't break markdown compatibility

## Project Structure

/pinniped/
├── rust-core/           # Core Rust library
│   ├── src/
│   │   ├── lib.rs      # Library entry point
│   │   ├── document.rs # Document model
│   │   ├── parser.rs   # Markdown parser
│   │   ├── table.rs    # Table-specific logic
│   │   ├── block.rs    # Block types (paragraph, list, etc)
│   │   └── error.rs    # Error types
│   ├── tests/
│   │   └── parser_tests.rs
│   └── Cargo.toml
│
├── wasm-bindings/       # WASM wrapper for web
│   ├── src/
│   │   └── lib.rs      # Thin WASM bindings
│   ├── Cargo.toml
│   └── build.sh        # Build script
│
├── web-test/            # Web test harness
│   ├── index.html      # Test interface
│   ├── main.js         # Test logic
│   ├── style.css       # Basic styling
│   └── serve.sh        # Dev server script
│
├── Cargo.toml          # Workspace root
├── README.md
└── AGENTS.md           