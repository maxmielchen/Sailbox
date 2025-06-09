# cli/dsl.sh
# CLI-Argumente parsen und weiterreichen

parse_cli() {
  local args=("$@")
  if [ ${#args[@]} -lt 1 ]; then
    show_help
    exit 1
  fi
  case ${args[0]} in
    user)
      shift
      parse_user "$@"
      ;;
    help|--help|-h)
      show_help
      ;;
    *)
      show_help
      ;;
  esac
}

show_help() {
  echo "Usage:"
  echo "  sail-bash user create [--username USER] [--password PASS] [-r] [-s]"
  echo "  sail-bash user delete [--username USER]"
  echo "  sail-bash help"
  echo
  echo "Commands:"
  echo "  user create         Create a new user."
  echo "    --username USER   The username for the new user. If omitted, you will be prompted."
  echo "    --password PASS   The password for the new user. If omitted, you will be prompted (input is hidden)."
  echo "    -r                Add the user to the root group (root privileges)."
  echo "    -s                Add the user to the sudo group (sudo privileges)."
  echo "  user delete         Delete an existing user."
  echo "    --username USER   The username to delete. If omitted, you will be prompted."
  echo "  help, --help, -h    Show this help message."
  echo
  echo "After creating or deleting a user, reboot the Sailbox to fully apply the changes."
}
