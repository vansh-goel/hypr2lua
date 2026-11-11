use lang2lua::convert;

#[test]
fn converts_sanitized_real_config_fixture() {
    let converted = convert(include_str!("fixtures/minimal.conf")).expect("fixture should parse");

    assert!(converted.warnings.is_empty(), "{:?}", converted.warnings);
    assert_eq!(converted.lua, include_str!("fixtures/minimal.lua"));
}
