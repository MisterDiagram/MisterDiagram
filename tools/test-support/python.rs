//! Optional Python for package-comparison tests; mandatory in full CI.
use std::process::Command;

pub fn command() -> Option<Command> {
    for executable in ["python3", "python"] {
        let found =
            Command::new(executable).args(["-c", "import sys; sys.exit(sys.version_info.major != 3)"]).output().is_ok_and(|out| out.status.success());
        if found {
            return Some(Command::new(executable));
        }
    }
    let reason = "Python 3 not found on PATH (tried python3 and python)";
    assert!(std::env::var("MISTERDIAGRAM_REQUIRE_PYTHON").as_deref() != Ok("1"), "{reason}; MISTERDIAGRAM_REQUIRE_PYTHON=1 forbids skipping");
    eprintln!("{reason}; package comparison skipped");
    None
}
