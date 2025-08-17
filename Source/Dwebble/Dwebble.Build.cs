// Copyright Epic Games, Inc. All Rights Reserved.

using System.IO;
using UnrealBuildTool;

public class Dwebble : ModuleRules
{
	public Dwebble(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
		// Type = ModuleType.CPlusPlus;

		var CargoTarget = GetCargoTargetTriple();

		// if (Target.WindowsPlatform.Compiler.IsMSVC())
		if (Target.Platform == UnrealTargetPlatform.Win64)
		{
			PublicSystemLibraries.Add("kernel32.lib");
			PublicSystemLibraries.Add("advapi32.lib");
			PublicSystemLibraries.Add("bcrypt.lib");
			PublicSystemLibraries.Add("ntdll.lib");
			PublicSystemLibraries.Add("userenv.lib");
			PublicSystemLibraries.Add("ws2_32.lib");
			PublicSystemLibraries.Add("msvcrt.lib");
			PublicSystemLibraries.Add("Shlwapi.lib");
		}

		#region cbindgen

		// PublicIncludePaths.Add(
		// 	Path.Combine(PluginDirectory, "cbindgen")
		// );

		#endregion

		#region cxx

		PublicIncludePaths.Add(
			Path.Combine(PluginDirectory, @$"target\{CargoTarget}\cxxbridge")
			// Path.Combine(PluginDirectory, @$"target\cxxbridge")
		);

		# endregion

		var CargoProfile = GetCargoProfile();

		#region cargo

		// const string Command = "cargo";
		// var Arguments = string.Equals(CargoProfile, "release", StringComparison.OrdinalIgnoreCase)
		// 	? $"build --release --target {CargoTarget}"
		// 	: $"build --target {CargoTarget}";
		//
		// Log.TraceInformationOnce("Dwebble Plugin: running Rust Cargo Build...");
		//
		// Log.TraceInformationOnce($"{Command} {Arguments}");
		//
		// var ProcStartInfo = new ProcessStartInfo()
		// {
		// 	FileName = Command,
		// 	Arguments = Arguments,
		// 	WorkingDirectory = PluginDirectory,
		// 	RedirectStandardOutput = true,
		// 	RedirectStandardError = true,
		// 	UseShellExecute = false,
		// 	CreateNoWindow = true
		// };
		//
		// try
		// {
		// 	using var Proc = new Process();
		// 	Proc.StartInfo = ProcStartInfo;
		//
		// 	Proc.OutputDataReceived += (sender, args) =>
		// 	{
		// 		if (!string.IsNullOrEmpty(args.Data))
		// 			Log.TraceInformationOnce($"[Cargo stdout] {args.Data}");
		// 	};
		//
		// 	Proc.ErrorDataReceived += (sender, args) =>
		// 	{
		// 		if (!string.IsNullOrEmpty(args.Data))
		// 			Log.TraceInformationOnce($"[Cargo stderr] {args.Data}");
		// 	};
		//
		// 	Proc.Start();
		//
		// 	Proc.BeginOutputReadLine();
		// 	Proc.BeginErrorReadLine();
		//
		// 	Proc.WaitForExit();
		//
		// 	if (Proc.ExitCode != 0)
		// 	{
		// 		throw new BuildException($"Rust cargo build failed with exit code {Proc.ExitCode}");
		// 	}
		//
		// 	Log.TraceInformationOnce("Rust cargo build completed successfully.");
		// }
		// catch (System.Exception Ex)
		// {
		// 	throw new BuildException($"Failed to run Rust cargo build: {Ex.Message}");
		// }

		#endregion

		// PublicAdditionalLibraries.Add(
		// 	Path.Combine(PluginDirectory, @$"target\{CargoTarget}\release\dwebble.dll.lib")
		// );

		// PublicAdditionalLibraries.Add(
		// 	Path.Combine(PluginDirectory, @$"cxx\dwebble_cxx.lib")
		// );

		var LibFileName = GetLibFileName();

		PublicAdditionalLibraries.Add(
			Path.Combine(PluginDirectory, @$"target\{CargoTarget}\{CargoProfile}\{LibFileName}")
		);

		PublicIncludePaths.Add(
			Path.Combine(PluginDirectory, $@"crates\cxx-async\cxx-async\include"));

		// PublicDelayLoadDLLs.Add(
		// 	Path.Combine(PluginDirectory, @$"target\{CargoTarget}\{CargoProfile}\dwebble.dll")
		// );

		PrivateIncludePaths.AddRange(
			new string[]
			{
				// ... add another private include paths required here ...
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
				"UE5Coro"
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


	private string GetCargoTargetTriple()
	{
		if (Target.Platform == UnrealTargetPlatform.Win64)
		{
			return "x86_64-pc-windows-msvc";
		}

		if (Target.Platform == UnrealTargetPlatform.Linux)
		{
			return "x86_64-unknown-linux-musl";
		}

		throw new BuildException($"Unsupported Unreal platform for Rust cargo build: {Target.Platform}");
	}

	private string GetLibFileName()
	{
		if (Target.Platform == UnrealTargetPlatform.Win64)
		{
			if (Target.WindowsPlatform.Compiler.IsMSVC())
			{
				return "dwebble.lib";
			}

			if (Target.WindowsPlatform.Compiler.IsClang())
			{
				// return "libdwebble.a";
				return "dwebble.lib";
			}
		}

		if (Target.Platform == UnrealTargetPlatform.Linux)
		{
			return "dwebble.a"; // Linux uses .a for static libraries
		}

		throw new BuildException($"Unsupported Unreal platform for Rust cargo build: {Target.Platform}");
	}

	private string GetCargoProfile()
	{
		return Target.Configuration == UnrealTargetConfiguration.DebugGame ? "debug" : "release";
	}
}