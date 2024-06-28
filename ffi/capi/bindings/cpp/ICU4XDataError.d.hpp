#ifndef ICU4XDataError_D_HPP
#define ICU4XDataError_D_HPP

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace capi {
    typedef enum ICU4XDataError {
      ICU4XDataError_Unknown = 0,
      ICU4XDataError_MarkerNotFound = 1,
      ICU4XDataError_IdentifierNotFound = 2,
      ICU4XDataError_InvalidRequest = 3,
      ICU4XDataError_FilteredResource = 4,
      ICU4XDataError_InconsistentData = 5,
      ICU4XDataError_Downcast = 6,
      ICU4XDataError_Deserialize = 7,
      ICU4XDataError_Custom = 8,
      ICU4XDataError_Io = 9,
    } ICU4XDataError;
}

class ICU4XDataError {
public:
  enum Value {
    Unknown = 0,
    MarkerNotFound = 1,
    IdentifierNotFound = 2,
    InvalidRequest = 3,
    FilteredResource = 4,
    InconsistentData = 5,
    Downcast = 6,
    Deserialize = 7,
    Custom = 8,
    Io = 9,
  };

  ICU4XDataError() = default;
  // Implicit conversions between enum and ::Value
  constexpr ICU4XDataError(Value v) : value(v) {}
  constexpr operator Value() const { return value; }
  // Prevent usage as boolean value
  explicit operator bool() const = delete;

  inline capi::ICU4XDataError AsFFI() const;
  inline static ICU4XDataError FromFFI(capi::ICU4XDataError c_enum);
private:
    Value value;
};


#endif // ICU4XDataError_D_HPP
