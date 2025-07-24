import init, { parse, to_markdown } from './pkg/pinniped_wasm.js';

// Sample markdown for testing
const SAMPLE_MARKDOWN = `# 🦭 Pinniped Markdown Demo

Welcome to the **Pinniped** markdown parser! This parser is built in *Rust* and compiled to WebAssembly.

## Features Showcase

### Text Formatting
- **Bold text** using double asterisks
- *Italic text* using single asterisks  
- \`Inline code\` using backticks
- Mix them: **bold with *italic* inside**

### Lists

#### Unordered Lists
- First item
- Second item with **bold**
- Third item with \`code\`

#### Ordered Lists
1. Step one
2. Step two with *emphasis*
3. Step three

### Code Blocks

\`\`\`rust
fn main() {
    println!("Hello from Rust!");
    let parser = PinnipedParser::new();
    parser.parse("# Hello World");
}
\`\`\`

\`\`\`javascript
// JavaScript example
function parseMarkdown(text) {
    return pinnipedParse(text);
}
\`\`\`

### Blockquotes

> This is a blockquote with **bold** text.
> It can span multiple lines and include *formatting*.

> Another quote with \`inline code\`.

### Links and References

Check out [Google](https://google.com) for search, or visit [GitHub](https://github.com) for code.

### Tables

| Feature | Status | Notes |
|---------|--------|-------|
| Headers | ✅ | All levels supported |
| Lists | ✅ | Ordered and unordered |
| Tables | ✅ | With header detection |
| Code | ✅ | Inline and blocks |
| Links | ✅ | Standard markdown format |

## Performance Notes

The parser handles:
- Unicode characters: 🚀 🎉 ✨
- Large documents efficiently
- Perfect round-trip conversion
- Robust error handling

---

*Try editing this text to see the parser in action!*`;

async function run() {
    await init();
    
    const input = document.getElementById('markdown-input');
    const output = document.getElementById('output');
    const preview = document.getElementById('preview');
    const parseButton = document.getElementById('parse-button');
    const clearButton = document.getElementById('clear-button');
    const loadSampleButton = document.getElementById('load-sample');
    
    // Debug info elements
    const parseTimeElement = document.getElementById('parse-time');
    const charCountElement = document.getElementById('char-count');
    const blockCountElement = document.getElementById('block-count');
    const roundtripStatusElement = document.getElementById('roundtrip-status');
    
    function parseAndRender() {
        const markdown = input.value;
        const startTime = performance.now();
        
        try {
            const result = parse(markdown);
            const roundTrip = to_markdown(result);
            const endTime = performance.now();
            
            // Update outputs
            output.textContent = roundTrip;
            preview.innerHTML = astToHtml(result);
            
            // Update debug info
            parseTimeElement.textContent = `${(endTime - startTime).toFixed(2)}ms`;
            charCountElement.textContent = markdown.length.toLocaleString();
            
            // Count blocks (rough estimate)
            const blockCount = roundTrip.split('\n\n').filter(block => block.trim().length > 0).length;
            blockCountElement.textContent = blockCount;
            
            // Check round-trip accuracy
            const isRoundtripPerfect = markdown === roundTrip;
            roundtripStatusElement.textContent = isRoundtripPerfect ? '✅ Perfect' : '⚠️ Different';
            roundtripStatusElement.style.color = isRoundtripPerfect ? '#10b981' : '#f59e0b';
            
        } catch (error) {
            output.textContent = `Error: ${error.message}`;
            preview.innerHTML = `<div style="color: #ef4444; padding: 1rem;">Parse Error: ${error.message}</div>`;
            parseTimeElement.textContent = 'Error';
            roundtripStatusElement.textContent = '❌ Error';
            roundtripStatusElement.style.color = '#ef4444';
        }
    }
    
    function astToHtml(ast) {
        // Convert the parsed AST structure directly to HTML
        if (!ast || !Array.isArray(ast)) {
            return '<p style="color: #ef4444;">Invalid AST structure</p>';
        }
        
        return ast.map(block => {
            if (!block || typeof block !== 'object') {
                return '<p>Invalid block</p>';
            }
            
            // Handle different block types
            if (block.Paragraph) {
                return `<p>${renderInlineElements(block.Paragraph.elements)}</p>`;
            } else if (block.Header) {
                const level = block.Header.level;
                const content = renderInlineElements(block.Header.text.elements);
                return `<h${level}>${content}</h${level}>`;
            } else if (block.UnorderedList) {
                const items = block.UnorderedList.map(item => 
                    `<li>${renderInlineElements(item.elements)}</li>`
                ).join('');
                return `<ul>${items}</ul>`;
            } else if (block.OrderedList) {
                const items = block.OrderedList.map(item => 
                    `<li>${renderInlineElements(item.elements)}</li>`
                ).join('');
                return `<ol>${items}</ol>`;
            } else if (block.CodeBlock) {
                const language = block.CodeBlock.language || '';
                const code = escapeHtml(block.CodeBlock.code);
                return `<pre><code class="language-${language}">${code}</code></pre>`;
            } else if (block.Blockquote) {
                return `<blockquote>${renderInlineElements(block.Blockquote.elements)}</blockquote>`;
            } else if (block.Table) {
                return renderTable(block.Table);
            } else {
                return `<p>Unknown block type: ${JSON.stringify(Object.keys(block))}</p>`;
            }
        }).join('');
    }
    
    function renderInlineElements(elements) {
        if (!Array.isArray(elements)) {
            return 'Invalid inline elements';
        }
        
        return elements.map(element => {
            if (!element || typeof element !== 'object') {
                return 'Invalid element';
            }
            
            if (element.Text) {
                return escapeHtml(element.Text);
            } else if (element.Bold) {
                return `<strong>${renderInlineElements(element.Bold)}</strong>`;
            } else if (element.Italic) {
                return `<em>${renderInlineElements(element.Italic)}</em>`;
            } else if (element.Code) {
                return `<code>${escapeHtml(element.Code)}</code>`;
            } else if (element.Link) {
                return `<a href="${escapeHtml(element.Link.url)}" target="_blank">${escapeHtml(element.Link.text)}</a>`;
            } else {
                return `Unknown inline: ${JSON.stringify(Object.keys(element))}`;
            }
        }).join('');
    }
    
    function renderTable(table) {
        if (!table.rows || !Array.isArray(table.rows)) {
            return '<p>Invalid table</p>';
        }
        
        if (table.has_header && table.rows.length >= 2) {
            const headerRow = `<tr>${table.rows[0].map(cell => `<th>${escapeHtml(cell)}</th>`).join('')}</tr>`;
            const bodyRows = table.rows.slice(2).map(row => 
                `<tr>${row.map(cell => `<td>${escapeHtml(cell)}</td>`).join('')}</tr>`
            ).join('');
            return `<table>${headerRow}${bodyRows}</table>`;
        } else {
            const rows = table.rows.map(row => 
                `<tr>${row.map(cell => `<td>${escapeHtml(cell)}</td>`).join('')}</tr>`
            ).join('');
            return `<table>${rows}</table>`;
        }
    }

    function markdownToHtml(markdown) {
        // Simple markdown to HTML converter for preview
        // This is just for visual preview, not using our parser
        return markdown
            .split('\n\n')
            .map(block => {
                const trimmed = block.trim();
                if (!trimmed) return '';
                
                // Headers
                if (trimmed.startsWith('# ')) {
                    return `<h1>${escapeHtml(trimmed.slice(2))}</h1>`;
                } else if (trimmed.startsWith('## ')) {
                    return `<h2>${escapeHtml(trimmed.slice(3))}</h2>`;
                } else if (trimmed.startsWith('### ')) {
                    return `<h3>${escapeHtml(trimmed.slice(4))}</h3>`;
                }
                
                // Code blocks
                if (trimmed.startsWith('```')) {
                    const lines = trimmed.split('\n');
                    const language = lines[0].slice(3);
                    const code = lines.slice(1, -1).join('\n');
                    return `<pre><code class="language-${language}">${escapeHtml(code)}</code></pre>`;
                }
                
                // Blockquotes
                if (trimmed.split('\n').every(line => line.trim().startsWith('> '))) {
                    const quoteText = trimmed.split('\n')
                        .map(line => line.trim().slice(2))
                        .join('\n');
                    return `<blockquote>${formatInline(quoteText)}</blockquote>`;
                }
                
                // Lists
                if (trimmed.split('\n').every(line => line.trim().startsWith('- '))) {
                    const items = trimmed.split('\n')
                        .map(line => `<li>${formatInline(line.trim().slice(2))}</li>`)
                        .join('');
                    return `<ul>${items}</ul>`;
                }
                
                if (trimmed.split('\n').every(line => /^\d+\. /.test(line.trim()))) {
                    const items = trimmed.split('\n')
                        .map(line => {
                            const match = line.trim().match(/^\d+\. (.+)/);
                            return match ? `<li>${formatInline(match[1])}</li>` : '';
                        })
                        .join('');
                    return `<ol>${items}</ol>`;
                }
                
                // Tables
                if (trimmed.includes('|')) {
                    const rows = trimmed.split('\n');
                    if (rows.length >= 2) {
                        const headerRow = rows[0].split('|').map(cell => cell.trim()).filter(cell => cell);
                        const isHeaderTable = rows[1].split('|').every(cell => /^:?-+:?$/.test(cell.trim()));
                        
                        if (isHeaderTable) {
                            const header = `<tr>${headerRow.map(cell => `<th>${formatInline(cell)}</th>`).join('')}</tr>`;
                            const bodyRows = rows.slice(2).map(row => {
                                const cells = row.split('|').map(cell => cell.trim()).filter(cell => cell);
                                return `<tr>${cells.map(cell => `<td>${formatInline(cell)}</td>`).join('')}</tr>`;
                            }).join('');
                            return `<table>${header}${bodyRows}</table>`;
                        }
                    }
                }
                
                // Regular paragraph
                return `<p>${formatInline(trimmed)}</p>`;
            })
            .filter(html => html)
            .join('');
    }
    
    function formatInline(text) {
        return escapeHtml(text)
            // Bold
            .replace(/\*\*([^*]+)\*\*/g, '<strong>$1</strong>')
            // Italic  
            .replace(/\*([^*]+)\*/g, '<em>$1</em>')
            // Inline code
            .replace(/`([^`]+)`/g, '<code>$1</code>')
            // Links
            .replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank">$1</a>');
    }
    
    function escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }
    
    // Event listeners
    parseButton.addEventListener('click', parseAndRender);
    
    clearButton.addEventListener('click', () => {
        input.value = '';
        output.textContent = '';
        preview.innerHTML = '<p style="color: #64748b; font-style: italic;">Preview will appear here...</p>';
        charCountElement.textContent = '0';
        blockCountElement.textContent = '0';
        parseTimeElement.textContent = '-';
        roundtripStatusElement.textContent = '✅';
        roundtripStatusElement.style.color = '#10b981';
    });
    
    loadSampleButton.addEventListener('click', () => {
        input.value = SAMPLE_MARKDOWN;
        parseAndRender();
    });
    
    // Auto-parse on input (with debouncing)
    let timeout;
    input.addEventListener('input', () => {
        clearTimeout(timeout);
        timeout = setTimeout(parseAndRender, 300);
    });
    
    // Load sample on initial load
    input.value = SAMPLE_MARKDOWN;
    parseAndRender();
}

run();
