#include "cxx-enum-namespace/src/lib.rs.h"

namespace example {
std::unique_ptr<ExampleStruct> create_example_struct() {
  return std::make_unique<ExampleStruct>();
}
} // namespace example