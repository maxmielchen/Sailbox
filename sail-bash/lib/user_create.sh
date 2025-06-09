# lib/user_create.sh
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/helpers.sh"

user_create() {
  USERNAME=""
  PASSWORD=""
  ROOT=0
  SUDO=0
  while [[ $# -gt 0 ]]; do
    case $1 in
      --username)
        USERNAME="$2"; shift 2;;
      --password)
        PASSWORD="$2"; shift 2;;
      -r)
        ROOT=1; shift;;
      -s)
        SUDO=1; shift;;
      *)
        shift;;
    esac
  done
  if [ -z "$USERNAME" ]; then
    USERNAME=$(prompt_username)
  fi
  user_exists "$USERNAME" && { echo "User already exists!"; exit 1; }
  if [ -z "$PASSWORD" ]; then
    PASSWORD=$(prompt_password)
  fi
  add_system_user "$USERNAME"
  if [ $? -ne 0 ]; then echo "Could not create user!"; exit 1; fi
  set_user_password "$USERNAME" "$PASSWORD"
  if [ $? -ne 0 ]; then delete_system_user "$USERNAME"; echo "Could not set password!"; exit 1; fi
  add_ssh_allowuser "$USERNAME"
  create_projects_dir "$USERNAME"
  add_to_group "$USERNAME" "docker"
  if [ $ROOT -eq 1 ]; then
    overwrite_group "$USERNAME" "root" && echo "Successfully added user to root group!" || echo "Could not add to root group."
  fi
  if [ $SUDO -eq 1 ]; then
    add_to_group "$USERNAME" "sudo" && echo "Successfully granted sudo access!" || echo "Could not grant sudo rights."
  fi
  echo "User created successfully!"
  echo "Please reboot the Sailbox to fully initialize the user!"
}
