use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "ripwire";

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::CommandLine,
        minimum_proto_version: Some(Version::new(0, 57, 0)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/redhat-et/ripwire")?
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
        env,
        permutations![
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
        ],
    )?;

    let version = &input.context.version;

    let os = match env.os {
        HostOS::Linux => "linux",
        HostOS::MacOS => "macos",
        _ => unreachable!(),
    };

    let arch = match env.arch {
        HostArch::X64 => "x64",
        HostArch::Arm64 => "arm64",
        _ => unreachable!(),
    };

    let prefix = format!("ripwire-{version}-{os}-{arch}");
    let filename = format!("{prefix}.tar.gz");

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/redhat-et/ripwire/releases/download/v{version}/{filename}"
        ),
        checksum_url: Some(format!(
            "https://github.com/redhat-et/ripwire/releases/download/v{version}/{filename}.sha256"
        )),
        download_name: Some(filename),
        archive_prefix: Some(prefix),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([("ripwire".into(), ExecutableConfig::new_primary("ripwire"))]),
        ..LocateExecutablesOutput::default()
    }))
}
