// Copyright 2018-2024 the Shell authors. MIT license.

use futures::future::LocalBoxFuture;
use miette::bail;
use miette::Result;

use deno_task_shell::{
    parse_arg_kinds, ArgKind, EnvChange, ExecuteResult, ShellCommand, ShellCommandContext,
    ShellOptions,
};

pub struct SetCommand;

impl ShellCommand for SetCommand {
    fn execute(&self, mut context: ShellCommandContext) -> LocalBoxFuture<'static, ExecuteResult> {
        let result = match execute_set(context.args) {
            Ok((code, env_changes)) => ExecuteResult::Continue(code, env_changes, Vec::new()),
            Err(err) => {
                context.stderr.write_line(&format!("set: {err}")).unwrap();
                ExecuteResult::Exit(2, Vec::new(), Vec::new())
            }
        };
        Box::pin(futures::future::ready(result))
    }
}

fn execute_set(args: Vec<String>) -> Result<(i32, Vec<EnvChange>)> {
    let args = parse_arg_kinds(&args);
    let mut env_changes = Vec::new();
    for arg in args {
        match arg {
            ArgKind::ShortFlag('e') => {
                env_changes.push(EnvChange::SetShellOptions(ShellOptions::ExitOnError, true));
            }
            ArgKind::PlusFlag('e') => {
                env_changes.push(EnvChange::SetShellOptions(ShellOptions::ExitOnError, false));
            }
            ArgKind::ShortFlag('x') => {
                env_changes.push(EnvChange::SetShellOptions(ShellOptions::PrintTrace, true));
            }
            ArgKind::PlusFlag('x') => {
                env_changes.push(EnvChange::SetShellOptions(ShellOptions::PrintTrace, false));
            }
            _ => bail!(format!("Unsupported argument: {:?}", arg)),
        }
    }
    Ok((0, env_changes))
}

#[tokio::test]
async fn test_exit_on_error() {
    assert_eq!(
        execute_set(vec!["-e".to_string()]).unwrap(),
        (
            0,
            vec![EnvChange::SetShellOptions(ShellOptions::ExitOnError, true)]
        )
    );

    assert_eq!(
        execute_set(vec!["+e".to_string()]).unwrap(),
        (
            0,
            vec![EnvChange::SetShellOptions(ShellOptions::ExitOnError, false)]
        )
    );

    assert_eq!(
        execute_set(vec!["-x".to_string()]).unwrap(),
        (
            0,
            vec![EnvChange::SetShellOptions(ShellOptions::PrintTrace, true)]
        )
    );

    assert_eq!(
        execute_set(vec!["+x".to_string()]).unwrap(),
        (
            0,
            vec![EnvChange::SetShellOptions(ShellOptions::PrintTrace, false)]
        )
    );

    assert!(execute_set(vec!["-t".to_string()]).is_err());
}
