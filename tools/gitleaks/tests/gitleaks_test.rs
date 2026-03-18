use proto_pdk_test_utils::*;

generate_download_install_tests!("gitleaks", "8.30.0");
generate_resolve_versions_tests!("gitleaks", {
    "8.24" => "8.24.3",
});
