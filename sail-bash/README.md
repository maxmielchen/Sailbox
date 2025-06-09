# sail-bash

Alternative to `sail` completely in Bash. Provides the same CLI commands for user management (create/delete, root/sudo).

## Usage

```bash
./sail-bash.sh user create [--username USER] [--password PASS] [-r] [-s]
./sail-bash.sh user delete [--username USER]
./sail-bash.sh help
```

### Commands and Parameters

- `user create`  
  Create a new user with the following options:
  - `--username USER`   : The username for the new user. If omitted, you will be prompted interactively.
  - `--password PASS`   : The password for the new user. If omitted, you will be prompted interactively (input is hidden).
  - `-r`                : Add the user to the `root` group (root privileges).
  - `-s`                : Add the user to the `sudo` group (sudo privileges).

- `user delete`  
  Delete an existing user:
  - `--username USER`   : The username to delete. If omitted, you will be prompted interactively.

- `help` or `--help` or `-h`  
  Show this help message with all available commands and options.

### What happens when you create a user?
- The user is created on the system.
- The password is set.
- The user is added to the SSH AllowUsers list.
- A project directory is created at `/home/USER/projects`.
- The user is added to the `docker` group (Docker access).
- Optionally, the user is added to the `root` and/or `sudo` group if specified.

### What happens when you delete a user?
- The user is removed from the system (including home directory).
- The user is removed from the SSH AllowUsers list.

---

**Note:** After creating or deleting a user, you should reboot the Sailbox to fully apply the changes.
