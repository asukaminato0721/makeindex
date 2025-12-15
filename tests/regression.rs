use std::{
    fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn repo_file(path: &str) -> PathBuf {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push(path);
    root
}

#[derive(Clone, Copy)]
enum Arg {
    Literal(&'static str),
    Repo(&'static str),
}

#[derive(Clone, Copy)]
struct Fixture {
    name: &'static str,
    args: &'static [Arg],
}

const FIXTURES: &[Fixture] = {
    use Arg::{Literal as Lit, Repo};
    &[
        Fixture {
            name: "b209a",
            args: &[],
        },
        Fixture {
            name: "b209b",
            args: &[],
        },
        Fixture {
            name: "b209c",
            args: &[],
        },
        Fixture {
            name: "b209d",
            args: &[],
        },
        Fixture {
            name: "b209e",
            args: &[],
        },
        Fixture {
            name: "b209f",
            args: &[],
        },
        Fixture {
            name: "b210a",
            args: &[Lit("-s"), Repo("makeindex/test/b210a.ist")],
        },
        Fixture {
            name: "b211a",
            args: &[],
        },
        Fixture {
            name: "b211b",
            args: &[Lit("-s"), Repo("makeindex/test/b211b.ist"), Lit("-g")],
        },
        Fixture {
            name: "b211c",
            args: &[Lit("-s"), Repo("makeindex/test/b211c.ist")],
        },
        Fixture {
            name: "b211d",
            args: &[],
        },
        Fixture {
            name: "b211e",
            args: &[],
        },
        Fixture {
            name: "b211f",
            args: &[],
        },
        Fixture {
            name: "b211g",
            args: &[],
        },
        Fixture {
            name: "b211h",
            args: &[],
        },
        Fixture {
            name: "book",
            args: &[],
        },
        Fixture {
            name: "test",
            args: &[],
        },
        Fixture {
            name: "tort",
            args: &[],
        },
    ]
};

#[test]
fn reference_indices_match_goldens() {
    let binary = env!("CARGO_BIN_EXE_makeindex");
    for fixture in FIXTURES {
        let idx_path = format!("makeindex/test/{}.idx", fixture.name);
        let expect_path = format!("makeindex/test/ok-{}.ind", fixture.name);
        let output = {
            let mut cmd = Command::new(binary);
            cmd.current_dir(env!("CARGO_MANIFEST_DIR"))
                .arg("-o")
                .arg("-")
                .arg("-t")
                .arg("-")
                .arg("-q");
            for arg in fixture.args {
                match arg {
                    Arg::Literal(v) => {
                        cmd.arg(v);
                    }
                    Arg::Repo(path) => {
                        cmd.arg(repo_file(path));
                    }
                }
            }
            cmd.arg(repo_file(&idx_path))
                .output()
                .unwrap_or_else(|e| panic!("failed to run makeindex for {}: {e}", fixture.name))
        };

        assert!(
            output.status.success(),
            "makeindex exited with {}\nfixture: {}\nstderr:\n{}",
            output.status,
            fixture.name,
            String::from_utf8_lossy(&output.stderr)
        );

        let actual = output.stdout;
        let expected = fs::read(repo_file(&expect_path))
            .unwrap_or_else(|_| panic!("missing golden for {}", fixture.name));

        assert_eq!(
            actual, expected,
            "output mismatch for fixture {}",
            fixture.name
        );
    }
}

#[test]
fn default_output_files_created() {
    let binary = env!("CARGO_BIN_EXE_makeindex");
    let workdir = temp_dir("default-output");
    fs::create_dir(&workdir).expect("failed to create temp dir");
    fs::copy(
        repo_file("makeindex/test/test.idx"),
        workdir.join("test.idx"),
    )
    .expect("failed to copy idx fixture");

    let status = Command::new(binary)
        .current_dir(&workdir)
        .arg("-q")
        .arg("test")
        .status()
        .expect("failed to run makeindex binary");
    assert!(status.success(), "default run failed");

    let actual = fs::read_to_string(workdir.join("test.ind")).expect("missing generated test.ind");
    let expected =
        fs::read_to_string(repo_file("makeindex/test/ok-test.ind")).expect("missing ok-test.ind");
    assert_eq!(actual, expected);
    assert!(
        workdir.join("test.ilg").exists(),
        "expected transcript file in default run"
    );

    fs::remove_dir_all(&workdir).ok();
}

fn temp_dir(label: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    path.push(format!("makeindex-{}-{}", label, unique));
    path
}
