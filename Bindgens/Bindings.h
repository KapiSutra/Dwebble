#pragma once

#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace dwebble {

extern "C" {

/// Frees memory allocated by Rust for strings passed to C++.
/// This MUST be called from C++ when the string is no longer needed.
void free_rust_string(char *s);

char *fill_uuid_v7_into_guid(void *buffer);

}  // extern "C"

}  // namespace dwebble
