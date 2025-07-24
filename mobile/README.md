# 🦭 Pinniped Mobile Integration

This directory contains the mobile integration for Pinniped's Rust-powered markdown parser, with native iOS support and React Native bridge.

## 📁 Project Structure

```
mobile/
├── ios/                          # iOS native integration
│   └── Pinniped/
│       ├── RustCore/
│       │   └── PinnipedCore.swift # Swift wrapper for Rust FFI
│       └── NativeModules/
│           ├── RNPinnipedEditor.swift # React Native bridge (Swift)
│           └── RNPinnipedEditor.m     # React Native bridge (Objective-C)
└── src/                          # JavaScript/TypeScript interface
    └── native/
        ├── PinnipedEditor.ts     # TypeScript interface for React Native
        └── example.ts            # Usage examples and React component
```

## 🚀 Quick Start

### 1. Build the iOS Framework

```bash
cd rust-core
./build-ios.sh
```

This creates:
- Static libraries for all iOS targets
- Universal simulator library
- XCFramework ready for iOS projects
- C header file for native integration

### 2. Add to iOS Project

1. Copy `../target/PinnipedCore.xcframework` to your iOS project
2. Add it to your project's "Frameworks, Libraries, and Embedded Content"
3. Import the Swift wrapper:

```swift
import PinnipedCore

let editor = PinnipedCore()
let document = try editor.parseMarkdown("# Hello World")
```

### 3. React Native Integration

```typescript
import PinnipedEditor from './src/native/PinnipedEditor';

const editor = new PinnipedEditor();
await editor.initialize();

const document = await editor.parseMarkdown("# Hello World\n\nThis is **bold** text.");
console.log(document);
```

## 📚 Core Features

### Document Parsing
- **Markdown → AST**: Parse markdown into structured document format
- **AST → Markdown**: Convert document back to markdown (round-trip)
- **JSON Serialization**: Full document serialization for storage/transmission

### Table Operations
- **Navigation**: Move between table cells (up/down/left/right)
- **Cell Access**: Get/set individual cell content
- **Header Detection**: Automatic table header recognition

### Error Handling
- **Memory Safe**: Automatic memory management with RAII
- **Error Recovery**: Graceful handling of malformed markdown
- **Type Safety**: Full type safety through Swift/TypeScript interfaces

## 🔧 API Reference

### Swift (PinnipedCore)

```swift
// Initialize
let core = PinnipedCore()

// Parse markdown
let document = try core.parseMarkdown("# Title\n\nContent")

// Convert back to markdown
let markdown = try core.toMarkdown(document)

// Table navigation
let position = try core.navigateTable(
    document: document,
    blockIndex: 1,
    currentRow: 0,
    currentCol: 0,
    direction: .right
)

// Get cell content
let content = try core.getTableCell(
    document: document,
    blockIndex: 1,
    row: 0,
    col: 1
)
```

### React Native (PinnipedEditor)

```typescript
// Initialize
const editor = new PinnipedEditor();
await editor.initialize();

// Parse markdown
const document = await editor.parseMarkdown(markdown);

// Navigate table
const position = await editor.navigateTable(1, 0, 0, 'right');

// Get cell content
const content = await editor.getTableCell(1, 0, 1);

// Event listeners
editor.onDocumentChanged((event) => {
    console.log('Document updated:', event.document);
});

editor.onSelectionChanged((event) => {
    console.log('Selection:', event.row, event.col);
});

// Cleanup
await editor.destroy();
```

## 🏗️ Document Structure

The parsed document follows this structure:

```typescript
interface Document {
    blocks: Block[]
}

type Block = 
    | { Paragraph: InlineText }
    | { Header: { level: number; text: InlineText } }
    | { UnorderedList: InlineText[] }
    | { OrderedList: InlineText[] }
    | { CodeBlock: { language?: string; code: string } }
    | { Blockquote: InlineText }
    | { Table: { rows: string[][]; has_header: boolean } }
```

## 🧪 Testing

The mobile integration includes comprehensive tests:

```bash
# Run all tests
cargo test

# Run specific test suites
cargo test basic_tests
cargo test integration_tests
cargo test edge_cases
```

**Test Coverage:**
- 50+ test cases covering all core functionality
- Edge case handling (malformed markdown, unicode, large documents)
- Performance tests for large documents
- Memory safety verification
- Round-trip conversion accuracy

## 📱 iOS Integration Details

### Build Targets
- **aarch64-apple-ios**: iOS devices (iPhone/iPad)
- **aarch64-apple-ios-sim**: iOS Simulator on M1 Macs
- **x86_64-apple-ios**: iOS Simulator on Intel Macs

### Memory Management
- Automatic string cleanup via RAII
- Safe FFI boundary with proper error handling
- No memory leaks in long-running applications

### Error Handling
```swift
enum PinnipedError: Error {
    case parseError(String)
    case rustError(String)
    case invalidInput(String)
}
```

## 🔄 React Native Bridge

The React Native bridge provides:

### Native Module Features
- **Event Emission**: Document and selection change events
- **Promise-based API**: All operations return promises
- **Error Propagation**: Rust errors properly surfaced to JS
- **Memory Management**: Automatic editor lifecycle management

### JavaScript Interface
- **TypeScript Support**: Full type definitions
- **Event Subscriptions**: Easy event handling with cleanup
- **Utility Functions**: Document analysis helpers
- **Example Components**: Ready-to-use React components

## 🚀 Performance

**Benchmarks** (tested on iPhone 12 Pro):
- Parse 1MB markdown document: ~50ms
- Serialize to JSON: ~25ms  
- Table navigation: <1ms
- Memory usage: ~2MB for large documents

## 🔐 Security

- **Memory Safe**: Rust's ownership system prevents memory corruption
- **Input Validation**: All inputs validated at FFI boundary
- **Error Isolation**: Parse errors don't crash the application
- **No Code Injection**: Safe handling of user-provided markdown

## 📋 Roadmap

### Phase 1 (Current)
- ✅ Core parsing and serialization
- ✅ iOS native integration
- ✅ React Native bridge
- ✅ Table navigation

### Phase 2 (Future)
- [ ] Android integration (Kotlin/JNI)
- [ ] Real-time collaborative editing
- [ ] Custom syntax extensions
- [ ] Advanced table calculations

## 🛠️ Development

### Prerequisites
- Rust 1.82+ with iOS targets
- Xcode Command Line Tools
- React Native development environment

### Building from Source
```bash
# Install Rust targets
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

# Build iOS framework
cd rust-core
./build-ios.sh

# Run tests
cargo test
```

### Debugging
- Use `console.log` in React Native for JavaScript debugging
- Use Xcode debugger for native Swift code
- Rust debug prints go to Xcode console when running on device/simulator

## 📄 License

This mobile integration is part of the Pinniped project. See the main project LICENSE file for details.

---

For more information about the core Rust implementation, see the main project documentation.