// This is free and unencumbered software released into the public domain.

import child_process
import child_process/stdio

/// Execute the `distrib` binary with the given arguments, inheriting
/// stdin, stdout, and stderr from the calling process. Returns the
/// process output (including the exit status code) on success, or a
/// `StartError` if the binary could not be launched.
pub fn run(args: List(String)) -> Result(child_process.Output, child_process.StartError) {
  child_process.from_name("distrib")
  |> child_process.args(args)
  |> child_process.run(stdio.inherit())
}
