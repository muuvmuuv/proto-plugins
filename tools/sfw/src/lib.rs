use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "sfw";

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
    let tags = load_git_tags("https://github.com/SocketDev/sfw-free")?
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
            HostOS::Windows => [HostArch::X64, HostArch::Arm64],
        ],
    )?;

    let version = &input.context.version;

    let os = match env.os {
        HostOS::Linux => "linux",
        HostOS::MacOS => "macos",
        HostOS::Windows => "windows",
        _ => unreachable!(),
    };

    let arch = match env.arch {
        HostArch::X64 => "x86_64",
        HostArch::Arm64 => "arm64",
        _ => unreachable!(),
    };

    // sfw-free publishes raw single-file binaries (no archive). proto's
    // non-archive install path will copy this to `<install_dir>/sfw`
    // automatically, which is what `proto activate` needs to add the dir to
    // PATH (see CLAUDE.md "PATH activation depends on filename matching the
    // tool id").
    let filename = env.os.get_exe_name(format!("sfw-free-{os}-{arch}"));

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/SocketDev/sfw-free/releases/download/v{version}/{filename}"
        ),
        download_name: Some(filename),
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
            "sfw".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("sfw")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}
