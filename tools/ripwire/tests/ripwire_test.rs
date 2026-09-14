use proto_pdk_test_utils::*;

// Upstream ships no Windows build.
#[cfg(not(windows))]
generate_download_install_tests!("ripwire", "0.6.0");
generate_resolve_versions_tests!("ripwire", {
    "0.3" => "0.3.8",
});
