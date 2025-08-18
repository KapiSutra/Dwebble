// Copyright 2019-Present tarnishablec. All Rights Reserved.

#pragma once

#include "CoreMinimal.h"
#include "DwebbleOidcTypes.h"
#include "Subsystems/LocalPlayerSubsystem.h"
#include "UE5Coro.h"
#include "DwebbleOidcSubsystem.generated.h"

/**
 *
 */
UCLASS()
class DWEBBLE_API UDwebbleOidcSubsystem : public ULocalPlayerSubsystem
{
	GENERATED_BODY()

protected:
	TOptional<FString> RefreshToken;

public:
	UFUNCTION(BlueprintCallable, Category = "Tauros | Oidc", meta = (Latent, LatentInfo = LatentInfo))
	FVoidCoroutine BrowserOidc(const FDwebbleOidcParams Params, FDwebbleOidcResult& Result,
	                           FLatentActionInfo LatentInfo);
};
