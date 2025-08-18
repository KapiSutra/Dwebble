// Copyright 2019-Present tarnishablec. All Rights Reserved.

#include "DwebbleOidcSubsystem.h"
#include "dwebble/ffi/index.rs.h"
#include "Dwebble/Misc/DwebbleMiscLibrary.h"

FVoidCoroutine UDwebbleOidcSubsystem::BrowserOidc(const FDwebbleOidcParams Params, FDwebbleOidcResult& Result,
                                                  FLatentActionInfo LatentInfo)
{
	auto [success, access_token, refresh_token, error_message] = co_await dwebble_cxx::oidc::browser_oidc(
		dwebble_cxx::ToRustString(Params.Issuer),
		dwebble_cxx::ToRustString(Params.ClientId),
		dwebble_cxx::ToRustString(Params.ClientSecret.Get("")),
		Params.LoopbackPort,
		dwebble_cxx::ToRustString(Params.LoopbackRoute));

	Result.bSuccess = success;
	Result.AccessToken = dwebble_cxx::ToFString(access_token);
	Result.ErrorMessage = dwebble_cxx::ToFString(error_message);
	RefreshToken.Emplace(dwebble_cxx::ToFString(refresh_token));
	co_return;
}
