# lib/user_delete.sh
. "$(dirname "$0")/helpers.sh"

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
    read -p "Username: " USERNAME
  fi
  user_exists "$USERNAME" || { echo "User does not exist!"; exit 1; }
  sudo deluser --remove-home "$USERNAME"
  sudo sed -i "/^AllowUsers $USERNAME/d" /etc/ssh/sshd_config
  echo "Successfully delete user!"
  echo "Please reboot die Sailbox, um den User komplett zu entfernen!"
}
