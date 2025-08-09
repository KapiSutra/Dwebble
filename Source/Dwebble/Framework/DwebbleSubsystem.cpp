// Copyright 2019-Present tarnishablec. All Rights Reserved.


#include "DwebbleSubsystem.h"

// #include "dwebble/ffi/index.rs.h"

void UDwebbleSubsystem::Initialize(FSubsystemCollectionBase& Collection)
{
    Super::Initialize(Collection);

    // std::array<uint8_t, 16> Arr;
    // FGuid NewGuid;
    // const auto RustUuidStringPtr = dwebble_cxx::uuid::fill_uuid_v7_into_guid(Arr);
    // FMemory::Memcpy(&NewGuid, Arr.data(), 16);
    // UE_LOG(LogTemp, Warning, TEXT("UDwebbleSubsystem::Initialize with %s"),
    //        *NewGuid.ToString(EGuidFormats::DigitsWithHyphensLower));
    //
    // if (RustUuidStringPtr != nullptr)
    // {
    //     const FString ResultString = FString(UTF8_TO_TCHAR(RustUuidStringPtr));
    //     UE_LOG(LogTemp, Warning, TEXT("%s"), *ResultString);
    //     dwebble_cxx::string::free_rust_string(RustUuidStringPtr);
    // }


    // FGuid NewGuid;
    // void* GuidDataPtr = &NewGuid;
    // const auto RustUuidStringPtr = dwebble_rs::fill_uuid_v7_into_guid(GuidDataPtr);
    //
    // if (RustUuidStringPtr != nullptr)
    // {
    //     const FString ResultString = FString(UTF8_TO_TCHAR(RustUuidStringPtr));
    //     UE_LOG(LogTemp, Warning, TEXT("%s"), *ResultString);
    //     dwebble_rs::free_rust_string(RustUuidStringPtr);
    // }
    //
    // UE_LOG(LogTemp, Warning, TEXT("UDwebbleSubsystem::Initialize with uuid ToString %s"),
    //        *NewGuid.ToString( EGuidFormats::DigitsWithHyphensLower));

    // UE_LOG(LogTemp, Warning, TEXT("UDwebbleSubsystem::Initialize with %d"), dwebble::test() + 1);
    // UE_LOG(LogTemp, Warning, TEXT("UDwebbleSubsystem::Initialize with %d"), dwebble::test());
}
