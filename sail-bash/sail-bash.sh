#!/bin/bash
# sail-bash: Alternative zu sail in Bash

# Module einbinden
. "$(dirname "$0")/cli/dsl.sh"
. "$(dirname "$0")/cli/action.sh"
. "$(dirname "$0")/lib/users.sh"

show_help() {
  echo "sail-bash user create [--username USER] [--password PASS] [-r] [-s]"
  echo "sail-bash user delete [--username USER]"
}

main() {
  parse_cli "$@"
}

main "$@"
