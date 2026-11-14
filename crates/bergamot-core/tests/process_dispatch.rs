use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::{Duration, Instant};

use bergamot_core::executors::process;
use bergamot_core::staging::StagedFile;
use bergamot_core::{
    ActionInvocation, ActionRegistry, AppSettings, CancellationToken, Cardinality, Dispatcher,
    DragContext, ExecutionError, ExecutorConfig, OutputConfig, OutputMode, Platform,
};
use tempfile::tempdir;
use tokio::sync::mpsc;

const VALID: &str = include_str!("../../../shared/fixtures/valid-process.json");
const FILE_NAME: &str = "my file $(echo hacked) & test.txt";
const CHILD_MARKER: &str = "process-fixture-heartbeat";

#[tokio::test]
async fn executable_overrides_are_shared_by_detection_and_execution() {
    let temp = tempdir().unwrap();
    let path = temp.path().join(FILE_NAME);
    fs::write(&path, "input").unwrap();
    let mut executor = fixture_process("print", &["${input}"], 5);
    let ExecutorConfig::Process { program, .. } = &mut executor else {
        unreachable!()
    };
    *program = "bergamot-executable-fixture".into();
    let settings = AppSettings {
        tool_overrides: [(
            program.clone(),
            std::env::current_exe()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
        )]
        .into(),
        ..Default::default()
    };
    assert!(process::resolve_dependency(&executor, &settings)
        .unwrap()
        .unwrap()
        .is_absolute());
    let output = process::execute_with_settings(
        &executor,
        std::slice::from_ref(&path),
        None,
        &CancellationToken::new(),
        &settings,
    )
    .await
    .unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains(FILE_NAME));
    assert!(process::resolve_dependency(&executor, &AppSettings::default()).is_err());
}

#[tokio::test]
async fn custom_search_paths_find_tools_and_are_inherited_by_the_child() {
    let temp = tempdir().unwrap();
    let tools = temp.path().join("custom tools");
    fs::create_dir(&tools).unwrap();
    let program = "bergamot-custom-path-fixture";
    let executable = tools.join(format!("{program}{}", std::env::consts::EXE_SUFFIX));
    fs::copy(std::env::current_exe().unwrap(), &executable).unwrap();
    let input = temp.path().join(FILE_NAME);
    fs::write(&input, "input").unwrap();
    let mut executor = fixture_process("print-path", &[], 5);
    let ExecutorConfig::Process { program: tool, .. } = &mut executor else {
        unreachable!()
    };
    *tool = program.to_owned();
    let settings = AppSettings {
        custom_paths: vec![tools.to_string_lossy().into_owned()],
        ..Default::default()
    };
    assert_eq!(
        process::resolve_dependency(&executor, &settings).unwrap(),
        Some(executable)
    );
    let output = process::execute_with_settings(
        &executor,
        &[input],
        None,
        &CancellationToken::new(),
        &settings,
    )
    .await
    .unwrap();
    assert!(String::from_utf8_lossy(&output.stdout)
        .contains(&format!("first PATH directory: {}", tools.display())));
}

#[tokio::test]
async fn process_expands_each_input_as_one_literal_argument() {
    let temp = tempdir().unwrap();
    let first = temp.path().join(FILE_NAME);
    let second = temp.path().join("${input_stem}.txt");
    for path in [&first, &second] {
        fs::write(path, b"test").unwrap();
    }

    let result = process::execute(
        &fixture_process(
            "print",
            &[
                "${inputs}",
                "file=${input}",
                "${input_dir}",
                "${input_stem}",
                "${input_ext}",
                "${work_dir}",
            ],
            5,
        ),
        &[first.clone(), second.clone()],
        None,
        &CancellationToken::new(),
    )
    .await
    .unwrap();

    let stdout = String::from_utf8(result.stdout).unwrap();
    let args: Vec<&str> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix("fixture argument: "))
        .collect();
    assert_eq!(
        args,
        [
            first.display().to_string(),
            second.display().to_string(),
            format!("file={}", first.display()),
            temp.path().display().to_string(),
            "my file $(echo hacked) & test".to_owned(),
            "txt".to_owned(),
            temp.path().display().to_string(),
        ]
    );

    let error = process::execute(
        &fixture_process("print", &["prefix=${inputs}"], 5),
        &[first],
        None,
        &CancellationToken::new(),
    )
    .await
    .unwrap_err();
    assert!(matches!(error, ExecutionError::InvalidConfig(_)));
}

#[tokio::test]
async fn dispatcher_publishes_only_successful_files_and_resolves_collisions() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("output.txt");
    fs::write(&input, b"original").unwrap();
    let registry = Arc::new(ActionRegistry::open(temp.path().join("actions")).unwrap());
    let dispatcher = Dispatcher::new(Arc::clone(&registry)).unwrap();
    let mut action = ActionRegistry::parse_manifest(VALID).unwrap();
    action.output.mode = OutputMode::File;
    action.output.extension = Some("txt".into());
    action.executor = fixture_process("write", &["${output}"], 5);
    registry.save_action(&action).unwrap();

    let invocation = invocation(&action.id, vec![input.clone()]);
    let (progress, mut events) = mpsc::channel(4);
    let result = dispatcher
        .dispatch(&invocation, progress, &CancellationToken::new())
        .await
        .unwrap();
    let output = temp.path().join("output (1).txt");
    assert_eq!(result.output_files, std::slice::from_ref(&output));
    assert_eq!(fs::read(&output).unwrap(), b"converted");
    assert_eq!(fs::read(&input).unwrap(), b"original");
    assert_eq!(events.recv().await.unwrap().completed, 0);
    assert_eq!(events.recv().await.unwrap().completed, 1);

    action.executor = fixture_process("write-then-fail", &["${output}"], 5);
    action.output.overwrite_original = true;
    registry.save_action(&action).unwrap();
    let (progress, _events) = mpsc::channel(4);
    let error = dispatcher
        .dispatch(&invocation, progress, &CancellationToken::new())
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        ExecutionError::ProcessFailed { code: Some(17), .. }
    ));
    assert_eq!(fs::read(&input).unwrap(), b"original");
    assert_no_staging_files(temp.path());

    action.executor = fixture_process("write", &["${output}"], 5);
    registry.save_action(&action).unwrap();
    let (progress, _events) = mpsc::channel(4);
    let result = dispatcher
        .dispatch(&invocation, progress, &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(result.output_files, std::slice::from_ref(&input));
    assert_eq!(fs::read(&input).unwrap(), b"converted");
    assert_no_staging_files(temp.path());
}

#[test]
fn staged_outputs_never_clobber_and_only_publish_a_produced_file() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.txt");
    fs::write(&input, b"original").unwrap();
    let mut config = OutputConfig {
        mode: OutputMode::File,
        extension: None,
        suffix: None,
        overwrite_original: false,
    };
    let first = StagedFile::new(&input, &config).unwrap();
    let second = StagedFile::new(&input, &config).unwrap();
    assert_ne!(first.path(), second.path());
    assert_eq!(first.path().extension().unwrap(), "txt");
    fs::write(first.path(), b"first").unwrap();
    fs::write(second.path(), b"second").unwrap();
    let (first_path, second_path) = std::thread::scope(|scope| {
        let first_job = scope.spawn(move || first.commit().unwrap());
        let second_job = scope.spawn(move || second.commit().unwrap());
        (first_job.join().unwrap(), second_job.join().unwrap())
    });
    assert_ne!(first_path, second_path);
    assert_eq!(fs::read(first_path).unwrap(), b"first");
    assert_eq!(fs::read(second_path).unwrap(), b"second");

    config.overwrite_original = true;
    assert!(StagedFile::new(&input, &config).unwrap().commit().is_err());
    assert_eq!(fs::read(&input).unwrap(), b"original");

    let malformed = StagedFile::new(&input, &config).unwrap();
    fs::create_dir(malformed.path()).unwrap();
    fs::write(malformed.path().join("unexpected.txt"), b"data").unwrap();
    assert!(malformed.commit().is_err());
    assert_eq!(fs::read(&input).unwrap(), b"original");
    assert_no_staging_files(temp.path());

    let empty = StagedFile::new(&input, &config).unwrap();
    fs::write(empty.path(), b"").unwrap();
    assert_eq!(empty.commit().unwrap(), input);
    assert!(fs::read(&input).unwrap().is_empty());
    assert_no_staging_files(temp.path());
}

#[tokio::test]
async fn dispatcher_reports_each_file_and_batches_once() {
    let temp = tempdir().unwrap();
    let files: Vec<PathBuf> = ["first.txt", "second.txt"]
        .map(|name| temp.path().join(name))
        .into_iter()
        .collect();
    for file in &files {
        fs::write(file, b"test").unwrap();
    }
    let registry = Arc::new(ActionRegistry::open(temp.path().join("actions")).unwrap());
    let dispatcher = Dispatcher::new(Arc::clone(&registry)).unwrap();
    let mut action = ActionRegistry::parse_manifest(VALID).unwrap();
    action.executor = fixture_process("print", &["${inputs}"], 5);
    let invocation = invocation(&action.id, files);

    for (cardinality, expected_jobs) in [(Cardinality::Each, 2), (Cardinality::Batch, 1)] {
        action.matching.cardinality = cardinality;
        registry.save_action(&action).unwrap();
        let (progress, mut events) = mpsc::channel(1);
        let cancel = CancellationToken::new();
        let dispatch = dispatcher.dispatch(&invocation, progress, &cancel);
        let collect_progress = async {
            let mut all_events = Vec::new();
            while let Some(event) = events.recv().await {
                all_events.push((event.action_id, event.completed, event.total));
            }
            all_events
        };
        let (result, events) = tokio::join!(dispatch, collect_progress);
        let result = result.unwrap();
        assert_eq!(result.completed, expected_jobs);
        let expected_events: Vec<_> = (0..=expected_jobs)
            .map(|index| (action.id.clone(), index, expected_jobs))
            .collect();
        assert_eq!(events, expected_events);
        let output = String::from_utf8(result.stdout).unwrap();
        assert_eq!(output.matches("fixture invocation").count(), expected_jobs);
        assert!(output.contains("first.txt"));
        assert!(output.contains("second.txt"));
    }

    action.matching.cardinality = Cardinality::Single;
    registry.save_action(&action).unwrap();
    let (progress, _events) = mpsc::channel(4);
    assert!(matches!(
        dispatcher
            .dispatch(&invocation, progress, &CancellationToken::new())
            .await,
        Err(ExecutionError::ActionUnavailable(_))
    ));
}

#[cfg(unix)]
#[tokio::test]
async fn scripts_receive_literal_positional_arguments_and_output_path() {
    let temp = tempdir().unwrap();
    let input = temp.path().join(FILE_NAME);
    let output = temp.path().join("script-output.txt");
    fs::write(&input, b"input").unwrap();
    let body = "printf '%s' \"$1\" > \"$BERGAMOT_OUTPUT\"\n";
    let script = temp.path().join("print-path.sh");
    fs::write(&script, body).unwrap();

    for (script_body, script_path) in [(Some(body.to_owned()), None), (None, Some(script))] {
        let executor = ExecutorConfig::Script {
            interpreter: "/bin/sh".to_owned(),
            script_body,
            script_path,
            timeout_seconds: 5,
        };
        process::execute(
            &executor,
            std::slice::from_ref(&input),
            Some(&output),
            &CancellationToken::new(),
        )
        .await
        .unwrap();
        assert_eq!(
            fs::read_to_string(&output).unwrap(),
            input.display().to_string()
        );
    }
}

#[tokio::test]
async fn timeout_kills_descendant_after_the_leader_exits() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.txt");
    fs::write(&input, b"test").unwrap();
    let heartbeat = temp.path().join(CHILD_MARKER);
    let executor = fixture_process("spawn-descendant", &[CHILD_MARKER], 3);
    let start = Instant::now();
    let error = process::execute(&executor, &[input], None, &CancellationToken::new())
        .await
        .unwrap_err();
    assert!(matches!(error, ExecutionError::TimedOut(3)));
    assert!(start.elapsed() < Duration::from_secs(7));
    assert_heartbeat_stops(&heartbeat).await;
}

#[tokio::test]
async fn cancellation_kills_descendant_and_discards_staged_output() {
    let temp = tempdir().unwrap();
    let input = temp.path().join("input.txt");
    fs::write(&input, b"original").unwrap();
    let heartbeat = temp.path().join(CHILD_MARKER);
    let registry = Arc::new(ActionRegistry::open(temp.path().join("actions")).unwrap());
    let dispatcher = Dispatcher::new(Arc::clone(&registry)).unwrap();
    let mut action = ActionRegistry::parse_manifest(VALID).unwrap();
    action.executor = fixture_process("spawn-descendant", &[CHILD_MARKER, "${output}"], 10);
    action.output.mode = OutputMode::File;
    registry.save_action(&action).unwrap();
    let invocation = invocation(&action.id, vec![input.clone()]);

    let cancel = CancellationToken::new();
    let (progress, _events) = mpsc::channel(4);
    let execution = dispatcher.dispatch(&invocation, progress, &cancel);
    let trigger = async {
        let deadline = Instant::now() + Duration::from_secs(5);
        while !heartbeat.exists() {
            assert!(Instant::now() < deadline, "descendant did not start");
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
        let start = Instant::now();
        cancel.cancel();
        start
    };
    let (result, start) = tokio::join!(execution, trigger);
    assert!(matches!(result, Err(ExecutionError::Cancelled)));
    assert!(start.elapsed() < Duration::from_secs(3));
    assert_heartbeat_stops(&heartbeat).await;
    assert_eq!(fs::read(&input).unwrap(), b"original");
    assert!(!temp.path().join("input (1).txt").exists());
    assert_no_staging_files(temp.path());
}

fn invocation(action_id: &str, files: Vec<PathBuf>) -> ActionInvocation {
    ActionInvocation {
        action_id: action_id.to_owned(),
        context: DragContext {
            platform: Platform::current().unwrap(),
            files,
        },
    }
}

fn fixture_process(mode: &str, args: &[&str], timeout_seconds: u32) -> ExecutorConfig {
    ExecutorConfig::Process {
        program: std::env::current_exe()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap(),
        args: [
            "--exact",
            "--ignored",
            "--no-capture",
            "process_fixture",
            "--",
            mode,
        ]
        .into_iter()
        .chain(args.iter().copied())
        .map(str::to_owned)
        .collect(),
        working_directory: None,
        env: BTreeMap::new(),
        timeout_seconds,
    }
}

fn assert_no_staging_files(directory: &Path) {
    assert!(fs::read_dir(directory).unwrap().all(|entry| {
        !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .starts_with(".bergamot-tmp-")
    }));
}

async fn assert_heartbeat_stops(heartbeat: &Path) {
    assert!(heartbeat.exists(), "descendant did not start");
    tokio::time::sleep(Duration::from_millis(100)).await;
    let size = fs::metadata(heartbeat).unwrap().len();
    tokio::time::sleep(Duration::from_millis(350)).await;
    assert_eq!(
        fs::metadata(heartbeat).unwrap().len(),
        size,
        "descendant is alive"
    );
}

// Spawn this harness with an exact filter to test real subprocesses without installed CLI tools.
#[test]
#[ignore = "run by the process integration tests"]
fn process_fixture() {
    let mut args = std::env::args_os().skip_while(|arg| arg != "--").skip(1);
    let Some(mode) = args.next() else {
        return;
    };
    match mode.to_str().unwrap() {
        "print" => {
            println!("fixture invocation");
            for arg in args {
                println!("fixture argument: {}", arg.to_string_lossy());
            }
        }
        "print-path" => {
            let path = std::env::var_os("PATH").unwrap();
            println!(
                "first PATH directory: {}",
                std::env::split_paths(&path).next().unwrap().display()
            );
        }
        "write" | "write-then-fail" => {
            fs::write(args.next().unwrap(), b"converted").unwrap();
            if mode == "write-then-fail" {
                std::process::exit(17);
            }
        }
        "spawn-descendant" => {
            let heartbeat = args.next().unwrap();
            if let Some(output) = args.next() {
                fs::write(output, b"unfinished").unwrap();
            }
            let child = std::process::Command::new(std::env::current_exe().unwrap())
                .args([
                    "--exact",
                    "--ignored",
                    "--no-capture",
                    "process_fixture",
                    "--",
                    "heartbeat",
                ])
                .arg(heartbeat)
                .spawn()
                .unwrap();
            // Deliberately release the leader while the descendant holds its output pipes.
            drop(child);
        }
        "heartbeat" => {
            use std::io::Write;
            let mut file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(args.next().unwrap())
                .unwrap();
            for _ in 0..240 {
                file.write_all(b".").unwrap();
                std::thread::sleep(Duration::from_millis(50));
            }
        }
        other => panic!("unknown fixture mode {other}"),
    }
}
