//mod paths;
use move_links::{cli_utils, paths::Paths, Config};
use std::{env, process};

#[test]
fn test_find_links() {
    init_test_dir();
    let config = Config::new(false);
    let cwd = get_cwd();
    let path_list = vec![
        //files
        (
            format!("{}/test-dir/f1", cwd),
            vec![
                format!("{}/test-dir/dir1/f1-l1", cwd),
                format!("{}/test-dir/dir4/dir5/f1-l2", cwd),
            ],
        ),
        (
            format!("{}/test-dir/dir2/f2", cwd),
            vec![
                format!("{}/test-dir/f2-l3", cwd),
                format!("{}/test-dir/dir2/f2-l2", cwd),
                format!("{}/test-dir/dir4/f2-l1", cwd),
            ],
        ),
        (
            format!("{}/test-dir/dir2/f2", cwd),
            vec![
                format!("{}/test-dir/dir4/f2-l1", cwd),
                format!("{}/test-dir/dir2/f2-l2", cwd),
                format!("{}/test-dir/f2-l3", cwd),
            ],
        ),
        (
            format!("{}/test-dir/dir4/dir5", cwd),
            vec![
                format!("{}/test-dir/dir5-l1", cwd),
                format!("{}/test-dir/dir4/dir5-l2", cwd),
                format!("{}/test-dir/dir5-l3", cwd),
            ],
        ),
        (
            format!("{}/test-dir/dir3", cwd),
            vec![
                format!("{}/test-dir/dir1/dir3", cwd),
                format!("{}/test-dir/dir2/dir3", cwd),
                format!("{}/test-dir/dir3/dir3", cwd),
            ],
        ),
    ];
    for path in path_list.iter() {
        let src_path = &path.0;
        let expect_link_paths = &path.1;
        let result_link_paths = cli_utils::find_links(
            src_path,
            &format!("{}/test-dir", cwd),
            &config,
        ).unwrap();
        let ok = same_vec_items(expect_link_paths, &result_link_paths);
        //assert_eq!(*expect_link_paths, result_link_paths);
        let err_msg = format!(
            "\nExpected: {:#?}\nResult: {:#?}\n",
            expect_link_paths, result_link_paths
        );
        assert!(ok, err_msg);
    }
}

#[test]
fn paths_new() {
    init_test_dir();
    let cwd = get_cwd();
    //let test_dir: PathBuf = []
    //env::set_current_dir(PathBuf::from())

    let path_list = [
        //files
        [
            (format!("{}", cwd), "."),
            (format!("{}/test-dir/f1", cwd), "test-dir/f1"),
            (format!("{}/test-dir/dir2/f1", cwd), "test-dir/dir2"),
        ],
        [
            (format!("{}/test-dir", cwd), "test-dir"),
            (format!("{}/test-dir/dir2/f2", cwd), "test-dir/dir2/f2"),
            (format!("{}/test-dir/f2", cwd), "test-dir"),
        ],
        [
            (format!("{}/test-dir", cwd), "test-dir/../test-dir"),
            (
                format!("{}/test-dir/dir3/f3", cwd),
                "test-dir/dir3/../dir3/f3",
            ),
            (
                format!("{}/test-dir/another-f3", cwd),
                "test-dir/../test-dir/another-f3",
            ),
        ],
        //directories
        [
            (format!("{}/test-dir", cwd), "test-dir"),
            (format!("{}/test-dir/dir4/dir5", cwd), "test-dir/dir4/dir5"),
            (format!("{}/test-dir/dir2/dir5", cwd), "test-dir/dir2"),
        ],
        [
            (format!("{}", cwd), "."),
            (format!("{}/test-dir", cwd), "test-dir"),
            (format!("{}/test-dir/dir1/test-dir", cwd), "test-dir/dir1"),
        ],
    ];
    for path_item in path_list.iter() {
        let (links_dir_param, src_path_param, dest_path_param) = (
            path_item[0].1.to_string(),
            path_item[1].1.to_string(),
            path_item[2].1.to_string(),
        );

        let path_expect = Paths {
            links_dir: path_item[0].0.to_string(),
            src_path: path_item[1].0.to_string(),
            dest_path: path_item[2].0.to_string(),
        };
        let path_result = Paths::new(&links_dir_param, &src_path_param, &dest_path_param).unwrap();
        assert_eq!(path_expect, path_result);
    }
}

fn same_vec_items<T: PartialEq>(v1: &Vec<T>, v2: &Vec<T>) -> bool {
    //check if the Vectors item are the same regardless of order
    let mut ok = true;
    for item in v1 {
        ok = v2.iter().any(|x| x == item);
    }
    ok
}

fn init_test_dir() {
    process::Command::new("./scripts/init-test-dir.sh")
        .output()
        .unwrap();
}

fn get_cwd() -> String {
    env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
}
