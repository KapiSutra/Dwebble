// Copyright 2019-Present tarnishablec. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "Kismet/BlueprintFunctionLibrary.h"
#include "DwebbleUUIDLibrary.generated.h"

/**
 * 
 */
UCLASS()
class DWEBBLE_API UDwebbleUUIDLibrary : public UBlueprintFunctionLibrary
{
    GENERATED_BODY()

public:
    UFUNCTION(BlueprintPure)
    static void Generate_UUID_V7(FGuid& Result);
};
