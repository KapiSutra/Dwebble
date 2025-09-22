// Copyright 2019-Present tarnishablec. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "Misc/Optional.h"
#include "DwebbleOidcTypes.generated.h"

USTRUCT(BlueprintType)
struct DWEBBLE_API FDwebbleOidcParams
{
	GENERATED_BODY()

	UPROPERTY(BlueprintReadWrite)
	FString Issuer;

	UPROPERTY(BlueprintReadWrite)
	FString ClientId;

	// UPROPERTY(BlueprintReadWrite)
	TOptional<FString> ClientSecret;

	UPROPERTY(BlueprintReadWrite)
	int32 LoopbackPort = 0;

	UPROPERTY(BlueprintReadWrite)
	FString LoopbackRoute;
};


USTRUCT(BlueprintType)
struct DWEBBLE_API FDwebbleOidcResult
{
	GENERATED_BODY()

	UPROPERTY(BlueprintReadOnly)
	bool bSuccess = false;

	UPROPERTY(BlueprintReadOnly)
	FString AccessToken;

	UPROPERTY(BlueprintReadOnly)
	FString ErrorMessage;
};
