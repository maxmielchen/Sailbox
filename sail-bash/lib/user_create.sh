# lib/user_create.sh
. "$(dirname "$0")/helpers.sh"

user_create() {
  # ...Argumente parsen wie gehabt...
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
    read -p "Username: " USERNAME
  fi
  user_exists "$USERNAME" && { echo "User already exist!"; exit 1; }
  if [ -z "$PASSWORD" ]; then
    read -s -p "Password: " PASSWORD; echo
  fi
  sudo adduser --disabled-password --gecos "" "$USERNAME"
  if [ $? -ne 0 ]; then echo "Could not create user!"; exit 1; fi
  echo "$USERNAME:$PASSWORD" | sudo chpasswd
  if [ $? -ne 0 ]; then sudo deluser --remove-home "$USERNAME"; echo "Could not set password!"; exit 1; fi
  echo "AllowUsers $USERNAME" | sudo tee -a /etc/ssh/sshd_config > /dev/null
  sudo mkdir -p "/home/$USERNAME/projects"
  sudo chown -R "$USERNAME" "/home/$USERNAME"
  sudo usermod -aG docker "$USERNAME"
  if [ $ROOT -eq 1 ]; then
    sudo usermod -G root "$USERNAME" && echo "Successfully rooting user!" || echo "Could not root"
  fi
  if [ $SUDO -eq 1 ]; then
    sudo usermod -aG sudo "$USERNAME" && echo "Successfully give user sudo access!" || echo "Could not give sudo rights"
  fi
  echo "Successfully create user!"
  echo "Please reboot die Sailbox, um den User komplett zu initialisieren!"
}
