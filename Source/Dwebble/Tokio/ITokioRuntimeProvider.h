#pragma once

#include "rust/cxx.h"

class ITokioRuntimeProvider
{
public:
	virtual ~ITokioRuntimeProvider() = default;

	virtual void SpawnAsync(rust::Fn<void()> Work, rust::Fn<void()> OnDoneGameThread) = 0;
	// virtual void SpawnBlocking(rust::Fn<void()> Work, rust::Fn<void()> OnDoneGameThread) = 0;
};
