use proto_pdk_test_utils::*;

generate_download_install_tests!("jq", "1.8.2");
generate_resolve_versions_tests!("jq", {
    "1.7" => "1.7.1",
});
