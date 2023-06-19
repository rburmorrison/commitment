# Commitment

[![Crates.io](https://img.shields.io/crates/v/commitment.svg)](https://crates.io/crates/commitment)
[![Crates.io](https://img.shields.io/crates/l/commitment)](./LICENSE.txt)

Simplify your Git pre-commit hooks.

## Usage

> **WARNING**
> 
> Commitment files contain arbitrary shell commands. Be cautious when installing
> a Commitment file and always review first!

To use Commitment, start by creating a `commitment.yml` file in the root
directory of your project. See the next section for an example.

Commitment files need to be installed before they take effect. To install a
Commitment file, run `commitment install` in the root of your project.

## Example Commitment File

```yaml
# Tasks are defined below. They run sequentially. If one fails, the following
# tasks are skipped and Commitment returns an error code.
 
cargo-build:
  # This must be defined for every task. Multiple commands can be specified and
  # will be executed within the same shell session. This means you can change
  # directories and run commands there.
  execute:
    - cargo build --color=always

cargo-fmt:
  execute:
    - cargo fmt --check

cargo-clippy:
  execute:
    - cargo clippy --color=always -- -D warnings
```

## Output

When Commitment executes a file each task and command within the task will be
output. For the commands themselves, the `stdout` and `stderr` will be displayed
as the command runs with line numbers. Line numbers colored in red mean that
that line came from `stderr`.

After all tasks are executed, the results will be displayed like this:

```
╔═══════════════════════════╗
║          RESULTS          ║
╚═══════════════════════════╝

 cargo-build....................SUCCESS
   cargo-fmt....................SKIPPED
cargo-clippy....................SUCCESS

PASSED: 2/3 (66.67%)
```

Possible statuses are:

| **Status**  | **Description**                                |
|-------------|------------------------------------------------|
| SUCCESS     | The task finished without errors               |
| FAILURE     | The task failed to complete                    |
| SKIPPED     | The task failed, but `can-fail` was `true`     |
| IGNORED     | A previous task has failed and this wasn't run |
