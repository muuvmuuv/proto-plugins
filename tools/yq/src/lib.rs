use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

static NAME: &str = "yq";

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
    let tags = load_git_tags("https://github.com/mikefarah/yq")?
        .iter()
        .filter_map(|tag| tag.strip_prefix("v"))
        .filter(|tag| Version::parse(tag).is_ok())
        .map(|tag| tag.to_owned())
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

/// Extract the SHA-256 hash for a given filename from yq's non-standard checksums file.
///
/// The yq project publishes a multi-hash checksums file where each line has the format:
/// `filename CRC32 MD4 MD5 SHA1 ... SHA-256 ...` (30+ hash algorithms per file).
/// The SHA-256 hash is at a fixed column position (index 18, where filename is index 0).
/// We also verify by checking the hash is exactly 64 hex characters.
fn extract_sha256_from_checksums(checksums: &str, filename: &str) -> Option<String> {
    // SHA-256 column index in yq's checksums file (0=filename, 1=CRC32, ..., 18=SHA-256)
    const SHA256_COLUMN: usize = 18;

    for line in checksums.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() > SHA256_COLUMN && parts[0] == filename {
            let hash = parts[SHA256_COLUMN];
            // Verify it's a valid SHA-256 hash (64 hex characters)
            if hash.len() == 64 && hash.chars().all(|c| c.is_ascii_hexdigit()) {
                return Some(hash.to_string());
            }
        }
    }
    None
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
        HostOS::MacOS => "darwin",
        HostOS::Windows => "windows",
        _ => unreachable!(),
    };

    let arch = match env.arch {
        HostArch::X64 => "amd64",
        HostArch::Arm64 => "arm64",
        _ => unreachable!(),
    };

    // yq publishes a raw single-file binary alongside the archive
    // (e.g. `yq_darwin_arm64` on unix, `yq_windows_amd64.exe` on windows).
    //
    // We download the raw binary rather than the tar.gz / zip so that proto's
    // default non-archive install path renames it to `<install_dir>/yq`
    // automatically. If we downloaded the archive, the extracted binary would
    // keep its platform-suffixed name (e.g. `yq_darwin_arm64`) and `proto activate`
    // wouldn't add the install dir to PATH — forcing invocations through the
    // shim, which mangles shell-special characters in arguments.
    let filename = env.os.get_exe_name(format!("yq_{os}_{arch}"));

    // yq publishes a non-standard multi-hash checksums file that proto cannot parse.
    // We fetch it ourselves and extract the SHA-256 hash for the target file.
    let checksums_url =
        format!("https://github.com/mikefarah/yq/releases/download/v{version}/checksums");
    let checksum = fetch(SendRequestInput::new(checksums_url))
        .ok()
        .and_then(|response| response.text().ok())
        .and_then(|body| extract_sha256_from_checksums(&body, &filename))
        .map(Checksum::sha256);

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!(
            "https://github.com/mikefarah/yq/releases/download/v{version}/{filename}"
        ),
        download_name: Some(filename),
        checksum,
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
            "yq".into(),
            ExecutableConfig::new_primary(env.os.get_exe_name("yq")),
        )]),
        ..LocateExecutablesOutput::default()
    }))
}
