fn main() {
    use std::io::Write;
    let output = std::process::Command::new("cat")
        .arg("*.txt")
        .arg("a.txt")
        .output()
        .expect("aFailed to build the project");
    std::io::stdout().write_all(&output.stdout).unwrap();
    println!("Hello, world!");
}
