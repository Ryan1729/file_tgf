use super::*;

use pretty_assertions::{assert_eq};

struct Spec {
    folder: &'static str,
    regex: &'static str
}

fn run_spec(Spec { folder, regex }: Spec) -> String {
    let opt = Opt::from_iter_safe(vec![
        "file_tgf",
        "--input",
        folder,
        regex
    ]).unwrap();

    run_for_tgf(opt).unwrap()
}

#[test]
fn the_empty_folder_produces_the_expected_tgf() {
    let actual = run_spec(Spec {
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
    let actual = run_spec(Spec {
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
    let actual = run_spec(Spec {
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
    let actual = run_spec(Spec {
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
    let actual = run_spec(Spec {
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
    let actual = run_spec(Spec {
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

