#ifndef PINNIPED_CORE_H
#define PINNIPED_CORE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/**
 * Parse markdown text and return document as JSON string.
 * 
 * @param input Null-terminated markdown text
 * @return JSON string representing the parsed document, or error JSON.
 *         Must be freed with pinniped_free_string().
 */
char* pinniped_parse_markdown(const char* input);

/**
 * Convert document JSON back to markdown text.
 * 
 * @param document_json Null-terminated JSON string representing a document
 * @return Markdown text string.
 *         Must be freed with pinniped_free_string().
 */
char* pinniped_to_markdown(const char* document_json);

/**
 * Navigate within a table at the specified block index.
 * 
 * @param document_json Null-terminated JSON string representing a document
 * @param block_index Zero-based index of the table block in the document
 * @param current_row Current row position (0-based, excluding separator)
 * @param current_col Current column position (0-based)
 * @param direction Navigation direction: 0=up, 1=down, 2=left, 3=right
 * @return JSON string with new position: {"row": int, "col": int, "valid": bool}
 *         Must be freed with pinniped_free_string().
 */
char* pinniped_table_navigate(
    const char* document_json,
    int32_t block_index,
    int32_t current_row,
    int32_t current_col,
    int32_t direction
);

/**
 * Get cell content at specified position within a table.
 * 
 * @param document_json Null-terminated JSON string representing a document
 * @param block_index Zero-based index of the table block in the document
 * @param row Row position (0-based, excluding separator)
 * @param col Column position (0-based)
 * @return JSON string with cell content: {"content": "cell text"}
 *         Must be freed with pinniped_free_string().
 */
char* pinniped_table_get_cell(
    const char* document_json,
    int32_t block_index,
    int32_t row,
    int32_t col
);

/**
 * Free a string returned by any Pinniped function.
 * 
 * @param s String pointer returned by a Pinniped function
 */
void pinniped_free_string(char* s);

#ifdef __cplusplus
}
#endif

#endif /* PINNIPED_CORE_H */