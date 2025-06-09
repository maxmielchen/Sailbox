# lib/users.sh
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
. "$SCRIPT_DIR/user_create.sh"
. "$SCRIPT_DIR/user_delete.sh"
. "$SCRIPT_DIR/helpers.sh"

# Fassade für Kompatibilität
# user_create und user_delete werden direkt importiert
