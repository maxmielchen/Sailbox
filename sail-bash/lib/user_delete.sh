# lib/user_delete.sh
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/helpers.sh"

user_delete() {
  USERNAME=""
  while [[ $# -gt 0 ]]; do
    case $1 in
      --username)
        USERNAME="$2"; shift 2;;
      *)
        shift;;
    esac
  done
  if [ -z "$USERNAME" ]; then
    USERNAME=$(prompt_username)
  fi
  user_exists "$USERNAME" || { echo "User does not exist!"; exit 1; }
  delete_system_user "$USERNAME"
  remove_ssh_allowuser "$USERNAME"
  echo "User deleted successfully!"
  echo "Please reboot the Sailbox to fully remove the user!"
}
