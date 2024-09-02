#include <catch2/catch_test_macros.hpp>

#include "lib.hpp"

TEST_CASE("Name is project", "[library]")
{
  auto const lib = library {};
  REQUIRE(lib.name == "project");
}
