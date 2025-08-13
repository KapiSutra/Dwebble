// Copyright 2019-Present tarnishablec. All Rights Reserved.


#include "DwebbleMiscLibrary.h"

#include "dwebble/ffi/index.rs.h"

int32 UDwebbleMiscLibrary::LocalFreeIpv4Port()
{
    return dwebble_cxx::port::free_local_ipv4_port();
}
