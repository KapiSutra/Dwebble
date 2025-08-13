// Copyright 2019-Present tarnishablec. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
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
