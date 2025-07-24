import Foundation
import React

@objc(RNPinnipedEditor)
class RNPinnipedEditor: RCTEventEmitter {
    
    private var editors: [String: PinnipedCore] = [:]
    private var documents: [String: Document] = [:]
    
    // MARK: - Editor Lifecycle
    
    @objc
    func createEditor(_ editorId: String,
                     resolver: @escaping RCTPromiseResolveBlock,
                     rejecter: @escaping RCTPromiseRejectBlock) {
        DispatchQueue.global(qos: .userInitiated).async {
            do {
                let editor = PinnipedCore()
                self.editors[editorId] = editor
                
                DispatchQueue.main.async {
                    resolver(true)
                }
            } catch {
                DispatchQueue.main.async {
                    rejecter("CREATE_EDITOR_ERROR", "Failed to create editor: \(error.localizedDescription)", error)
                }
            }
        }
    }
    
    @objc
    func destroyEditor(_ editorId: String,
                      resolver: @escaping RCTPromiseResolveBlock,
                      rejecter: @escaping RCTPromiseRejectBlock) {
        editors.removeValue(forKey: editorId)
        documents.removeValue(forKey: editorId)
        resolver(true)
    }
    
    // MARK: - Document Operations
    
    @objc
    func parseMarkdown(_ editorId: String,
                      markdown: String,
                      resolver: @escaping RCTPromiseResolveBlock,
                      rejecter: @escaping RCTPromiseRejectBlock) {
        guard let editor = editors[editorId] else {
            rejecter("NO_EDITOR", "Editor not found for ID: \(editorId)", nil)
            return
        }
        
        DispatchQueue.global(qos: .userInitiated).async {
            do {
                let document = try editor.parseMarkdown(markdown)
                self.documents[editorId] = document
                
                // Convert to JSON for React Native
                let encoder = JSONEncoder()
                let data = try encoder.encode(document)
                let json = String(data: data, encoding: .utf8)!
                
                DispatchQueue.main.async {
                    resolver(json)
                    
                    // Emit document changed event
                    self.sendEvent(withName: "documentChanged", body: [
                        "editorId": editorId,
                        "document": json
                    ])
                }
            } catch {
                DispatchQueue.main.async {
                    rejecter("PARSE_ERROR", "Failed to parse markdown: \(error.localizedDescription)", error)
                }
            }
        }
    }
    
    @objc
    func toMarkdown(_ editorId: String,
                   resolver: @escaping RCTPromiseResolveBlock,
                   rejecter: @escaping RCTPromiseRejectBlock) {
        guard let editor = editors[editorId],
              let document = documents[editorId] else {
            rejecter("NO_EDITOR_OR_DOCUMENT", "Editor or document not found for ID: \(editorId)", nil)
            return
        }
        
        DispatchQueue.global(qos: .userInitiated).async {
            do {
                let markdown = try editor.toMarkdown(document)
                
                DispatchQueue.main.async {
                    resolver(markdown)
                }
            } catch {
                DispatchQueue.main.async {
                    rejecter("TO_MARKDOWN_ERROR", "Failed to convert to markdown: \(error.localizedDescription)", error)
                }
            }
        }
    }
    
    // MARK: - Table Operations
    
    @objc
    func navigateTable(_ editorId: String,
                      blockIndex: NSNumber,
                      currentRow: NSNumber,
                      currentCol: NSNumber,
                      direction: String,
                      resolver: @escaping RCTPromiseResolveBlock,
                      rejecter: @escaping RCTPromiseRejectBlock) {
        guard let editor = editors[editorId],
              let document = documents[editorId] else {
            rejecter("NO_EDITOR_OR_DOCUMENT", "Editor or document not found for ID: \(editorId)", nil)
            return
        }
        
        guard let nav = TableNavigation.from(string: direction) else {
            rejecter("INVALID_DIRECTION", "Invalid navigation direction: \(direction)", nil)
            return
        }
        
        DispatchQueue.global(qos: .userInitiated).async {
            do {
                let position = try editor.navigateTable(
                    document: document,
                    blockIndex: blockIndex.intValue,
                    currentRow: currentRow.intValue,
                    currentCol: currentCol.intValue,
                    direction: nav
                )
                
                DispatchQueue.main.async {
                    if let position = position {
                        resolver([
                            "row": position.row,
                            "col": position.col,
                            "valid": position.valid
                        ])
                        
                        // Emit selection changed event
                        self.sendEvent(withName: "selectionChanged", body: [
                            "editorId": editorId,
                            "blockIndex": blockIndex,
                            "row": position.row,
                            "col": position.col
                        ])
                    } else {
                        resolver(NSNull())
                    }
                }
            } catch {
                DispatchQueue.main.async {
                    rejecter("NAVIGATE_ERROR", "Failed to navigate table: \(error.localizedDescription)", error)
                }
            }
        }
    }
    
    @objc
    func getTableCell(_ editorId: String,
                     blockIndex: NSNumber,
                     row: NSNumber,
                     col: NSNumber,
                     resolver: @escaping RCTPromiseResolveBlock,
                     rejecter: @escaping RCTPromiseRejectBlock) {
        guard let editor = editors[editorId],
              let document = documents[editorId] else {
            rejecter("NO_EDITOR_OR_DOCUMENT", "Editor or document not found for ID: \(editorId)", nil)
            return
        }
        
        DispatchQueue.global(qos: .userInitiated).async {
            do {
                let content = try editor.getTableCell(
                    document: document,
                    blockIndex: blockIndex.intValue,
                    row: row.intValue,
                    col: col.intValue
                )
                
                DispatchQueue.main.async {
                    resolver(content)
                }
            } catch {
                DispatchQueue.main.async {
                    rejecter("GET_CELL_ERROR", "Failed to get table cell: \(error.localizedDescription)", error)
                }
            }
        }
    }
    
    // MARK: - Utility Methods
    
    @objc
    func getDocumentInfo(_ editorId: String,
                        resolver: @escaping RCTPromiseResolveBlock,
                        rejecter: @escaping RCTPromiseRejectBlock) {
        guard let document = documents[editorId] else {
            rejecter("NO_DOCUMENT", "Document not found for ID: \(editorId)", nil)
            return
        }
        
        var blockInfo: [[String: Any]] = []
        for (index, block) in document.blocks.enumerated() {
            var info: [String: Any] = ["index": index]
            
            switch block {
            case .paragraph:
                info["type"] = "paragraph"
            case .header(let header):
                info["type"] = "header"
                info["level"] = header.level
            case .unorderedList(let list):
                info["type"] = "unorderedList"
                info["itemCount"] = list.items.count
            case .orderedList(let list):
                info["type"] = "orderedList"
                info["itemCount"] = list.items.count
            case .codeBlock(let code):
                info["type"] = "codeBlock"
                info["language"] = code.language ?? NSNull()
            case .blockquote:
                info["type"] = "blockquote"
            case .table(let table):
                info["type"] = "table"
                info["rowCount"] = table.rows.count
                info["colCount"] = table.rows.first?.count ?? 0
                info["hasHeader"] = table.hasHeader
            }
            
            blockInfo.append(info)
        }
        
        resolver([
            "blockCount": document.blocks.count,
            "blocks": blockInfo
        ])
    }
    
    // MARK: - RCTEventEmitter Overrides
    
    override func supportedEvents() -> [String]! {
        return [
            "documentChanged",
            "selectionChanged",
            "parseError"
        ]
    }
    
    override static func requiresMainQueueSetup() -> Bool {
        return false
    }
    
    override func constantsToExport() -> [AnyHashable : Any]! {
        return [
            "version": "1.0.0",
            "supportedFormats": ["markdown"],
            "tableNavigationDirections": [
                "up": TableNavigation.up.rawValue,
                "down": TableNavigation.down.rawValue,
                "left": TableNavigation.left.rawValue,
                "right": TableNavigation.right.rawValue
            ]
        ]
    }
}

// MARK: - Extensions

extension TableNavigation {
    static func from(string: String) -> TableNavigation? {
        switch string.lowercased() {
        case "up": return .up
        case "down": return .down
        case "left": return .left
        case "right": return .right
        default: return nil
        }
    }
}