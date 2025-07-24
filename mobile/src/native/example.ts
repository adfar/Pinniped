/**
 * Example usage of PinnipedEditor in a React Native app
 */

import PinnipedEditor, { DocumentUtils } from './PinnipedEditor';

export async function exampleUsage() {
  // Create and initialize editor
  const editor = new PinnipedEditor();
  await editor.initialize();

  // Subscribe to events
  const unsubscribeDocumentChanged = editor.onDocumentChanged((event) => {
    console.log('Document changed:', event);
  });

  const unsubscribeSelectionChanged = editor.onSelectionChanged((event) => {
    console.log('Selection changed:', event.blockIndex, event.row, event.col);
  });

  // Sample markdown with table
  const markdown = `# 🦭 Pinniped Markdown Demo

Welcome to the **Pinniped** markdown parser! This parser is built in *Rust* and compiled for mobile.

## Features Showcase

### Text Formatting
- **Bold text** using double asterisks
- *Italic text* using single asterisks  
- \`Inline code\` using backticks

### Tables

| Feature | Status | Notes |
|---------|--------|-------|
| Headers | ✅ | All levels supported |
| Lists | ✅ | Ordered and unordered |
| Tables | ✅ | With header detection |
| Code | ✅ | Inline and blocks |

Try editing this text to see the parser in action!`;

  try {
    // Parse the markdown
    const document = await editor.parseMarkdown(markdown);
    console.log('Parsed document:', document);

    // Get document info
    const info = await editor.getDocumentInfo();
    console.log('Document info:', info);

    // Find table blocks
    const tableBlocks = DocumentUtils.findTableBlocks(document);
    console.log('Found tables:', tableBlocks.length);

    if (tableBlocks.length > 0) {
      const tableBlockIndex = tableBlocks[0].index;
      
      // Get cell content
      const cellContent = await editor.getTableCell(tableBlockIndex, 0, 0);
      console.log('Cell (0,0) content:', cellContent);

      // Navigate in table
      const newPosition = await editor.navigateTable(tableBlockIndex, 0, 0, 'right');
      console.log('Navigation result:', newPosition);

      if (newPosition) {
        const nextCellContent = await editor.getTableCell(
          tableBlockIndex, 
          newPosition.row, 
          newPosition.col
        );
        console.log('Next cell content:', nextCellContent);
      }
    }

    // Convert back to markdown
    const regeneratedMarkdown = await editor.toMarkdown();
    console.log('Regenerated markdown:', regeneratedMarkdown);

    // Count words
    const wordCount = DocumentUtils.countWords(document);
    console.log('Word count:', wordCount);

  } catch (error) {
    console.error('Error:', error);
  } finally {
    // Clean up
    unsubscribeDocumentChanged();
    unsubscribeSelectionChanged();
    await editor.destroy();
  }
}

// React component example
import React, { useState, useEffect, useRef } from 'react';
import { View, TextInput, Text, ScrollView, StyleSheet } from 'react-native';

interface MarkdownEditorProps {
  initialMarkdown?: string;
  onDocumentChange?: (document: any) => void;
}

export const MarkdownEditor: React.FC<MarkdownEditorProps> = ({
  initialMarkdown = '',
  onDocumentChange
}) => {
  const [markdown, setMarkdown] = useState(initialMarkdown);
  const [document, setDocument] = useState<any>(null);
  const [error, setError] = useState<string | null>(null);
  const editorRef = useRef<PinnipedEditor | null>(null);

  useEffect(() => {
    // Initialize editor
    const initEditor = async () => {
      try {
        const editor = new PinnipedEditor();
        await editor.initialize();
        editorRef.current = editor;

        // Subscribe to document changes
        editor.onDocumentChanged((event) => {
          const doc = JSON.parse(event.document);
          setDocument(doc);
          onDocumentChange?.(doc);
        });

        // Parse initial markdown
        if (initialMarkdown) {
          await editor.parseMarkdown(initialMarkdown);
        }
      } catch (err) {
        setError(`Failed to initialize editor: ${err}`);
      }
    };

    initEditor();

    // Cleanup
    return () => {
      editorRef.current?.destroy();
    };
  }, []);

  const handleMarkdownChange = async (text: string) => {
    setMarkdown(text);
    
    if (editorRef.current) {
      try {
        await editorRef.current.parseMarkdown(text);
        setError(null);
      } catch (err) {
        setError(`Parse error: ${err}`);
      }
    }
  };

  const renderDocument = () => {
    if (!document) return null;

    return (
      <ScrollView style={styles.preview}>
        <Text style={styles.previewTitle}>Preview</Text>
        {document.blocks.map((block: any, index: number) => {
          if (block.Paragraph) {
            return (
              <Text key={index} style={styles.paragraph}>
                {DocumentUtils.getPlainText(block.Paragraph)}
              </Text>
            );
          } else if (block.Header) {
            return (
              <Text 
                key={index} 
                style={[styles.header, { fontSize: 24 - block.Header.level * 2 }]}
              >
                {DocumentUtils.getPlainText(block.Header.text)}
              </Text>
            );
          } else if (block.Table) {
            return (
              <View key={index} style={styles.table}>
                {block.Table.rows.map((row: string[], rowIndex: number) => (
                  <View key={rowIndex} style={styles.tableRow}>
                    {row.map((cell: string, cellIndex: number) => (
                      <Text key={cellIndex} style={styles.tableCell}>
                        {cell}
                      </Text>
                    ))}
                  </View>
                ))}
              </View>
            );
          }
          return null;
        })}
      </ScrollView>
    );
  };

  return (
    <View style={styles.container}>
      <TextInput
        style={styles.input}
        multiline
        value={markdown}
        onChangeText={handleMarkdownChange}
        placeholder="Enter markdown here..."
        textAlignVertical="top"
      />
      {error && (
        <Text style={styles.error}>{error}</Text>
      )}
      {renderDocument()}
    </View>
  );
};

const styles = StyleSheet.create({
  container: {
    flex: 1,
    flexDirection: 'row',
  },
  input: {
    flex: 1,
    borderWidth: 1,
    borderColor: '#ccc',
    padding: 10,
    fontFamily: 'monospace',
    fontSize: 14,
  },
  preview: {
    flex: 1,
    padding: 10,
    backgroundColor: '#f5f5f5',
  },
  previewTitle: {
    fontSize: 18,
    fontWeight: 'bold',
    marginBottom: 10,
  },
  paragraph: {
    marginBottom: 10,
    lineHeight: 20,
  },
  header: {
    fontWeight: 'bold',
    marginBottom: 10,
    marginTop: 10,
  },
  table: {
    borderWidth: 1,
    borderColor: '#ddd',
    marginBottom: 10,
  },
  tableRow: {
    flexDirection: 'row',
    borderBottomWidth: 1,
    borderBottomColor: '#ddd',
  },
  tableCell: {
    flex: 1,
    padding: 8,
    borderRightWidth: 1,
    borderRightColor: '#ddd',
  },
  error: {
    color: 'red',
    padding: 10,
    backgroundColor: '#ffe6e6',
    borderWidth: 1,
    borderColor: '#ffcccc',
  },
});