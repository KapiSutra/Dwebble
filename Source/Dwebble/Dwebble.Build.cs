// Copyright Epic Games, Inc. All Rights Reserved.

using System.IO;
using UnrealBuildTool;

public class Dwebble : ModuleRules
{
	public Dwebble(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
		Type = ModuleType.CPlusPlus;

		PublicAdditionalLibraries.AddRange(
		[
			// Path.Combine(PluginDirectory, @"Bindings\dwebble.lib"),
			// Path.Combine(PluginDirectory, @"target\debug\dwebble.lib"),
		]);

		// PublicPreBuildLibraries.Add(Path.Combine(PluginDirectory, @"Bindings\dwebble.lib"));


		PublicIncludePaths.AddRange(
			new string[]
			{
				// ... add public include paths required here ...
				// Path.Combine(PluginDirectory, @"target\cxxbridge"),
				// Path.Combine(PluginDirectory, "Bindgens"),
			}
		);


		PrivateIncludePaths.AddRange(
			new string[]
			{
				// ... add other private include paths required here ...
			}
		);


		PublicDependencyModuleNames.AddRange(
			new[]
			{
				"Core",
				// ... add other public dependencies that you statically link with here ...
			}
		);


		PrivateDependencyModuleNames.AddRange(
			new[]
			{
				"CoreUObject",
				"Engine",
				"Slate",
				"SlateCore",
				// ... add private dependencies that you statically link with here ...	
			}
		);


		DynamicallyLoadedModuleNames.AddRange(
			new string[]
			{
				// ... add any modules that your module loads dynamically here ...
			}
		);
	}
}