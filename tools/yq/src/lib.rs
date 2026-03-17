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

    let ext = if env.os == HostOS::Windows {
        "zip"
    } else {
        "tar.gz"
    };

    let filename = format!("yq_{os}_{arch}.{ext}");

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

    // The archive contains `yq_{os}_{arch}` (e.g. `yq_darwin_arm64`)
    let exe_name = env.os.get_exe_name(format!("yq_{os}_{arch}"));

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([("yq".into(), ExecutableConfig::new_primary(exe_name))]),
        ..LocateExecutablesOutput::default()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use proto_pdk_test_utils::*;

    generate_download_install_tests!("yq", "4.52.4");
    generate_resolve_versions_tests!("yq", {
        "4.52" => "4.52.4",
    });

    #[test]
    fn test_extract_sha256_from_checksums() {
        let checksums = "yq_darwin_arm64.tar.gz  7ff6bdf2  a9bb6ecfe2a9395127176ea1dfe8804a  361320bf1ea622ef9ff4046ca55c53ae  cc23541987beee103b7d14c95cf9e3347f427edb  25f125374b942ea11bdece9c78706685df00cb184f8c0333  qgala6i6d2rdnhvxr345adpnfh55mwr6f2xdbny  30242d6b522c1f593d890fbea589867d4caecb54  a9bb6ecfe2a9395127176ea1dfe8804a  4axr2xmjvrlsmwm4rcrl5ne2ttf56ijh  907e3301f0811a09c8253061ab35c45284eef5eeba7f38a6d3cd0456e96effdd30e2b711d9e9a7d618631946ee99b58cd7b73565cfebf7d9032bf3bf6d1b0217  cf584930ebe22da4e6e01bed807444e1e8c0458b  a403e502db9f5c5df45d6dca4e726433eb9f7633ab6e8c74db1c11a5ecee1407  b97b3adbe34a89418cc7f8c1404a27432e208021ba513ea5c99b7c8256a0b333  ef95efbe25197676bc18c592639b61802e5928ff  71aad9cf2b6cea23083bc3f92c751df6d1cfa52a7d248b9e64e5d643e33c1fea  b29d5a95c197aec20d497ac627c8256e74a6e2c11bd385e52826c3157a1f6749207337cea509dc73a523f439955042e4bf4c3d58fbf7f0f974b0048b440c5dce  f89bd8a93589e62651f9ac2039902be613422c99a5485109c40d997f  99778ab9ac307b89889607a8f84b4c16e668077ccb8665617547b9059a219ecc  fe810a5ca53f5b83b6f0764197afa58916e4243ddb8e6210388fb97df6adbcc4de0457d6eb219df1e9e483f1b323880f  c4bbd2c446287c83bc69e518b22d85201f4c00a15606e1573a993a9db04bf5206e24f99be05a769034c3a74d6b6698e149bbf66e9b2881209e6ce23b704f3cb4  351d28576b0b3f91f1e32009f3d188e6666f08a8f7d1ba1c164556cc6cec2a8d  9ef9e14981adf166140e2ac36a665ad6989c8ed945c27223dad275f4f92e3a1d65b41bf039b55a0106168b1cfb16ec402f6927ad8f5116fcad70faef796059ad  5c6ea41c14fe18e5e6fd5845ee94dd566a3d9b4f3263cd6b41f3a647  a112e0163e3d3eab62cd57a6237ed366256e91661dd1db0c2ce75da0962e1ee5  b6d39852afc0651ffedff6c9f3cc38deef7a611b6a22242ef6cd1e7f51790f3c6487ab17efa9fb455b4e8b57454f02b0  fec59b0ec26fc154acafc46e82ae6795e633e4ff896756ec487fb2b8e4a3ec63d87dfed7625c3366a434a17f21cb15d9d3edb2579bc8a34747c035f4768f183c  6fae700c  fc4664bc0d7363a45297427d99fa0cb0  958d2bb4ba764344002d4685d154012a97006f272d8f5153abd4d4f6544d23b5  50ff89033773d7e43aa7a653f3dc39455f3507b0e88c48d477ecf74d4bebf9a7  8d30933d0db36b08c79ffb838cac6d20afb979f084db6336844e4cae6022ea9409efaafa6523b7e2240fd7cedbd68a06e74d8fa876ee6f0c91fd2696301e387a";
        let hash = extract_sha256_from_checksums(checksums, "yq_darwin_arm64.tar.gz");
        assert_eq!(
            hash,
            Some("99778ab9ac307b89889607a8f84b4c16e668077ccb8665617547b9059a219ecc".to_string())
        );
    }

    #[test]
    fn test_extract_sha256_not_found() {
        let checksums = "yq_darwin_arm64.tar.gz  abc  def";
        let hash = extract_sha256_from_checksums(checksums, "yq_linux_amd64.tar.gz");
        assert_eq!(hash, None);
    }
}
