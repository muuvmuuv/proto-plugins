use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "maestro";

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
    // Releases are tagged `cli-X.Y.Z`; the bare `vX.Y.Z` tags duplicate the
    // 2.x line and have no release assets, so only `cli-` tags count.
    let tags = load_git_tags("https://github.com/mobile-dev-inc/maestro")?
        .iter()
        .filter_map(|tag| tag.strip_prefix("cli-"))
        .filter(|tag| Version::parse(tag).is_ok())
        .map(|tag| tag.to_owned())
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let version = &input.context.version;

    // maestro.zip is a platform-independent JVM distribution (launcher
    // scripts + jars); it runs anywhere Java does, so no os/arch check.
    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/mobile-dev-inc/maestro/releases/download/cli-{version}/maestro.zip"
        ),
        download_name: Some("maestro.zip".into()),
        checksum_url: Some(format!(
            "https://github.com/mobile-dev-inc/maestro/releases/download/cli-{version}/checksums_sha256.txt"
        )),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    let exe_path = if env.os == HostOS::Windows {
        "maestro/bin/maestro.bat"
    } else {
        "maestro/bin/maestro"
    };

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([("maestro".into(), ExecutableConfig::new_primary(exe_path))]),
        // Lets `proto activate` put the real launcher on PATH instead of the
        // shim (see CLAUDE.md "PATH activation depends on filename matching
        // the tool id" — the launcher lives in a nested bin dir here).
        exes_dirs: vec!["maestro/bin".into()],
        ..LocateExecutablesOutput::default()
    }))
}
