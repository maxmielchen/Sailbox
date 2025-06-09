# lib/users.sh
. "$(dirname "$0")/user_create.sh"
. "$(dirname "$0")/user_delete.sh"
. "$(dirname "$0")/helpers.sh"

# Fassade für Kompatibilität
# user_create und user_delete werden direkt importiert
