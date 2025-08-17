// Copyright 2019-Present tarnishablec. All Rights Reserved.


#include "DwebbleOidcLibrary.h"

FDwebbleOidcParams UDwebbleOidcLibrary::SetOidcParamClientSecret(const FDwebbleOidcParams& Params,
                                                                 const FString Secret)
{
	FDwebbleOidcParams Result = Params;
	Result.ClientSecret.Emplace(Secret);
	return Result;
}
