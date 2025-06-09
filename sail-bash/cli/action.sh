# cli/action.sh
# Aktionen für user create/delete

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
