#pragma once

#include "rust/cxx.h"
#include "rust/cxx_async.h"

// ReSharper disable once CppUE4CodingStandardNamingViolationWarning
namespace dwebble_cxx::oidc
{
	struct FOidcResult;
}

CXXASYNC_DEFINE_FUTURE(rust::String, RustFutureString);

CXXASYNC_DEFINE_FUTURE(dwebble_cxx::oidc::FOidcResult, RustFutureOidcResult);
