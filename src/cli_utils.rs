use std::process;

pub fn move_link(src_path: &String, dest_path: &String) {
    let _output_mv = process::Command::new("mv")
        .arg(src_path)
        .arg(dest_path)
        .output()
        .expect("Failed to execute mv");
}

pub fn redirect_links(links: &Vec<String>, dest_path: &String) {
    for link in links {
        let _output_ln = process::Command::new("ln")
            .arg("-fs")
            .arg(dest_path)
            .arg(link)
            .output()
            .expect("Failed to execute ln");
    }
}

pub fn find_links(src_path: &String, links_dir: &String) -> Option<Vec<String>> {
    let output_find = process::Command::new("find")
        .arg(links_dir)
        .arg("-lname")
        .arg(src_path)
        .output()
        .expect("Failed to execute find");

    let mut links_paths = String::from_utf8_lossy(&output_find.stdout)
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if links_paths.len() > 1 {
        links_paths.pop();
        Some(links_paths)
    } else {
        None
    }
}
