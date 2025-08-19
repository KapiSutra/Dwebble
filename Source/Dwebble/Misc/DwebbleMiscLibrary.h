// Copyright 2019-Present tarnishablec. All Rights Reserved.

// ReSharper disable CppUE4CodingStandardNamingViolationWarning
#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "rust/cxx.h"
#include "DwebbleMiscLibrary.generated.h"

/**
 * 
 */
UCLASS()
class DWEBBLE_API UDwebbleMiscLibrary : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()

public:
	UFUNCTION(BlueprintCallable)
	static int32 LocalFreeIpv4Port();
};


namespace dwebble_cxx::string
{
	inline rust::String ToRustString(const FString& In)
	{
		const auto Utf8 = StringCast<UTF8CHAR>(*In);
		return rust::String(std::string(
			reinterpret_cast<const char*>(Utf8.Get()),
			Utf8.Length()));
	}

	inline FString ToFString(const rust::String& In)
	{
		auto Copy = In;
		const UTF8CHAR* Src = reinterpret_cast<const UTF8CHAR*>(Copy.c_str());
		const auto Conv = StringCast<TCHAR>(Src);
		return FString(Conv.Get());
	}
}
