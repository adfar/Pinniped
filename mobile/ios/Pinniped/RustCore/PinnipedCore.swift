import Foundation

/// Swift wrapper for Pinniped Rust core markdown parser
public class PinnipedCore {
    
    /// Initialize the Pinniped core
    public init() {
        // No initialization needed for static functions
    }
    
    /// Parse markdown text into a Document structure
    /// - Parameter input: Markdown text to parse
    /// - Returns: Parsed Document structure
    /// - Throws: PinnipedError if parsing fails
    public func parseMarkdown(_ input: String) throws -> Document {
        guard let resultPtr = pinniped_parse_markdown(input) else {
            throw PinnipedError.parseError("Failed to call parse function")
        }
        
        defer { pinniped_free_string(resultPtr) }
        
        let resultString = String(cString: resultPtr)
        guard let data = resultString.data(using: .utf8) else {
            throw PinnipedError.parseError("Invalid UTF-8 in result")
        }
        
        // Try to decode as error first
        if let errorResponse = try? JSONDecoder().decode(ErrorResponse.self, from: data) {
            throw PinnipedError.rustError(errorResponse.error)
        }
        
        // Decode as document
        do {
            return try JSONDecoder().decode(Document.self, from: data)
        } catch {
            throw PinnipedError.parseError("Failed to decode document: \(error.localizedDescription)")
        }
    }
    
    /// Convert a Document back to markdown text
    /// - Parameter document: Document to convert
    /// - Returns: Markdown text representation
    /// - Throws: PinnipedError if conversion fails
    public func toMarkdown(_ document: Document) throws -> String {
        let encoder = JSONEncoder()
        let jsonData = try encoder.encode(document)
        guard let jsonString = String(data: jsonData, encoding: .utf8) else {
            throw PinnipedError.parseError("Failed to create JSON string")
        }
        
        guard let resultPtr = pinniped_to_markdown(jsonString) else {
            throw PinnipedError.parseError("Failed to call to_markdown function")
        }
        
        defer { pinniped_free_string(resultPtr) }
        
        return String(cString: resultPtr)
    }
    
    /// Navigate within a table
    /// - Parameters:
    ///   - document: Document containing the table
    ///   - blockIndex: Index of the table block
    ///   - currentRow: Current row position
    ///   - currentCol: Current column position
    ///   - direction: Navigation direction
    /// - Returns: New cell position, or nil if navigation is invalid
    /// - Throws: PinnipedError if operation fails
    public func navigateTable(
        document: Document,
        blockIndex: Int,
        currentRow: Int,
        currentCol: Int,
        direction: TableNavigation
    ) throws -> CellPosition? {
        let encoder = JSONEncoder()
        let jsonData = try encoder.encode(document)
        guard let jsonString = String(data: jsonData, encoding: .utf8) else {
            throw PinnipedError.parseError("Failed to create JSON string")
        }
        
        guard let resultPtr = pinniped_table_navigate(
            jsonString,
            Int32(blockIndex),
            Int32(currentRow),
            Int32(currentCol),
            direction.rawValue
        ) else {
            throw PinnipedError.parseError("Failed to call table_navigate function")
        }
        
        defer { pinniped_free_string(resultPtr) }
        
        let resultString = String(cString: resultPtr)
        guard let data = resultString.data(using: .utf8) else {
            throw PinnipedError.parseError("Invalid UTF-8 in navigation result")
        }
        
        // Try to decode as error first
        if let errorResponse = try? JSONDecoder().decode(ErrorResponse.self, from: data) {
            throw PinnipedError.rustError(errorResponse.error)
        }
        
        // Decode as cell position
        do {
            let position = try JSONDecoder().decode(CellPosition.self, from: data)
            return position.valid ? position : nil
        } catch {
            throw PinnipedError.parseError("Failed to decode navigation result: \(error.localizedDescription)")
        }
    }
    
    /// Get cell content at specified position
    /// - Parameters:
    ///   - document: Document containing the table
    ///   - blockIndex: Index of the table block
    ///   - row: Row position
    ///   - col: Column position
    /// - Returns: Cell content
    /// - Throws: PinnipedError if operation fails
    public func getTableCell(
        document: Document,
        blockIndex: Int,
        row: Int,
        col: Int
    ) throws -> String {
        let encoder = JSONEncoder()
        let jsonData = try encoder.encode(document)
        guard let jsonString = String(data: jsonData, encoding: .utf8) else {
            throw PinnipedError.parseError("Failed to create JSON string")
        }
        
        guard let resultPtr = pinniped_table_get_cell(
            jsonString,
            Int32(blockIndex),
            Int32(row),
            Int32(col)
        ) else {
            throw PinnipedError.parseError("Failed to call table_get_cell function")
        }
        
        defer { pinniped_free_string(resultPtr) }
        
        let resultString = String(cString: resultPtr)
        guard let data = resultString.data(using: .utf8) else {
            throw PinnipedError.parseError("Invalid UTF-8 in cell result")
        }
        
        // Try to decode as error first
        if let errorResponse = try? JSONDecoder().decode(ErrorResponse.self, from: data) {
            throw PinnipedError.rustError(errorResponse.error)
        }
        
        // Decode as cell content
        do {
            let cellResponse = try JSONDecoder().decode(CellContentResponse.self, from: data)
            return cellResponse.content
        } catch {
            throw PinnipedError.parseError("Failed to decode cell content: \(error.localizedDescription)")
        }
    }
}

// MARK: - Supporting Types

/// Document structure representing parsed markdown
public struct Document: Codable {
    public let blocks: [Block]
    
    public init(blocks: [Block]) {
        self.blocks = blocks
    }
}

/// Block types in a document
public enum Block: Codable {
    case paragraph(ParagraphBlock)
    case header(HeaderBlock)
    case unorderedList(UnorderedListBlock)
    case orderedList(OrderedListBlock)
    case codeBlock(CodeBlock)
    case blockquote(BlockquoteBlock)
    case table(TableBlock)
    
    // Custom coding keys to match Rust enum serialization
    private enum CodingKeys: String, CodingKey {
        case Paragraph, Header, UnorderedList, OrderedList, CodeBlock, Blockquote, Table
    }
    
    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        
        if let paragraph = try container.decodeIfPresent(InlineText.self, forKey: .Paragraph) {
            self = .paragraph(ParagraphBlock(content: paragraph))
        } else if let header = try container.decodeIfPresent(HeaderData.self, forKey: .Header) {
            self = .header(HeaderBlock(level: header.level, text: header.text))
        } else if let list = try container.decodeIfPresent([InlineText].self, forKey: .UnorderedList) {
            self = .unorderedList(UnorderedListBlock(items: list))
        } else if let list = try container.decodeIfPresent([InlineText].self, forKey: .OrderedList) {
            self = .orderedList(OrderedListBlock(items: list))
        } else if let code = try container.decodeIfPresent(CodeBlockData.self, forKey: .CodeBlock) {
            self = .codeBlock(CodeBlock(language: code.language, code: code.code))
        } else if let quote = try container.decodeIfPresent(InlineText.self, forKey: .Blockquote) {
            self = .blockquote(BlockquoteBlock(content: quote))
        } else if let table = try container.decodeIfPresent(TableData.self, forKey: .Table) {
            self = .table(TableBlock(rows: table.rows, hasHeader: table.has_header))
        } else {
            throw DecodingError.dataCorrupted(
                DecodingError.Context(codingPath: decoder.codingPath, debugDescription: "Unknown block type")
            )
        }
    }
    
    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        
        switch self {
        case .paragraph(let block):
            try container.encode(block.content, forKey: .Paragraph)
        case .header(let block):
            try container.encode(HeaderData(level: block.level, text: block.text), forKey: .Header)
        case .unorderedList(let block):
            try container.encode(block.items, forKey: .UnorderedList)
        case .orderedList(let block):
            try container.encode(block.items, forKey: .OrderedList)
        case .codeBlock(let block):
            try container.encode(CodeBlockData(language: block.language, code: block.code), forKey: .CodeBlock)
        case .blockquote(let block):
            try container.encode(block.content, forKey: .Blockquote)
        case .table(let block):
            try container.encode(TableData(rows: block.rows, has_header: block.hasHeader), forKey: .Table)
        }
    }
}

// Helper structures for encoding/decoding
private struct HeaderData: Codable {
    let level: UInt8
    let text: InlineText
}

private struct CodeBlockData: Codable {
    let language: String?
    let code: String
}

private struct TableData: Codable {
    let rows: [[String]]
    let has_header: Bool
}

/// Individual block types
public struct ParagraphBlock {
    public let content: InlineText
    public init(content: InlineText) { self.content = content }
}

public struct HeaderBlock {
    public let level: UInt8
    public let text: InlineText
    public init(level: UInt8, text: InlineText) { self.level = level; self.text = text }
}

public struct UnorderedListBlock {
    public let items: [InlineText]
    public init(items: [InlineText]) { self.items = items }
}

public struct OrderedListBlock {
    public let items: [InlineText]
    public init(items: [InlineText]) { self.items = items }
}

public struct CodeBlock {
    public let language: String?
    public let code: String
    public init(language: String?, code: String) { self.language = language; self.code = code }
}

public struct BlockquoteBlock {
    public let content: InlineText
    public init(content: InlineText) { self.content = content }
}

public struct TableBlock {
    public let rows: [[String]]
    public let hasHeader: Bool
    public init(rows: [[String]], hasHeader: Bool) { self.rows = rows; self.hasHeader = hasHeader }
}

/// Inline text with formatting elements
public struct InlineText: Codable {
    public let elements: [InlineElement]
    public init(elements: [InlineElement]) { self.elements = elements }
}

/// Inline formatting elements
public enum InlineElement: Codable {
    case text(String)
    case bold([InlineElement])
    case italic([InlineElement])
    case code(String)
    case link(text: String, url: String)
    
    private enum CodingKeys: String, CodingKey {
        case Text, Bold, Italic, Code, Link
    }
    
    public init(from decoder: Decoder) throws {
        let container = try decoder.container(keyedBy: CodingKeys.self)
        
        if let text = try container.decodeIfPresent(String.self, forKey: .Text) {
            self = .text(text)
        } else if let bold = try container.decodeIfPresent([InlineElement].self, forKey: .Bold) {
            self = .bold(bold)
        } else if let italic = try container.decodeIfPresent([InlineElement].self, forKey: .Italic) {
            self = .italic(italic)
        } else if let code = try container.decodeIfPresent(String.self, forKey: .Code) {
            self = .code(code)
        } else if let link = try container.decodeIfPresent(LinkData.self, forKey: .Link) {
            self = .link(text: link.text, url: link.url)
        } else {
            throw DecodingError.dataCorrupted(
                DecodingError.Context(codingPath: decoder.codingPath, debugDescription: "Unknown inline element type")
            )
        }
    }
    
    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        
        switch self {
        case .text(let text):
            try container.encode(text, forKey: .Text)
        case .bold(let elements):
            try container.encode(elements, forKey: .Bold)
        case .italic(let elements):
            try container.encode(elements, forKey: .Italic)
        case .code(let code):
            try container.encode(code, forKey: .Code)
        case .link(let text, let url):
            try container.encode(LinkData(text: text, url: url), forKey: .Link)
        }
    }
}

private struct LinkData: Codable {
    let text: String
    let url: String
}

/// Table navigation directions
public enum TableNavigation: Int32 {
    case up = 0
    case down = 1
    case left = 2
    case right = 3
}

/// Cell position in a table
public struct CellPosition: Codable {
    public let row: Int
    public let col: Int
    public let valid: Bool
    
    public init(row: Int, col: Int, valid: Bool) {
        self.row = row
        self.col = col
        self.valid = valid
    }
}

/// Error types for Pinniped operations
public enum PinnipedError: Error, LocalizedError {
    case parseError(String)
    case rustError(String)
    case invalidInput(String)
    
    public var errorDescription: String? {
        switch self {
        case .parseError(let message):
            return "Parse error: \(message)"
        case .rustError(let message):
            return "Rust core error: \(message)"
        case .invalidInput(let message):
            return "Invalid input: \(message)"
        }
    }
}

// Internal helper structures
private struct ErrorResponse: Codable {
    let error: String
}

private struct CellContentResponse: Codable {
    let content: String
}

// MARK: - C Function Declarations

/// C function declarations for FFI
@_silgen_name("pinniped_parse_markdown")
private func pinniped_parse_markdown(_ input: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

@_silgen_name("pinniped_to_markdown")
private func pinniped_to_markdown(_ document_json: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>?

@_silgen_name("pinniped_table_navigate")
private func pinniped_table_navigate(
    _ document_json: UnsafePointer<CChar>,
    _ block_index: Int32,
    _ current_row: Int32,
    _ current_col: Int32,
    _ direction: Int32
) -> UnsafeMutablePointer<CChar>?

@_silgen_name("pinniped_table_get_cell")
private func pinniped_table_get_cell(
    _ document_json: UnsafePointer<CChar>,
    _ block_index: Int32,
    _ row: Int32,
    _ col: Int32
) -> UnsafeMutablePointer<CChar>?

@_silgen_name("pinniped_free_string")
private func pinniped_free_string(_ s: UnsafeMutablePointer<CChar>)