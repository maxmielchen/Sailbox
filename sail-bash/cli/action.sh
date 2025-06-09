# cli/action.sh
# Aktionen für user create/delete
. "$(dirname "$0")/../lib/users.sh"

parse_user() {
  case $1 in
    create)
      shift
      user_create "$@"
      ;;
    delete)
      shift
      user_delete "$@"
      ;;
    *)
      show_help
      ;;
  esac
}
