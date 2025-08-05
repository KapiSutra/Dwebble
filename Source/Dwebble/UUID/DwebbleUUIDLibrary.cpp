// Copyright 2019-Present tarnishablec. All Rights Reserved.


#include "DwebbleUUIDLibrary.h"

#include <dwebble_rs.h>

void UDwebbleUUIDLibrary::Generate_UUID_V7(FGuid& Result)
{
    dwebble_rs::fill_uuid_v7_into_guid(&Result);
}
