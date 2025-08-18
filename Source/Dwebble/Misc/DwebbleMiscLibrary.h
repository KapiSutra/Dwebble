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
		const FTCHARToUTF8 Conv(*In);
		const std::string Utf8(Conv.Get(), Conv.Length());
		return rust::String(Utf8);
	}

	inline FString ToFString(rust::String& In)
	{
		return UTF8_TO_TCHAR(In.c_str());
	}
}
