use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "Gitleaks";

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
    let tags = load_git_tags("https://github.com/gitleaks/gitleaks")?
        .iter()
        .filter_map(|tag| tag.strip_prefix("v"))
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
        HostOS::Linux => "linux",
        HostOS::MacOS => "darwin",
        HostOS::Windows => "windows",
        _ => unreachable!(),
    };

    let arch = match env.arch {
        HostArch::X64 => "x64",
        HostArch::Arm64 => "arm64",
        HostArch::X86 => "x32",
        _ => unreachable!(),
    };

    let ext = if env.os == HostOS::Windows {
        "zip"
    } else {
        "tar.gz"
    };

    let filename = format!("gitleaks_{version}_{os}_{arch}.{ext}");

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/gitleaks/gitleaks/releases/download/v{version}/{filename}"
        ),
        download_name: Some(filename),
        checksum_url: Some(format!(
            "https://github.com/gitleaks/gitleaks/releases/download/v{version}/gitleaks_{version}_checksums.txt"
        )),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([(
            "gitleaks".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("gitleaks")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}

#[cfg(test)]
mod tests {
    use proto_pdk_test_utils::*;

    generate_download_install_tests!("gitleaks", "8.30.0");
    generate_resolve_versions_tests!("gitleaks", {
        "8.30" => "8.30.0",
    });
}
