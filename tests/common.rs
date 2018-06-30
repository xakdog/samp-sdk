use std::process::Command;

const TESTS_DIR: &str = "tests";
const PLUGIN_DIR: &str = "tests/plugin";
const SERVER_URL: &str = "http://files.sa-mp.com/samp037svr_R2-1.tar.gz";


pub fn get_samp() {
    let archive = "server.tar.gz";

    println!("Downloading samp server");
    Command::new("curl")
            .args(&["-o", archive])
            .arg(SERVER_URL)
            .current_dir(TESTS_DIR)
            .output()
            .expect("failed to download samp server");

    println!("Unpacking...");
    Command::new("tar")
            .arg("xvzf")
            .arg(archive)
            .current_dir(TESTS_DIR)
            .output()
            .expect("failed to unpack samp server");

    Command::new("rm")
            .arg(archive)
            .current_dir(TESTS_DIR)
            .output()
            .expect("failed to remove server.tar.gz");
}

pub fn copy_samp_config() {
    // copy config
}

pub fn copy_samp_plugin() {
    // copy to samp directory
}

pub fn compile_samp_plugin() {
    println!("Building test samp plugin...");

    Command::new("cargo")
            .arg("build")
            .current_dir(PLUGIN_DIR)
            .output()
            .expect("failed to build samp plugin");
}

pub fn run_samp() {
    println!("running...");
}
