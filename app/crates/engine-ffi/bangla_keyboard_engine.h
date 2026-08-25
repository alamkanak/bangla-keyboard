#ifndef BANGLA_KEYBOARD_ENGINE_H
#define BANGLA_KEYBOARD_ENGINE_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Initialize the engine with the path to the data directory.
 * Returns 0 on success, -1 on failure.
 *
 * # Safety
 * `data_dir` must be a valid, null-terminated UTF-8 C string.
 */
int32_t bk_engine_init(const char *data_dir);

/**
 * Shut down the engine and free resources.
 */
void bk_engine_shutdown(void);

/**
 * Set the layout mode: 0 = Phonetic, 1 = UniBijoy.
 */
void bk_set_mode(int32_t mode);

/**
 * Get the current layout mode: 0 = Phonetic, 1 = UniBijoy.
 */
int32_t bk_get_mode(void);

/**
 * Handle a key press. Returns: 0 = Commit, 1 = UpdatePreview, 2 = Nothing, 3 = CommitReplaceLast.
 */
int32_t bk_handle_key(char key, bool shift);

/**
 * Handle backspace. Returns: 0 = Commit, 1 = UpdatePreview, 2 = Nothing.
 */
int32_t bk_handle_backspace(void);

/**
 * Handle enter/return. Returns the committed text or NULL.
 * Caller must free the returned string with bk_free_string.
 */
char *bk_handle_enter(void);

/**
 * Handle space. Returns the committed text or NULL.
 * Caller must free the returned string with bk_free_string.
 */
char *bk_handle_space(void);

/**
 * Get the current preview text. Caller must free with bk_free_string.
 */
char *bk_get_preview(void);

/**
 * Check if the engine is currently composing text.
 */
bool bk_is_composing(void);

/**
 * Reset the composing state.
 */
void bk_reset(void);

/**
 * Get the number of candidates.
 */
int32_t bk_candidate_count(void);

/**
 * Get a candidate by index. Caller must free with bk_free_string.
 */
char *bk_get_candidate(int32_t index);

/**
 * Select a candidate by index. Returns the committed text or NULL.
 * Caller must free the returned string with bk_free_string.
 */
char *bk_select_candidate(int32_t index);

/**
 * Free a string returned by any bk_* function.
 *
 * # Safety
 * `s` must be a pointer previously returned by a `bk_*` function, or null.
 */
void bk_free_string(char *s);

#endif  /* BANGLA_KEYBOARD_ENGINE_H */
