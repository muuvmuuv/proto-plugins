use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "jq";

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
    let tags = load_git_tags("https://github.com/jqlang/jq")?
        .iter()
        .filter_map(|tag| tag.strip_prefix("jq-"))
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
            HostOS::Windows => [HostArch::X64],
        ],
    )?;

    let version = &input.context.version;

    let filename = match (env.os, env.arch) {
        (HostOS::Linux, HostArch::X64) => "jq-linux-amd64",
        (HostOS::Linux, HostArch::Arm64) => "jq-linux-arm64",
        (HostOS::MacOS, HostArch::X64) => "jq-macos-amd64",
        (HostOS::MacOS, HostArch::Arm64) => "jq-macos-arm64",
        (HostOS::Windows, HostArch::X64) => "jq-windows-amd64.exe",
        _ => unreachable!(),
    };

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/jqlang/jq/releases/download/jq-{version}/{filename}"
        ),
        download_name: Some(filename.to_string()),
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
            "jq".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("jq")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}

#[cfg(test)]
mod tests {
    use proto_pdk_test_utils::*;

    generate_download_install_tests!("jq", "1.8.1");
    generate_resolve_versions_tests!("jq", {
        "1.8" => "1.8.1",
    });
}
