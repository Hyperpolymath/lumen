mod cli {
    use std::process::{Command, Stdio};
    use std::sync::Once;

    #[test]
    fn without_arguments_prints_nothing_to_say() {
        ensure_compiled();

        let cli_output = Command::new("tests/_build/cli")
            .stdin(Stdio::null())
            .output()
            .unwrap();

        let stdout = String::from_utf8_lossy(&cli_output.stdout);
        let stderr = String::from_utf8_lossy(&cli_output.stderr);

        assert_eq!(
            String::from_utf8_lossy(&cli_output.stdout),
            "<<\"Nothing to say.\">>\n",
            "\nstdout = {}\nstderr = {}",
            stdout,
            stderr
        );
    }

    #[test]
    fn with_false_argument_prints_nothing_to_say() {
        ensure_compiled();

        let cli_output = Command::new("tests/_build/cli")
            .arg("false")
            .stdin(Stdio::null())
            .output()
            .unwrap();

        let stdout = String::from_utf8_lossy(&cli_output.stdout);
        let stderr = String::from_utf8_lossy(&cli_output.stderr);

        assert_eq!(
            stdout, "<<\"Nothing to say.\">>\n",
            "\nstdout = {}\nstderr = {}",
            stdout, stderr
        );
    }

    #[test]
    fn with_true_argument_prints_nothing_to_say() {
        ensure_compiled();

        let cli_output = Command::new("tests/_build/cli")
            .arg("true")
            .stdin(Stdio::null())
            .output()
            .unwrap();

        let stdout = String::from_utf8_lossy(&cli_output.stdout);
        let stderr = String::from_utf8_lossy(&cli_output.stderr);

        assert_eq!(
            stdout, "<<\"Hello, world!\">>\n",
            "\nstdout = {}\nstderr = {}",
            stdout, stderr
        );
    }

    static COMPILED: Once = Once::new();

    fn ensure_compiled() {
        COMPILED.call_once(|| {
            compile();
        })
    }

    fn compile() {
        let mut command = Command::new("../bin/lumen");

        command
            .arg("compile")
            .arg("--output")
            .arg("tests/_build/cli");

        let compile_output = command
            .arg("tests/cli/init.erl")
            .stdin(Stdio::null())
            .output()
            .unwrap();

        assert!(
            compile_output.status.success(),
            "stdout = {}\nstderr = {}",
            String::from_utf8_lossy(&compile_output.stdout),
            String::from_utf8_lossy(&compile_output.stderr)
        );
    }
}
