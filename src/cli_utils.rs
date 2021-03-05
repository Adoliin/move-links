use crate::Config;
use std::process;

pub fn move_link(src_path: &String, dest_path: &String, config: &Config) {
    let _output_mv = process::Command::new("mv")
        .arg(src_path)
        .arg(dest_path)
        .output()
        .expect("Failed to execute mv");

    verbose_msg(config, format!(
        "Executed: `mv {} {}`", src_path, dest_path
    ));
}

pub fn redirect_links(links: &Vec<String>, dest_path: &String, config: &Config) {
    for link in links {
        let _output_ln = process::Command::new("ln")
            .arg("-fs")
            .arg(dest_path)
            .arg(link)
            .output()
            .expect("Failed to execute ln");

        verbose_msg(config, format!(
            "Executed: `ln -fs {} {}`", dest_path, link
        ));
    }

}

pub fn find_links(src_path: &String, links_dir: &String, config: &Config) -> Option<Vec<String>> {
    let output_find = process::Command::new("find")
        .arg(links_dir)
        .arg("-lname")
        .arg(src_path)
        .output()
        .expect("Failed to execute find");

    verbose_msg(config, format!(
        "Executed: `find {} -lname {}`", links_dir, src_path
    ));

    let mut links_paths = String::from_utf8_lossy(&output_find.stdout)
        .split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    if links_paths.len() > 1 {
        links_paths.pop();
        verbose_msg(config, format!(
            "Found {} links: {:?} in {}",
            links_paths.len(),
            links_paths,
            links_dir,
        ));
        Some(links_paths)
    } else {
        verbose_msg(config, format!(
            "Found 0 links in {}",
            links_dir,
        ));
        None
    }
}

fn verbose_msg(config: &Config, msg: String) {
    if config.verbose {
        println!("{}", msg);
    }
}
