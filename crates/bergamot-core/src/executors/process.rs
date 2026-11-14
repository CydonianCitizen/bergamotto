use std::ffi::{OsStr, OsString};
use std::io;
use std::path::{absolute, Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use command_group::{AsyncCommandGroup, AsyncGroupChild};
use tempfile::NamedTempFile;
use tokio::io::{AsyncRead, AsyncReadExt};
use tokio::process::Command;
use tokio_util::sync::CancellationToken;

use crate::descriptor::ExecutorConfig;

use super::{ExecutionError, ExecutionOutput};

/// Runs a direct command or a script file with separate OS arguments.
/// `${inputs}` expands only when it occupies an entire argument.
/// Scripts receive input paths as positional arguments, plus `BERGAMOT_OUTPUT` in file mode.
/// Commands and script text are trusted configuration supplied by the manifest author.
pub async fn execute(
    executor: &ExecutorConfig,
    files: &[PathBuf],
    output: Option<&Path>,
    cancel: &CancellationToken,
) -> Result<ExecutionOutput, ExecutionError> {
    if files.is_empty() {
        return Err(ExecutionError::InvalidConfig(
            "an executor requires at least one input file".to_owned(),
        ));
    }
    if cancel.is_cancelled() {
        return Err(ExecutionError::Cancelled);
    }

    let inputs = files.iter().map(absolute).collect::<io::Result<Vec<_>>>()?;
    let output = output.map(absolute).transpose()?;
    let default_work_dir = inputs[0].parent().ok_or_else(|| {
        ExecutionError::InvalidConfig("the first input has no parent directory".to_owned())
    })?;

    let (mut command, timeout_seconds, _script_temp) = match executor {
        ExecutorConfig::Process {
            program,
            args,
            working_directory,
            env,
            timeout_seconds,
        } => {
            reject_windows_batch_file(program)?;
            let work_dir = match working_directory {
                Some(path) => {
                    resolve_working_directory(path, &inputs, output.as_deref(), default_work_dir)?
                }
                None => default_work_dir.to_path_buf(),
            };

            let mut command = Command::new(resolve_program(program)?);
            for argument in args {
                for expanded in expand_argument(argument, &inputs, output.as_deref(), &work_dir)? {
                    command.arg(expanded);
                }
            }
            command.current_dir(work_dir).envs(env);
            (command, *timeout_seconds, None)
        }
        ExecutorConfig::Script {
            interpreter,
            script_body,
            script_path,
            timeout_seconds,
        } => {
            reject_windows_batch_file(interpreter)?;
            let work_dir = default_work_dir;
            let script =
                resolve_script(script_path.as_deref(), script_body.as_deref(), interpreter)?;

            let mut command = Command::new(resolve_program(interpreter)?);
            if is_powershell(interpreter) {
                command.args([
                    OsStr::new("-NoProfile"),
                    OsStr::new("-NonInteractive"),
                    OsStr::new("-File"),
                ]);
            }
            command.arg(script.path);
            command.args(&inputs);
            command.current_dir(work_dir);
            match std::env::join_paths(&inputs) {
                Ok(paths) => {
                    command.env("BERGAMOT_INPUT_FILES", paths);
                }
                Err(_) => {
                    command.env_remove("BERGAMOT_INPUT_FILES");
                }
            }
            if let Some(output) = output.as_deref() {
                command.env("BERGAMOT_OUTPUT", output);
            } else {
                command.env_remove("BERGAMOT_OUTPUT");
            }
            command.env("BERGAMOT_WORK_DIR", work_dir);
            (command, *timeout_seconds, script.temp_file)
        }
        ExecutorConfig::Webhook { .. } => {
            return Err(ExecutionError::InvalidConfig(
                "webhook executors must use the webhook executor".to_owned(),
            ));
        }
    };

    if timeout_seconds == 0 {
        return Err(ExecutionError::InvalidConfig(
            "process timeout must be greater than zero".to_owned(),
        ));
    }

    command
        .kill_on_drop(true)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut group = ActiveGroup {
        child: command.group().kill_on_drop(true).spawn()?,
        active: true,
    };
    let completion = {
        let leader = group.child.inner();
        let stdout = leader.stdout.take();
        let stderr = leader.stderr.take();
        let wait_and_read = async move {
            // The deadline stays active while descendants hold inherited pipes open.
            tokio::try_join!(leader.wait(), read_pipe(stdout), read_pipe(stderr))
        };
        let timeout = Duration::from_secs(u64::from(timeout_seconds));
        tokio::select! {
            biased;
            _ = cancel.cancelled() => Err(ExecutionError::Cancelled),
            result = tokio::time::timeout(timeout, wait_and_read) => match result {
                Ok(result) => result.map_err(ExecutionError::Io),
                Err(_) => Err(ExecutionError::TimedOut(timeout_seconds)),
            },
        }
    };

    let (status, stdout, stderr) = match completion {
        Ok(output) => output,
        Err(error) => {
            group.terminate().await;
            return Err(error);
        }
    };
    if !status.success() {
        group.terminate().await;
        return Err(ExecutionError::ProcessFailed {
            code: status.code(),
            stderr: String::from_utf8_lossy(&stderr).into_owned(),
        });
    }

    group.active = false;
    Ok(ExecutionOutput { stdout, stderr })
}

fn resolve_working_directory(
    configured: &Path,
    inputs: &[PathBuf],
    output: Option<&Path>,
    default_work_dir: &Path,
) -> Result<PathBuf, ExecutionError> {
    let Some(configured) = configured.to_str() else {
        return Ok(default_work_dir.join(configured));
    };
    if configured.contains("${inputs}") {
        return Err(ExecutionError::InvalidConfig(
            "working directory cannot contain `${inputs}`".to_owned(),
        ));
    }
    let expanded = expand_argument(configured, inputs, output, default_work_dir)?;
    Ok(default_work_dir.join(&expanded[0]))
}

fn resolve_script(
    script_path: Option<&Path>,
    script_body: Option<&str>,
    interpreter: &str,
) -> Result<ResolvedScript, ExecutionError> {
    match (script_path, script_body) {
        (Some(path), None) => Ok(ResolvedScript {
            // Resolve configured scripts before entering the input's directory.
            path: absolute(path)?,
            temp_file: None,
        }),
        (None, Some(body)) => {
            let extension = if is_powershell(interpreter) {
                ".ps1"
            } else {
                ".script"
            };
            let mut file = tempfile::Builder::new()
                .prefix(".bergamot-script-")
                .suffix(extension)
                .tempfile()?;
            io::Write::write_all(&mut file, body.as_bytes())?;
            io::Write::flush(&mut file)?;
            let path = file.path().to_path_buf();
            Ok(ResolvedScript {
                path,
                temp_file: Some(file),
            })
        }
        (None, None) => Err(ExecutionError::InvalidConfig(
            "script executor requires `scriptBody` or `scriptPath`".to_owned(),
        )),
        (Some(_), Some(_)) => Err(ExecutionError::InvalidConfig(
            "script executor accepts either `scriptBody` or `scriptPath`, not both".to_owned(),
        )),
    }
}

struct ResolvedScript {
    path: PathBuf,
    temp_file: Option<NamedTempFile>,
}

fn expand_argument(
    template: &str,
    inputs: &[PathBuf],
    output: Option<&Path>,
    work_dir: &Path,
) -> Result<Vec<OsString>, ExecutionError> {
    if template == "${inputs}" {
        return Ok(inputs
            .iter()
            .map(|path| path.as_os_str().to_owned())
            .collect());
    }
    if template.contains("${inputs}") {
        return Err(ExecutionError::InvalidConfig(
            "`${inputs}` must occupy a complete argument".to_owned(),
        ));
    }

    let input = inputs.first().ok_or_else(|| {
        ExecutionError::InvalidConfig("an input placeholder requires an input file".to_owned())
    })?;
    let mut result = OsString::new();
    let mut remaining = template;

    while let Some(start) = remaining.find("${") {
        result.push(&remaining[..start]);
        let after_open = &remaining[start + 2..];
        let end = after_open.find('}').ok_or_else(|| {
            ExecutionError::InvalidConfig("unterminated placeholder in argument".to_owned())
        })?;
        let placeholder = &after_open[..end];
        let value: &OsStr = match placeholder {
            "input" => input.as_os_str(),
            "input_dir" => input.parent().unwrap_or_else(|| Path::new(".")).as_os_str(),
            "input_stem" => input.file_stem().unwrap_or_else(|| OsStr::new("")),
            "input_ext" => input.extension().unwrap_or_else(|| OsStr::new("")),
            "output" => output
                .ok_or_else(|| {
                    ExecutionError::InvalidConfig(
                        "`${output}` requires a staged output path".to_owned(),
                    )
                })?
                .as_os_str(),
            "work_dir" => work_dir.as_os_str(),
            unknown => {
                return Err(ExecutionError::InvalidConfig(format!(
                    "unknown argument placeholder `${unknown}`"
                )));
            }
        };
        result.push(value);
        remaining = &after_open[end + 1..];
    }
    result.push(remaining);
    Ok(vec![result])
}

fn resolve_program(program: &str) -> io::Result<PathBuf> {
    let path = Path::new(program);
    if path.components().count() > 1 {
        return absolute(path);
    }
    Ok(path.to_path_buf())
}

#[cfg(windows)]
fn reject_windows_batch_file(program: &str) -> Result<(), ExecutionError> {
    let file_name = Path::new(program)
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or(program)
        .trim_end_matches([' ', '.']);
    let is_batch_file = file_name.rsplit_once('.').is_some_and(|(_, extension)| {
        extension.eq_ignore_ascii_case("bat") || extension.eq_ignore_ascii_case("cmd")
    });
    if is_batch_file
        || file_name.eq_ignore_ascii_case("cmd")
        || file_name.eq_ignore_ascii_case("cmd.exe")
    {
        return Err(ExecutionError::InvalidConfig(
            "Windows command shells and batch files are not valid process executables".to_owned(),
        ));
    }
    Ok(())
}

#[cfg(not(windows))]
fn reject_windows_batch_file(_program: &str) -> Result<(), ExecutionError> {
    Ok(())
}

fn is_powershell(interpreter: &str) -> bool {
    Path::new(interpreter)
        .file_name()
        .and_then(OsStr::to_str)
        .is_some_and(|name| {
            name.eq_ignore_ascii_case("powershell")
                || name.eq_ignore_ascii_case("powershell.exe")
                || name.eq_ignore_ascii_case("pwsh")
                || name.eq_ignore_ascii_case("pwsh.exe")
        })
}

struct ActiveGroup {
    child: AsyncGroupChild,
    active: bool,
}

impl ActiveGroup {
    async fn terminate(&mut self) {
        let _ = self.child.start_kill();
        self.active = false;
        let _ = tokio::time::timeout(Duration::from_secs(1), self.child.inner().wait()).await;
    }
}

impl Drop for ActiveGroup {
    fn drop(&mut self) {
        if self.active {
            let _ = self.child.start_kill();
        }
    }
}

async fn read_pipe(pipe: Option<impl AsyncRead + Unpin>) -> io::Result<Vec<u8>> {
    let mut output = Vec::new();
    if let Some(mut pipe) = pipe {
        pipe.read_to_end(&mut output).await?;
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placeholders_in_inserted_paths_are_literal() {
        let inputs = [PathBuf::from("${output}.txt")];
        let expanded = expand_argument("prefix=${input}", &inputs, None, Path::new("."));
        assert_eq!(expanded.unwrap(), [OsString::from("prefix=${output}.txt")]);
    }

    #[cfg(windows)]
    #[test]
    fn windows_rejects_normalized_batch_paths() {
        for program in ["tool.bat", r"C:\tools\helper.CMD. ", "cmd.exe"] {
            assert!(reject_windows_batch_file(program).is_err());
        }
        assert!(reject_windows_batch_file("allowed.exe").is_ok());
    }
}
