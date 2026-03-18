use proto_pdk_test_utils::*;

generate_download_install_tests!("just", "1.47.1");
generate_resolve_versions_tests!("just", {
    "1.47" => "1.47.1",
});
