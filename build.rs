use std::fs::read_to_string;

fn main() {
    let git_hash_main_branch = ".git/refs/heads/main";

    if let Ok(hash) = std::env::var("GIT_HASH") {
        println!("cargo:rustc-env=GIT_HASH={}", hash);
    } else if let Ok(hash) = std::env::var("GITHUB_SHA") {
        println!("cargo:rustc-env=GIT_HASH={}", hash);
    } else if let Ok(hash) = read_to_string(git_hash_main_branch) {
        println!("cargo:rustc-env=GIT_HASH={}", hash);
    } else {
        println!("cargo:rustc-env=GIT_HASH=unknown");
    }
}
