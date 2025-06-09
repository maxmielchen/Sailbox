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
