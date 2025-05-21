// Copyright 2019-Present tarnishablec. All Rights Reserved.


#include "DwebbleSubsystem.h"
#include "dwebble/src/lib.rs.h"

void UDwebbleSubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
    Super::Initialize(Collection);

    // UE_LOG(LogTemp, Warning, TEXT("UDwebbleSubsystem::Initialize with %d"), dwebble::test());
}
