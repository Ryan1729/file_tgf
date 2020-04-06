use super::*;

use pretty_assertions::{assert_eq};
use std::collections::HashSet;

struct TGFSpec {
    folder: &'static str,
    regex: &'static str
}

fn run_tgf_spec(TGFSpec { folder, regex }: TGFSpec) -> String {
    let opt = Opt::from_iter_safe(vec![
        "file_tgf",
        "--input",
        folder,
        regex
    ]).unwrap();

    run_for_tgf(opt).unwrap()
}

struct EdgesSpec {
    folder: &'static str,
    regex: &'static str
}

fn run_edges_spec(EdgesSpec { folder, regex }: EdgesSpec) -> Vec<(String, String)> {
    let opt = Opt::from_iter_safe(vec![
        "file_tgf",
        "--input",
        folder,
        regex
    ]).unwrap();

    run_for_edges(opt).unwrap()
}

macro_rules! assert_has_same_lines {
    ($actual: expr, $expected: expr) => {{
        let actual_set: HashSet<&str> = $actual.lines().collect();
        let expected_set: HashSet<&str> = $expected.lines().collect();

        assert_eq!(actual_set, expected_set);
    }}
}

#[test]
fn the_empty_folder_produces_the_expected_tgf() {
    let actual = run_tgf_spec(TGFSpec {
        folder: "./test_file_tree/empty",
        regex: ".*"
    });
    
    let expected = 
r#"#
"#;

    assert_eq!(actual, expected);
}

#[test]
fn the_single_blank_file_produces_the_expected_tgf() {
    let actual = run_tgf_spec(TGFSpec {
        folder: "./test_file_tree/single",
        regex: ".*"
    });

    let expected = 
r#"#
"#;

    assert_eq!(actual, expected);
}

#[test]
fn the_single_loop_file_produces_the_expected_tgf() {
    let actual = run_tgf_spec(TGFSpec {
        folder: "./test_file_tree/single_loop",
        regex: ".*"
    });

    let expected = 
r#"1 loop
#
1 1
"#;

    assert_eq!(actual, expected);
}

#[test]
fn the_three_node_line_files_produces_the_expected_tgf() {
    let actual = run_tgf_spec(TGFSpec {
        folder: "./test_file_tree/three_node_line",
        regex: ".*"
    });

    let expected = 
r#"1 a
2 b
3 c
#
1 2
2 3
"#;

    assert_eq!(actual, expected);
}

#[test]
fn the_deadly_diamond_files_produces_the_expected_tgf() {
    let actual = run_tgf_spec(TGFSpec {
        folder: "./test_file_tree/deadly_diamond",
        regex: ".*"
    });

    let expected = 
r#"1 a
2 b
3 c
4 d
#
1 2
1 3
2 4
3 4
"#;

    assert_eq!(actual, expected);
}

#[test]
fn the_5_cell_files_produce_the_expected_tgf() {
    let actual = run_tgf_spec(TGFSpec {
        folder: "./test_file_tree/5-cell",
        regex: ".*"
    });

    let expected = 
r#"1 a
2 b
3 c
4 d
5 e
#
1 1
1 2
1 3
1 4
1 5
2 1
2 2
2 3
2 4
2 5
3 1
3 2
3 3
3 4
3 5
4 1
4 2
4 3
4 4
4 5
5 1
5 2
5 3
5 4
5 5
"#;

    assert_eq!(actual, expected);
}

macro_rules! edges {
    ($(($s1: expr, $s2: expr)),* $(,)?) => {
        vec![
            $(($s1.to_string(), $s2.to_string())),*
        ]
    }
}

/// AKA K3,3
#[test]
fn the_three_houses_files_produce_the_expected_edges() {
    let actual = run_edges_spec(EdgesSpec {
        folder: "./test_file_tree/three_houses",
        regex: ".*"
    });

    let expected = edges![
        ("1", "a"),
        ("1", "b"),
        ("1", "c"),
        ("2", "a"),
        ("2", "b"),
        ("2", "c"),
        ("3", "a"),
        ("3", "b"),
        ("3", "c"),
        ("a", "1"),
        ("a", "2"),
        ("a", "3"),
        ("b", "1"),
        ("b", "2"),
        ("b", "3"),
        ("c", "1"),
        ("c", "2"),
        ("c", "3"),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn the_k2_2_files_produce_the_expected_edges() {
    let actual = run_edges_spec(EdgesSpec {
        folder: "./test_file_tree/K2,2",
        regex: ".*"
    });

    let expected = edges![
        ("1", "a"),
        ("1", "b"),
        ("2", "a"),
        ("2", "b"),
        ("a", "1"),
        ("a", "2"),
        ("b", "1"),
        ("b", "2"),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn the_found_example_1_files_produce_the_expected_edges() {
    let actual = run_edges_spec(EdgesSpec {
        folder: "./test_file_tree/found_example_1",
        regex: ".*"
    });

    let expected = edges![
        ("pub_arb_g_i", "g_i"),
        ("pub_arb_g_i", "proptest"),
        ("pub_arb_platform_types", "platform_types"),
        ("pub_arb_rust_code", "macros"),
        ("pub_arb_rust_code", "proptest"),
        ("pub_arb_std", "proptest"),
        ("pub_arb_text_buffer", "text_buffer"),
    ];

    assert_eq!(actual, expected);
}

#[test]
fn the_found_example_1_files_produces_a_tgf_with_the_expected_lines() {
    let actual = run_tgf_spec(TGFSpec {
        folder: "./test_file_tree/found_example_1",
        regex: ".*"
    });

    let expected = 
r#"1 pub_arb_g_i
2 g_i
3 proptest
4 pub_arb_platform_types
5 platform_types
6 pub_arb_rust_code
7 macros
8 pub_arb_std
9 pub_arb_text_buffer
10 text_buffer
#
1 2
1 3
4 5
6 7
6 3
8 3
9 10
"#;

    assert_has_same_lines!(actual, expected);
}