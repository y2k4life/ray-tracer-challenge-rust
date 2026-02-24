use std::{
    path::Path,
    process::{exit, Command},
};

const CHAPTERS: &[&str] = &[
    "chapter_01",
    "chapter_02",
    "chapter_03",
    "chapter_04",
    "chapter_05",
    "chapter_06",
    "chapter_07",
    "chapter_08",
    "chapter_09",
    "chapter_10a",
    "chapter_10b",
    "chapter_11",
    "chapter_12",
    "chapter_13",
    "chapter_14",
    "chapter_15",
    "chapter_16",
];

fn main() {
    let task = std::env::args().nth(1).unwrap_or_default();
    match task.as_str() {
        "build"   => run_all(&["build", "--all"]),
        "test"    => run_all(&["test", "--all"]),
        "release" => run_all(&["build", "--all", "--release"]),
        "clippy"  => run_all(&["clippy"]),
        "fmt"     => run_all(&["fmt"]),
        "update"  => run_all(&["update"]),
        "clean"   => run_all(&["clean"]),
        "run"     => run_examples(),
        "delete"  => delete_ppms(),
        "all"     => {
            run_all(&["update"]);
            run_all(&["clean"]);
            run_all(&["fmt"]);
            run_all(&["clippy"]);
            run_all(&["build", "--all"]);
            run_all(&["test", "--all"]);
            run_examples();
            run_all(&["clean"]);
        }
        _ => {
            eprintln!(
                "Usage: cargo xtask [build|test|release|clippy|fmt|update|clean|run|delete|all]"
            );
            exit(1);
        }
    }
}

fn run_all(args: &[&str]) {
    for chapter in CHAPTERS {
        println!("\n*** {chapter} ***");
        let ok = Command::new("cargo")
            .args(args)
            .current_dir(chapter)
            .status()
            .expect("cargo not found")
            .success();
        if !ok {
            eprintln!("Failed in {chapter}");
            exit(1);
        }
    }
}

/// Replicates btr.sh -r: for each chapter, runs all examples from
/// chapter_01 up to and including the current chapter.
fn run_examples() {
    for (i, chapter) in CHAPTERS.iter().enumerate() {
        println!("\n*** {chapter} ***");
        for example in &CHAPTERS[..=i] {
            println!("  running example {example}");
            let ok = Command::new("cargo")
                .args(["run", "--example", example, "--release"])
                .current_dir(chapter)
                .status()
                .expect("cargo not found")
                .success();
            if !ok {
                eprintln!("Failed running example {example} in {chapter}");
                exit(1);
            }
        }
    }
}

/// Replicates btr.sh -d: deletes all .ppm files from each chapter directory.
fn delete_ppms() {
    for chapter in CHAPTERS {
        let dir = Path::new(chapter);
        let Ok(entries) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("ppm") {
                match std::fs::remove_file(&path) {
                    Ok(_) => println!("Deleted {}", path.display()),
                    Err(e) => eprintln!("Could not delete {}: {e}", path.display()),
                }
            }
        }
    }
}
