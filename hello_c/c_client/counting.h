// Generated by `wit-bindgen` 0.26.0. DO NOT EDIT!
#ifndef __BINDINGS_COUNTING_H
#define __BINDINGS_COUNTING_H
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif

#include <stdbool.h>
#include <stdint.h>

typedef struct counting_string_t {
  uint8_t *ptr;
  size_t len;
} counting_string_t;

typedef struct {
  uint8_t *ptr;
  size_t len;
} counting_list_u8_t;

typedef struct {
  bool is_some;
  uint8_t val;
} counting_option_u8_t;

typedef struct {
  bool is_err;
  union {
    uint64_t ok;
    counting_string_t err;
  } val;
} counting_result_u64_string_t;

// Imported Functions from `counting`
extern void counting_print(counting_string_t *msg);

// Exported Functions from `counting`
bool counting_count(counting_list_u8_t *bytes, uint8_t *maybe_target,
                    uint64_t *ret, counting_string_t *err);

// Helper Functions

void counting_list_u8_free(counting_list_u8_t *ptr);

void counting_option_u8_free(counting_option_u8_t *ptr);

void counting_result_u64_string_free(counting_result_u64_string_t *ptr);

// Transfers ownership of `s` into the string `ret`
void counting_string_set(counting_string_t *ret, const char *s);

// Creates a copy of the input nul-terminate string `s` and
// stores it into the component model string `ret`.
void counting_string_dup(counting_string_t *ret, const char *s);

// Deallocates the string pointed to by `ret`, deallocating
// the memory behind the string.
void counting_string_free(counting_string_t *ret);

#ifdef __cplusplus
}
#endif
#endif
