# lib/helpers.sh

user_exists() {
  id -u "$1" >/dev/null 2>&1
}
