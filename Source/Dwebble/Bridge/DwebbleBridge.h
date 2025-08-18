// ReSharper disable CppUE4CodingStandardNamingViolationWarning
#pragma once

#include "rust/cxx.h"
#include "rust/cxx_async.h"

namespace dwebble_cxx
{
	namespace oidc
	{
		struct FOidcResult;
	}
}

CXXASYNC_DEFINE_FUTURE(rust::String, RustFutureString);

CXXASYNC_DEFINE_FUTURE(dwebble_cxx::oidc::FOidcResult, RustFutureOidcResult);
