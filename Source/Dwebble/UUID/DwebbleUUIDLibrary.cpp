// Copyright 2019-Present tarnishablec. All Rights Reserved.


#include "DwebbleUUIDLibrary.h"

#include "dwebble/ffi/index.rs.h"

void UDwebbleUUIDLibrary::Generate_UUID_V7(FGuid& Result)
{
    std::array<uint8_t, 16>& Buffer = *reinterpret_cast<std::array<uint8_t, 16>*>(&Result);
    const auto Str = dwebble_cxx::uuid::fill_uuid_v7_into_guid(Buffer);
    dwebble_cxx::string::free_rust_string(Str);
}
