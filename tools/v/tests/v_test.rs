use proto_pdk_test_utils::*;

// The plugin installs V on macOS and Linux only.
#[cfg(not(windows))]
generate_download_install_tests!("v", "0.5.2");
generate_resolve_versions_tests!("v", {
    "0.4" => "0.4.12",
    "weekly.2026.08" => "2026.8.0",
});

// weekly.2026.41 has no release archive, so installing it builds V from source.
#[cfg(not(windows))]
mod build_from_source {
    use proto_pdk_test_utils::*;
    use std::collections::HashMap;

    // On Linux weekly.2026.41 fails V's own post-build check, upstream's `make` included:
    // its unused-code pruning drops `array__get`, which the Linux backtrace code calls.
    #[cfg(target_os = "macos")]
    generate_native_install_tests!("v", "weekly.2026.41", None, |config| {
        config.tool_config(HashMap::from([("build-from-source", true)]));
    });

    #[tokio::test(flavor = "multi_thread")]
    async fn refuses_without_opt_in() {
        let sandbox = create_empty_proto_sandbox();
        let mut plugin = sandbox.create_plugin("v").await;
        let mut spec = ToolSpec::parse("weekly.2026.41").unwrap();

        let error = flow::manage::Manager::new(&mut plugin.tool)
            .install(&mut spec, flow::install::InstallOptions::default())
            .await
            .unwrap_err();

        assert!(error.to_string().contains("build-from-source = true"));
    }
}
