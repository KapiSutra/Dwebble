// Copyright 2019-Present tarnishablec. All Rights Reserved.

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


// ReSharper disable once CppUE4CodingStandardNamingViolationWarning
namespace dwebble_cxx
{
	inline rust::String ToRustString(const FString& In)
	{
		const FTCHARToUTF8 Conv(*In); // 生命周期到本作用域末
		// 复制到 std::string，避免使用临时指针悬空
		const std::string Utf8(Conv.Get(), Conv.Length());
		return rust::String(Utf8); // 或：rust::String(Utf8.c_str())
	}

	inline FString ToFString(rust::String& In)
	{
		return UTF8_TO_TCHAR(In.c_str());
	}
}
