set dotenv-load

# Prints available commands
default:
  @just --list

# Run cargo formatter
fmt +ARGS='':
  cargo +nightly fmt {{ARGS}}

# printctl CLI
printctl +ARGS='--help':
    cargo run -- {{ARGS}}

# Sets up the basic .env file
setup:
  #!/usr/bin/env bash
  cat <<EOF > .env
  EOF
  echo "Created local .env config"
  cat <<EOF > .envrc
  use nix
  EOF
  echo "Created local .envrc config"
