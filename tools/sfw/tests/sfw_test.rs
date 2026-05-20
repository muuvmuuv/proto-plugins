use proto_pdk_test_utils::*;

generate_download_install_tests!("sfw", "1.10.0");
generate_resolve_versions_tests!("sfw", {
    "1.7" => "1.7.2",
});
