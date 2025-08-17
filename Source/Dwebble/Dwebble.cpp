// Copyright Epic Games, Inc. All Rights Reserved.

#include "Dwebble.h"


#define LOCTEXT_NAMESPACE "FDwebbleModule"

void FDwebbleModule::StartupModule()
{
	// This code will execute after your module is loaded into memory; the exact timing is specified in the .uplugin file per-module
	// const FString PluginDir = IPluginManager::Get().FindPlugin("Dwebble")->GetBaseDir();
	// const FString DllPath = PluginDir + "/target/x86_64-pc-windows-msvc/release/" + "dwebble.dll";
	// FPlatformProcess::GetDllHandle(*DllPath);
	
}

void FDwebbleModule::ShutdownModule()
{
	// This function may be called during shutdown to clean up your module.  For modules that support dynamic reloading,
	// we call this function before unloading the module.
}


#undef LOCTEXT_NAMESPACE

IMPLEMENT_MODULE(FDwebbleModule, Dwebble)
