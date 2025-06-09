#!/bin/bash
# sail-bash: Alternative zu sail in Bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Module einbinden
. "$SCRIPT_DIR/cli/dsl.sh"
. "$SCRIPT_DIR/cli/action.sh"
. "$SCRIPT_DIR/lib/users.sh"

show_help() {
  echo "sail-bash user create [--username USER] [--password PASS] [-r] [-s]"
  echo "sail-bash user delete [--username USER]"
}

main() {
  parse_cli "$@"
}

main "$@"
