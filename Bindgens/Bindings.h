#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

namespace dwebble {

extern "C" {

void fill_uuid_v7_into_buffer(void *buffer);

}  // extern "C"

}  // namespace dwebble
