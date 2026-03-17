use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "just";

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
    let tags = load_git_tags("https://github.com/casey/just")?;

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

    let target = match (env.os, env.arch) {
        (HostOS::Linux, HostArch::X64) => "x86_64-unknown-linux-musl",
        (HostOS::Linux, HostArch::Arm64) => "aarch64-unknown-linux-musl",
        (HostOS::MacOS, HostArch::X64) => "x86_64-apple-darwin",
        (HostOS::MacOS, HostArch::Arm64) => "aarch64-apple-darwin",
        (HostOS::Windows, HostArch::X64) => "x86_64-pc-windows-msvc",
        (HostOS::Windows, HostArch::Arm64) => "aarch64-pc-windows-msvc",
        _ => unreachable!(),
    };

    let ext = if env.os == HostOS::Windows {
        "zip"
    } else {
        "tar.gz"
    };

    let filename = format!("just-{version}-{target}.{ext}");

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/casey/just/releases/download/{version}/{filename}"
        ),
        download_name: Some(filename),
        checksum_url: Some(format!(
            "https://github.com/casey/just/releases/download/{version}/SHA256SUMS"
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
            "just".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("just")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}

#[cfg(test)]
mod tests {
    use proto_pdk_test_utils::*;

    generate_download_install_tests!("just", "1.47.1");
    generate_resolve_versions_tests!("just", {
        "1.47" => "1.47.1",
    });
}
