#ifndef RUST_FLUTTER_FFI_H
#define RUST_FLUTTER_FFI_H

/* Generated with cbindgen:0.27.0 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

uintptr_t sum(uintptr_t a, uintptr_t b);

uintptr_t sum_async(uintptr_t a, uintptr_t b);

char *sum_str(const char *a, const char *b);

void main_engine(void);

void free_c_string(char *s);

#endif  /* RUST_FLUTTER_FFI_H */
