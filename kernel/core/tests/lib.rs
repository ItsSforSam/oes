#[test]
/// This isis used for
fn ui() {
    // panic!("{}", std::env::current_dir().expect("Cannot get cwd").display());
    let tests_dir = std::path::Path::new("tests");
    let t = trybuild::TestCases::new();
    t.compile_fail(tests_dir.join("ui/syscall.rs"));
}

fn main() {}
