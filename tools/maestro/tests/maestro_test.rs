use proto_pdk_test_utils::*;

generate_download_install_tests!("maestro", "2.6.1");
generate_resolve_versions_tests!("maestro", {
    "2.5" => "2.5.1",
});
