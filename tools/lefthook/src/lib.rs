use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "lefthook";

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::CommandLine,
        minimum_proto_version: Some(Version::new(0, 46, 0)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/evilmartians/lefthook")?
        .iter()
        .filter_map(|tag| tag.strip_prefix("v"))
        .filter(|tag| Version::parse(tag).is_ok())
        .map(|tag| tag.to_owned())
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations![
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64, HostArch::X86, HostArch::Arm64],
        ],
    )?;

    let version = &input.context.version;

    let os = match env.os {
        HostOS::Linux => "Linux",
        HostOS::MacOS => "MacOS",
        HostOS::Windows => "Windows",
        _ => unreachable!(),
    };

    let arch = match env.arch {
        HostArch::X64 => "x86_64",
        HostArch::Arm64 => "arm64",
        HostArch::X86 => "i386",
        _ => unreachable!(),
    };

    let ext = if env.os == HostOS::Windows {
        ".exe"
    } else {
        ".gz"
    };

    let filename = format!("lefthook_{version}_{os}_{arch}{ext}");

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/evilmartians/lefthook/releases/download/v{version}/{filename}"
        ),
        download_name: Some(filename),
        checksum_url: Some(format!(
            "https://github.com/evilmartians/lefthook/releases/download/v{version}/lefthook_checksums.txt"
        )),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(input): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;
    let version = &input.context.version;

    let os = match env.os {
        HostOS::Linux => "Linux",
        HostOS::MacOS => "MacOS",
        HostOS::Windows => "Windows",
        _ => unreachable!(),
    };

    let arch = match env.arch {
        HostArch::X64 => "x86_64",
        HostArch::Arm64 => "arm64",
        HostArch::X86 => "i386",
        _ => unreachable!(),
    };

    // The .gz archive unpacks to `lefthook_{version}_{os}_{arch}`
    let exe_name = env
        .os
        .get_exe_name(format!("lefthook_{version}_{os}_{arch}"));

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([("lefthook".into(), ExecutableConfig::new_primary(exe_name))]),
        ..LocateExecutablesOutput::default()
    }))
}

#[cfg(test)]
mod tests {
    use proto_pdk_test_utils::*;

    generate_download_install_tests!("lefthook", "2.1.4");
    generate_resolve_versions_tests!("lefthook", {
        "2.1" => "2.1.4",
    });
}
