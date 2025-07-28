// Copyright 2019-Present tarnishablec. All Rights Reserved.


#include "DwebbleSubsystem.h"
#include "Bindings.h"
// #include "dwebble/src/ffi.rs.h"

void UDwebbleSubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
    Super::Initialize(Collection);

    FGuid NewGuid;
    void* GuidDataPtr = &NewGuid;
    const auto RustUuidStringPtr = dwebble::fill_uuid_v7_into_guid(GuidDataPtr);

    if (RustUuidStringPtr != nullptr)
    {
        const FString ResultString = FString(UTF8_TO_TCHAR(RustUuidStringPtr));
        UE_LOG(LogTemp, Warning, TEXT("%s"), *ResultString);
        dwebble::free_rust_string(RustUuidStringPtr);
    }

    UE_LOG(LogTemp, Warning, TEXT("UDwebbleSubsystem::Initialize with uuid ToString %s"),
           *NewGuid.ToString( EGuidFormats::DigitsWithHyphensLower));

    // UE_LOG(LogTemp, Warning, TEXT("UDwebbleSubsystem::Initialize with %d"), dwebble::test() + 1);
    // UE_LOG(LogTemp, Warning, TEXT("UDwebbleSubsystem::Initialize with %d"), dwebble::test());
}
