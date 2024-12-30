//
// Created by bspeice on 12/30/24.
//
#pragma once

#include <memory>

namespace example_enum {
enum class ExampleEnum: int8_t {
  A,
  B
};
} // namespace example_enum

namespace example {
struct ExampleStruct;
std::unique_ptr<ExampleStruct> create_example_struct();
} // namespace example
