//! proto plugin for V, the vlang compiler. It installs a tag's GitHub release archive when
//! it has one and otherwise, when the project opts in, builds the tag from source, such as
//! recent weekly tags.

use extism_pdk::*;
use proto_pdk::*;
use serde::Deserialize;
use std::collections::HashMap;

#[host_fn]
extern "ExtismHost" {
    fn host_log(input: Json<HostLogInput>);
    fn send_request(input: Json<SendRequestInput>) -> Json<SendRequestOutput>;
}

static NAME: &str = "V";
static REPO: &str = "https://github.com/vlang/v";

/// Shell script that builds V at a tag in the install dir, see its header.
static BUILD_SCRIPT: &str = include_str!("build.sh");

/// Settings from `[tools.v]` in `.prototools`.
#[derive(Default, Deserialize)]
#[serde(default, rename_all = "kebab-case")]
struct VPluginConfig {
    /// Allows building a tag without a release archive from source, which runs the
    /// plugin's build script with the host's git, make and cc.
    build_from_source: bool,
}

/// The part of a GitHub release this plugin reads.
#[derive(Deserialize)]
struct Release {
    assets: Vec<Asset>,
}

/// A release archive and the `sha256:<hex>` digest GitHub computed when it was uploaded.
/// Assets uploaded before GitHub computed digests have none.
#[derive(Deserialize)]
struct Asset {
    name: String,
    digest: Option<String>,
}

/// Registers V as a language tool.
#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        minimum_proto_version: Some(Version::new(0, 57, 0)),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

/// Lists V's release tags, with `latest` the newest of them, and its weekly tags under
/// their own names as aliases of the versions [`weekly_version`] maps them to.
#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags(REPO)?;

    // Releases are tagged `0.4.12`, or `0.5` for a minor release. `tag` cannot map a version
    // back to the `v`-prefixed tags of 2019, so those are left out.
    let releases = tags
        .iter()
        .filter(|tag| tag.starts_with(|it: char| it.is_ascii_digit()))
        .filter_map(|tag| {
            Version::parse(tag)
                .or_else(|_| Version::parse(format!("{tag}.0")))
                .ok()
        })
        .map(VersionSpec::Version)
        .collect();
    let mut output = LoadVersionsOutput::from_versions(releases);

    for tag in tags {
        if let Some(version) = weekly_version(&tag) {
            output
                .aliases
                .insert(tag, UnresolvedVersionSpec::Version(version.clone()));
            output.versions.push(VersionSpec::Version(version));
        }
    }

    Ok(Json(output))
}

/// Builds V from source when its tag has no GitHub release and the project opts in with
/// [`VPluginConfig::build_from_source`], and otherwise leaves the install to
/// [`download_prebuilt`].
#[plugin_fn]
pub fn native_install(
    Json(input): Json<NativeInstallInput>,
) -> FnResult<Json<NativeInstallOutput>> {
    let tag = tag(&input.context.version);

    // Fails on hosts that V publishes no build for, which this build is untested on.
    release_asset()?;

    if fetch_release(&tag)?.is_some() {
        return Ok(Json(NativeInstallOutput {
            skip_install: true,
            ..NativeInstallOutput::default()
        }));
    }

    if !get_tool_config::<VPluginConfig>()?.build_from_source {
        return Ok(Json(NativeInstallOutput {
            error: Some(format!(
                "V {tag} has no release archive and can only be built from source, which runs \
                 this plugin's build script: git fetches vlang/v at the tag and vlang/vc, and \
                 make and cc build them in the install dir. To allow it, set \
                 `build-from-source = true` under `[tools.v]` in .prototools."
            )),
            ..NativeInstallOutput::default()
        }));
    }

    host_log!(
        stderr,
        "V {tag} has no release archive, building it from source with git, make and cc (about 2 minutes)"
    );

    let output = exec(ExecCommandInput {
        command: "sh".into(),
        args: vec!["-c".into(), BUILD_SCRIPT.into(), "sh".into(), tag.clone()],
        cwd: Some(input.install_dir),
        ..ExecCommandInput::default()
    })?;

    if output.exit_code != 0 {
        let log = output.stderr.lines().collect::<Vec<_>>();

        return Ok(Json(NativeInstallOutput {
            error: Some(format!(
                "building V {tag} from source failed:\n{}",
                log[log.len().saturating_sub(30)..].join("\n")
            )),
            ..NativeInstallOutput::default()
        }));
    }

    Ok(Json(NativeInstallOutput {
        installed: true,
        checksum: Some(Checksum::sha256(output.stdout.trim().into())),
        ..NativeInstallOutput::default()
    }))
}

/// Downloads the host's archive from the tag's GitHub release, verified against GitHub's
/// digest of it where the asset has one.
#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let tag = tag(&input.context.version);
    let asset = release_asset()?;
    let digest = fetch_release(&tag)?
        .and_then(|release| release.assets.into_iter().find(|it| it.name == asset))
        .ok_or_else(|| anyhow!("the release of V {tag} has no {asset}"))?
        .digest;

    Ok(Json(DownloadPrebuiltOutput {
        download_url: format!("{REPO}/releases/download/{tag}/{asset}"),
        checksum: digest
            .as_deref()
            .and_then(|digest| digest.strip_prefix("sha256:"))
            .map(|hash| Checksum::sha256(hash.into())),
        archive_prefix: Some("v".into()),
        ..DownloadPrebuiltOutput::default()
    }))
}

/// Points proto at the `v` binary in the root of V's tree, which it runs in place, as V
/// finds `vlib/`, `thirdparty/` and `cmd/` beside its real path.
#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let exe = ExecutableConfig {
        // When the C that a weekly V generates fails to compile, V silently compiles again
        // with V 0.5.2, which it downloads when missing. Switched off, so a weekly build is
        // one or fails; releases ignore the variable. Only the shim can set it, so the
        // README asks projects to also set it in `[env]`.
        shim_env_vars: Some(
            [("V_MACOS_V3_NO_FALLBACK".into(), "1".into())]
                .into_iter()
                .collect(),
        ),
        ..ExecutableConfig::new_primary("v")
    };

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([("v".into(), exe)]),
        ..LocateExecutablesOutput::default()
    }))
}

/// Maps a weekly tag, `weekly.2026.41`, to the version proto knows it by, `2026.41.0`.
/// Irregular old tags such as `weekly.2021.12.1` or `weekly.2025.1`, beside a
/// `weekly.2025.01`, map to none, which keeps [`tag`] its inverse.
fn weekly_version(tag: &str) -> Option<Version> {
    let (year, week) = tag.strip_prefix("weekly.")?.split_once('.')?;

    if year.len() != 4 || week.len() != 2 {
        return None;
    }

    Some(Version::new(year.parse().ok()?, week.parse().ok()?, 0))
}

/// The vlang/v tag of a version from [`load_versions`].
fn tag(version: &VersionSpec) -> String {
    match version.as_version() {
        Some(it) if it.major >= 2000 => format!("weekly.{}.{:02}", it.major, it.minor),
        // V tags a minor release `0.5`, never `0.5.0`.
        Some(it) if it.patch == 0 => format!("{}.{}", it.major, it.minor),
        _ => version.to_string(),
    }
}

/// The name of the host's archive in a V release, failing on hosts V has none for. V
/// publishes glibc builds only for Linux.
fn release_asset() -> AnyResult<&'static str> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        env,
        permutations![
            HostOS::Linux => [HostArch::X64, HostArch::Arm64],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
        ],
    )?;

    if env.libc == HostLibc::Musl {
        return Err(anyhow!("V publishes no musl builds"));
    }

    Ok(match (env.os, env.arch) {
        (HostOS::Linux, HostArch::X64) => "v_linux.zip",
        (HostOS::Linux, _) => "v_linux_arm64.zip",
        (_, HostArch::X64) => "v_macos_x86_64.zip",
        _ => "v_macos_arm64.zip",
    })
}

/// Fetches the GitHub release of V's `tag`, or `None` when the tag has none.
fn fetch_release(tag: &str) -> AnyResult<Option<Release>> {
    let mut request = SendRequestInput::new(format!(
        "https://api.github.com/repos/vlang/v/releases/tags/{tag}"
    ));

    // GitHub allows 60 unauthenticated API requests an hour per IP. The host expands the
    // variable into the header.
    if get_host_env_var("GITHUB_TOKEN")?.is_some() {
        request
            .headers
            .insert("Authorization".into(), "Bearer ${GITHUB_TOKEN}".into());
    }

    let response = send_request!(input, request);

    match response.status {
        200 => Ok(Some(response.json()?)),
        404 => Ok(None),
        status => Err(anyhow!(
            "GitHub answered {status} for the release of V {tag}; set GITHUB_TOKEN if it is rate limiting"
        )),
    }
}
