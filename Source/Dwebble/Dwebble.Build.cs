// Copyright Epic Games, Inc. All Rights Reserved.

using System.IO;
using System.Diagnostics;
using EpicGames.Core;
using UnrealBuildTool;

public class Dwebble : ModuleRules
{
	public Dwebble(ReadOnlyTargetRules Target) : base(Target)
	{
		PCHUsage = PCHUsageMode.UseExplicitOrSharedPCHs;
		// Type = ModuleType.CPlusPlus;

		PublicAdditionalLibraries.Add(
			Path.Combine(PluginDirectory, @"target\x86_64-pc-windows-msvc\release\dwebble.lib")
		);

		PublicIncludePaths.Add(
			Path.Combine(PluginDirectory, "Bindgens")
		);

		if (Target.WindowsPlatform.Compiler.IsMSVC())
		{
			PublicSystemLibraries.Add("kernel32.lib");
			PublicSystemLibraries.Add("advapi32.lib");
			PublicSystemLibraries.Add("bcrypt.lib");
			PublicSystemLibraries.Add("ntdll.lib");
			PublicSystemLibraries.Add("userenv.lib");
			PublicSystemLibraries.Add("ws2_32.lib");
			PublicSystemLibraries.Add("msvcrt.lib");
		}

		const string Command = "cargo";
		const string Arguments = "build --release --target x86_64-pc-windows-msvc";

		Log.TraceInformationOnce("Dwebble Plugin: running Rust Cargo Build...");

		var ProcStartInfo = new ProcessStartInfo(Command, Arguments)
		{
			WorkingDirectory = PluginDirectory, // 设置工作目录为 Rust 项目根目录
			RedirectStandardOutput = true, // 重定向标准输出
			RedirectStandardError = true, // 重定向标准错误
			UseShellExecute = false, // 不使用 shell 执行，以便重定向输出
			CreateNoWindow = true
		};

		try
		{
			using var Proc = new Process();
			Proc.StartInfo = ProcStartInfo;
			Proc.Start(); // 启动进程

			// 读取输出以避免死锁 (对于长时间运行的进程很重要)
			var Output = Proc.StandardOutput.ReadToEnd();
			var Error = Proc.StandardError.ReadToEnd();

			Proc.WaitForExit(); // 等待进程完成

			// 检查 Cargo 命令的退出码
			if (Proc.ExitCode != 0)
			{
				// 如果 Cargo 构建失败，抛出 BuildException，中断 Unreal 编译
				throw new BuildException(
					$"Rust cargo build failed with exit code {Proc.ExitCode}.\n" +
					$"Output:\n{Output}\n" +
					$"Error:\n{Error}"
				);
			}

			Log.TraceInformationOnce($"Rust cargo build completed successfully.\nOutput:\n{Output}");
		}
		catch (System.Exception Ex)
		{
			// 捕获执行进程时可能发生的异常
			throw new BuildException($"Failed to run Rust cargo build: {Ex.Message}");
		}

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