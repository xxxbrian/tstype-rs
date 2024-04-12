extern crate tstype_rs;
use tstype_rs::parser::parse;
use tstype_rs::builder::build;

#[test]
fn test_build_basic() {
    let ts_type = parse("User").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "User");
}

#[test]
fn test_build_single_s_array() {
    let ts_type = parse("User[]").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "User[]");
}

#[test]
fn test_build_single_g_array() {
    let ts_type = parse("Array<User>").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "User[]");
}

#[test]
fn test_build_multiple_s_array() {
    let ts_type = parse("User[][][][]").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "User[][][][]");
}

#[test]
fn test_build_multiple_g_array() {
    let ts_type = parse("Array<Array<Array<User>>>").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "User[][][]");
}

#[test]
fn test_build_multiple_complex_array() {
    let ts_type = parse("Array<Array<Array<User[]>>[]>").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "User[][][][][]");
}

#[test]
fn test_build_map() {
    let ts_type = parse("Record<string, User>").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "Record<string, User>");
}

#[test]
fn test_build_map_with_complex_array() {
    let ts_type = parse("Record<string, Array<User[]>>").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "Record<string, User[][]>");
}

#[test]
fn test_build_map_with_complex_array_and_array_wrapper() {
    let ts_type = parse("Array<Record<string, Array<User>[]>>").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "Record<string, User[][]>[]");
}

#[test]
fn test_build_union() {
    let ts_type = parse("User | Record<string, User>").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "Record<string, User> | User");
}

#[test]
fn test_build_union_with_multiple_unions() {
    let ts_type = parse("User | Record<string, User> | Array<User>").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "Record<string, User> | User[] | User");
}

#[test]
fn test_build_realcase() {
    let ts_type = parse("User[] | Record<string,Record<number| string|Something, Array<string[][][] | Record<string|number, User[]>>>[][]>[] | number[] | string").unwrap();
    let result = build(ts_type);
    assert_eq!(result, "Record<string, Record<Something | number | string, Record<string | number, User[]> | string[][][][]>[][]>[] | number[] | User[] | string");
}