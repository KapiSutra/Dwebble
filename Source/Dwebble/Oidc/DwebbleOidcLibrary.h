// Copyright 2019-Present tarnishablec. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "DwebbleOidcTypes.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "DwebbleOidcLibrary.generated.h"

/**
 * 
 */
UCLASS()
class DWEBBLE_API UDwebbleOidcLibrary : public UBlueprintFunctionLibrary
{
	GENERATED_BODY()

public:
	UFUNCTION(BlueprintCallable)
	static FDwebbleOidcParams SetOidcParamClientSecret(
		const FDwebbleOidcParams& Params,
		const FString Secret
	);
};
