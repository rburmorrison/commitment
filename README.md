# Commitment

Simplify your Git pre-commit hooks.

## Example Commitment File

```yaml
# Set task defaults here. If not specified, reasonable defaults will be set for
# you.
defaults:
  can-fail: false

# This is an examle of a task. You can specify as many as you'd like.
cargo-clippy:

  # Define the steps it takes for this task to complete here.
  execute:
    - cargo clippy -- -D warnings
```
