// Copyright Epic Games, Inc. All Rights Reserved.

using System.IO;
using EpicGames.Core;
using UnrealBuildTool;

public class Dwebble : ModuleRules
{
	public Dwebble(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
		// Type = ModuleType.CPlusPlus;

		PublicAdditionalLibraries.AddRange(
		[
			// Path.Combine(PluginDirectory, @"Bindings\dwebble.lib"),
			Path.Combine(PluginDirectory, @"target\debug\dwebble.lib"),
		]);

		PublicIncludePaths.AddRange(
			[
				// ... add public include paths required here ...
				// Path.Combine(PluginDirectory, @"target\cxxbridge"),
				Path.Combine(PluginDirectory, "Bindgens")
			]
		);

		if (Target.WindowsPlatform.Compiler.IsMSVC())
		{
			PublicSystemLibraries.Add("kernel32.lib");
			PublicSystemLibraries.Add("advapi32.lib");
			PublicSystemLibraries.Add("bcrypt.lib");
			PublicSystemLibraries.Add("kernel32.lib");
			PublicSystemLibraries.Add("ntdll.lib");
			PublicSystemLibraries.Add("userenv.lib");
			PublicSystemLibraries.Add("ws2_32.lib");
			PublicSystemLibraries.Add("kernel32.lib");
			PublicSystemLibraries.Add("ws2_32.lib");
			PublicSystemLibraries.Add("kernel32.lib");
			PublicSystemLibraries.Add("ntdll.lib");
			PublicSystemLibraries.Add("kernel32.lib");
			PublicSystemLibraries.Add("msvcrt.lib");
		}


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