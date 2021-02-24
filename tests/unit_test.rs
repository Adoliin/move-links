//mod paths;
use move_links::paths::Paths;

use std::{env, process};

#[test]
fn paths_new() {
    process::Command::new("./scripts/init-test-dir.sh")
        .output()
        .unwrap();
    let cwd = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();
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
