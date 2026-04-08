use proto_pdk_test_utils::*;

generate_download_install_tests!("lefthook", "2.1.4");
generate_resolve_versions_tests!("lefthook", {
    "2.0" => "2.0.16",
});
