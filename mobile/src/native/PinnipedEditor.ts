import { NativeModules, NativeEventEmitter, EmitterSubscription } from 'react-native';

const { RNPinnipedEditor } = NativeModules;

if (!RNPinnipedEditor) {
  throw new Error('RNPinnipedEditor native module is not available. Make sure the native code is properly linked.');
}

const editorEmitter = new NativeEventEmitter(RNPinnipedEditor);

// Type definitions
export interface Document {
  blocks: Block[];
}

export type Block = 
  | { Paragraph: InlineText }
  | { Header: { level: number; text: InlineText } }
  | { UnorderedList: InlineText[] }
  | { OrderedList: InlineText[] }
  | { CodeBlock: { language?: string; code: string } }
  | { Blockquote: InlineText }
  | { Table: { rows: string[][]; has_header: boolean } };

export interface InlineText {
  elements: InlineElement[];
}

export type InlineElement =
  | { Text: string }
  | { Bold: InlineElement[] }
  | { Italic: InlineElement[] }
  | { Code: string }
  | { Link: { text: string; url: string } };

export interface CellPosition {
  row: number;
  col: number;
  valid: boolean;
}

export interface DocumentInfo {
  blockCount: number;
  blocks: BlockInfo[];
}

export interface BlockInfo {
  index: number;
  type: string;
  level?: number;
  itemCount?: number;
  language?: string;
  rowCount?: number;
  colCount?: number;
  hasHeader?: boolean;
}

export type TableNavigationDirection = 'up' | 'down' | 'left' | 'right';

export interface PinnipedEditorEvents {
  documentChanged: {
    editorId: string;
    document: string;
  };
  selectionChanged: {
    editorId: string;
    blockIndex: number;
    row: number;
    col: number;
  };
  parseError: {
    editorId: string;
    error: string;
  };
}

export class PinnipedEditor {
  private editorId: string;
  private eventSubscriptions: EmitterSubscription[] = [];

  constructor(editorId?: string) {
    this.editorId = editorId || `editor-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }

  /**
   * Initialize the editor instance
   */
  async initialize(): Promise<void> {
    await RNPinnipedEditor.createEditor(this.editorId);
  }

  /**
   * Parse markdown text into a Document structure
   */
  async parseMarkdown(markdown: string): Promise<Document> {
    const jsonString = await RNPinnipedEditor.parseMarkdown(this.editorId, markdown);
    return JSON.parse(jsonString);
  }

  /**
   * Convert the current document back to markdown
   */
  async toMarkdown(): Promise<string> {
    return RNPinnipedEditor.toMarkdown(this.editorId);
  }

  /**
   * Navigate within a table
   */
  async navigateTable(
    blockIndex: number,
    currentRow: number,
    currentCol: number,
    direction: TableNavigationDirection
  ): Promise<CellPosition | null> {
    const result = await RNPinnipedEditor.navigateTable(
      this.editorId,
      blockIndex,
      currentRow,
      currentCol,
      direction
    );
    return result === null ? null : result;
  }

  /**
   * Get content of a table cell
   */
  async getTableCell(
    blockIndex: number,
    row: number,
    col: number
  ): Promise<string> {
    return RNPinnipedEditor.getTableCell(this.editorId, blockIndex, row, col);
  }

  /**
   * Get information about the current document
   */
  async getDocumentInfo(): Promise<DocumentInfo> {
    return RNPinnipedEditor.getDocumentInfo(this.editorId);
  }

  /**
   * Subscribe to document change events
   */
  onDocumentChanged(callback: (event: PinnipedEditorEvents['documentChanged']) => void): () => void {
    const subscription = editorEmitter.addListener('documentChanged', (event) => {
      if (event.editorId === this.editorId) {
        callback(event);
      }
    });

    this.eventSubscriptions.push(subscription);

    return () => {
      const index = this.eventSubscriptions.indexOf(subscription);
      if (index > -1) {
        this.eventSubscriptions.splice(index, 1);
        subscription.remove();
      }
    };
  }

  /**
   * Subscribe to selection change events
   */
  onSelectionChanged(callback: (event: PinnipedEditorEvents['selectionChanged']) => void): () => void {
    const subscription = editorEmitter.addListener('selectionChanged', (event) => {
      if (event.editorId === this.editorId) {
        callback(event);
      }
    });

    this.eventSubscriptions.push(subscription);

    return () => {
      const index = this.eventSubscriptions.indexOf(subscription);
      if (index > -1) {
        this.eventSubscriptions.splice(index, 1);
        subscription.remove();
      }
    };
  }

  /**
   * Subscribe to parse error events
   */
  onParseError(callback: (event: PinnipedEditorEvents['parseError']) => void): () => void {
    const subscription = editorEmitter.addListener('parseError', (event) => {
      if (event.editorId === this.editorId) {
        callback(event);
      }
    });

    this.eventSubscriptions.push(subscription);

    return () => {
      const index = this.eventSubscriptions.indexOf(subscription);
      if (index > -1) {
        this.eventSubscriptions.splice(index, 1);
        subscription.remove();
      }
    };
  }

  /**
   * Clean up resources and destroy the editor
   */
  async destroy(): Promise<void> {
    // Remove all event subscriptions
    this.eventSubscriptions.forEach(subscription => subscription.remove());
    this.eventSubscriptions = [];

    // Destroy the native editor instance
    await RNPinnipedEditor.destroyEditor(this.editorId);
  }

  /**
   * Get the editor ID
   */
  getEditorId(): string {
    return this.editorId;
  }

  /**
   * Get editor constants (version, supported features, etc.)
   */
  static getConstants(): Record<string, any> {
    return RNPinnipedEditor.getConstants?.() || {};
  }
}

// Utility functions for working with Document structure

export class DocumentUtils {
  /**
   * Find all table blocks in a document
   */
  static findTableBlocks(document: Document): Array<{ block: Block; index: number }> {
    return document.blocks
      .map((block, index) => ({ block, index }))
      .filter(({ block }) => 'Table' in block);
  }

  /**
   * Get plain text content from inline text
   */
  static getPlainText(inlineText: InlineText): string {
    return inlineText.elements
      .map(element => this.getPlainTextFromElement(element))
      .join('');
  }

  private static getPlainTextFromElement(element: InlineElement): string {
    if ('Text' in element) {
      return element.Text;
    } else if ('Bold' in element) {
      return element.Bold.map(e => this.getPlainTextFromElement(e)).join('');
    } else if ('Italic' in element) {
      return element.Italic.map(e => this.getPlainTextFromElement(e)).join('');
    } else if ('Code' in element) {
      return element.Code;
    } else if ('Link' in element) {
      return element.Link.text;
    }
    return '';
  }

  /**
   * Count words in a document
   */
  static countWords(document: Document): number {
    let wordCount = 0;
    
    for (const block of document.blocks) {
      if ('Paragraph' in block) {
        wordCount += this.countWordsInText(this.getPlainText(block.Paragraph));
      } else if ('Header' in block) {
        wordCount += this.countWordsInText(this.getPlainText(block.Header.text));
      } else if ('UnorderedList' in block) {
        for (const item of block.UnorderedList) {
          wordCount += this.countWordsInText(this.getPlainText(item));
        }
      } else if ('OrderedList' in block) {
        for (const item of block.OrderedList) {
          wordCount += this.countWordsInText(this.getPlainText(item));
        }
      } else if ('Blockquote' in block) {
        wordCount += this.countWordsInText(this.getPlainText(block.Blockquote));
      }
    }
    
    return wordCount;
  }

  private static countWordsInText(text: string): number {
    return text.trim().split(/\s+/).filter(word => word.length > 0).length;
  }
}

export default PinnipedEditor;