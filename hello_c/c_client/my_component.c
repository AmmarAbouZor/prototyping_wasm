#include "counting.h"
#include <string.h>

// Exported function from `counting`
bool counting_count(counting_list_u8_t *bytes, uint8_t *maybe_target,
                    uint64_t *ret, counting_string_t *err) {
  // Create the message
  const char *message = "Hello from C Plugin";
  counting_string_t msg;
  msg.ptr = (uint8_t *)message;
  msg.len = strlen(message);

  // Call the imported `counting_print` function
  counting_print(&msg);

  // Set the return value
  *ret = 200;

  // Return true to indicate success
  return true;
}
