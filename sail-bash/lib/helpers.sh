# lib/helpers.sh

user_exists() {
  id -u "$1" >/dev/null 2>&1
}

prompt_username() {
  read -p "Username: " USERNAME
  echo "$USERNAME"
}

prompt_password() {
  read -s -p "Password: " PASSWORD; echo
  echo "$PASSWORD"
}

add_system_user() {
  sudo adduser --disabled-password --gecos "" "$1"
}

set_user_password() {
  echo "$1:$2" | sudo chpasswd
}

add_ssh_allowuser() {
  echo "AllowUsers $1" | sudo tee -a /etc/ssh/sshd_config > /dev/null
}

create_projects_dir() {
  sudo mkdir -p "/home/$1/projects"
  sudo chown -R "$1" "/home/$1"
}

add_to_group() {
  sudo usermod -aG "$2" "$1"
}

overwrite_group() {
  sudo usermod -G "$2" "$1"
}

delete_system_user() {
  sudo deluser --remove-home "$1"
}

remove_ssh_allowuser() {
  sudo sed -i "/^AllowUsers $1/d" /etc/ssh/sshd_config
}
