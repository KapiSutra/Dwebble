// Copyright 2019-Present tarnishablec. All Rights Reserved.

#include "DwebbleOidcSubsystem.h"
#include "dwebble/ffi/index.rs.h"
#include "Dwebble/Misc/DwebbleMiscLibrary.h"

FVoidCoroutine UDwebbleOidcSubsystem::BrowserOidc(const FDwebbleOidcParams Params, FDwebbleOidcResult &Result, FLatentActionInfo LatentInfo)
{
	auto Ret = co_await dwebble_cxx::oidc::oidc_access_token(
			dwebble_cxx::ToRustString(Params.Issuer),
			dwebble_cxx::ToRustString(Params.ClientId),
			dwebble_cxx::ToRustString(Params.ClientSecret.Get("")),
			Params.LoopbackPort,
			dwebble_cxx::ToRustString(Params.LoopbackRoute));

	if (Ret.empty())
	{
		Result.bSuccess = false;
	}
	else
	{
		Result.bSuccess = true;
		Result.AccessToken = dwebble_cxx::ToFString(Ret);
	}

	co_return;
}
