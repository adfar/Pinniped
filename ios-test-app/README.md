# 🦭 Pinniped iOS Test App

A Typora-like WYSIWYG markdown editor showcasing the Pinniped Rust-powered parser on iOS.

## 🚀 Setup Instructions

### 1. Create Xcode Project

Since I can't create `.xcodeproj` files directly, you'll need to create the Xcode project:

1. Open Xcode
2. Create a new project: **iOS App**
3. Product Name: `PinnipedTestApp`
4. Interface: **SwiftUI**
5. Language: **Swift**
6. Save to: `/Users/aidanfarnum/piano/pinniped/Pinniped/ios-test-app/`

### 2. Add Project Files

After creating the project, **replace** the default files with the ones I've created:

- Replace `ContentView.swift` with the one in this directory
- Replace `App.swift` with the one in this directory  
- Add `WYSIWYGTextView.swift`
- Add `MarkdownRenderer.swift`
- Add `DebugInfoView.swift`
- Add the `PinnipedCore/` folder to your project

### 3. Add the XCFramework

1. Copy `/Users/aidanfarnum/piano/pinniped/Pinniped/target/PinnipedCore.xcframework` to your project folder
2. In Xcode, go to your target's **General** tab
3. Under **Frameworks, Libraries, and Embedded Content**, click **+**
4. Click **Add Other...** → **Add Files...**
5. Select `PinnipedCore.xcframework`
6. Make sure it's set to **Embed & Sign**

### 4. Configure Build Settings

1. Go to **Build Settings** → **Swift Compiler - General**
2. Set **Objective-C Bridging Header** to: `PinnipedTestApp/PinnipedTestApp-Bridging-Header.h`
3. Under **Search Paths** → **Header Search Paths**, add: `$(PROJECT_DIR)/PinnipedTestApp/PinnipedCore`

### 5. Build and Run

- Select iOS Simulator or a connected device
- Press **Cmd+R** to build and run

## 📱 Features

### WYSIWYG Editing
- **Headers**: Type `# Header` and see it rendered as large, bold text
- **Bold/Italic**: `**bold**` and `*italic*` text rendered inline
- **Code**: `` `inline code` `` with background highlighting
- **Lists**: `- item` and `1. item` rendered as proper lists
- **Tables**: Full table rendering with borders and headers
- **Blockquotes**: `> quote` with left border styling
- **Links**: `[text](url)` rendered as clickable links

### Real-time Parsing
- Parse-as-you-type with 200ms debouncing
- Performance metrics in debug panel
- Error handling with visual feedback

### Debug Information
- Parse time (should be <50ms for normal documents)
- Block count and character count
- Parse status indicator
- Toggle with info button in top-right

## 🧪 Testing

The app loads with comprehensive sample content including:
- All markdown syntax types
- Unicode characters and emojis
- Complex nested formatting
- Multiple tables
- Code blocks in different languages

Try editing the text to see the real-time WYSIWYG rendering in action!

## 🔧 Project Structure

```
PinnipedTestApp/
├── App.swift                      # SwiftUI app entry point
├── ContentView.swift              # Main view with editor state
├── WYSIWYGTextView.swift          # Custom UITextView wrapper
├── MarkdownRenderer.swift         # Document → NSAttributedString converter
├── DebugInfoView.swift            # Performance/debug information
├── PinnipedCore/
│   ├── PinnipedCore.swift         # Swift wrapper for Rust FFI
│   └── pinniped_core.h            # C header file
└── PinnipedTestApp-Bridging-Header.h
```

## 🚀 Performance

Expected performance on iPhone 12 Pro:
- **Parse Time**: 10-50ms for typical documents
- **Rendering**: Near-instant NSAttributedString conversion
- **Memory**: ~5MB for large documents with tables
- **Battery**: Minimal impact with debounced parsing

## 🐛 Troubleshooting

### Build Errors
- Ensure the XCFramework is properly embedded
- Check that the bridging header path is correct
- Verify all Swift files are added to the target

### Runtime Errors
- Check that the Rust library exports are working by looking at debug output
- Verify the sample content loads (indicates parsing is working)
- Use the debug panel to see parse errors

### Performance Issues
- Large documents (>100KB) may parse slower
- Table rendering is more expensive than simple text
- Consider increasing debounce time for very large documents

---

This test app demonstrates the full power of the Pinniped Rust-powered markdown parser with a native iOS WYSIWYG interface!