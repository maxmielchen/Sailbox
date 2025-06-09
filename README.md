![repo](https://img.shields.io/github/repo-size/maxmielchen/sailbox?style=flat-square)
![count](https://img.shields.io/github/directory-file-count/maxmielchen/sailbox?style=flat-square)
![top](https://img.shields.io/github/languages/top/maxmielchen/sailbox?style=flat-square)
![count](https://img.shields.io/github/languages/count/maxmielchen/sailbox?style=flat-square)
![sailbox](https://img.shields.io/github/actions/workflow/status/maxmielchen/sailbox/docker-publish.yml?label=sailbox%3Alatest&style=flat-square)

# Sailbox

Sailbox is a Docker image that simplifies setting up a remote development environment. It includes essential programming tools and provides CRUD options for managing SSH users. With Sailbox, you can easily set up a remote environment for any IDE or even without an IDE.

---

## Table of Contents
- [Features](#features)
- [Quickstart (Docker CLI)](#quickstart-docker-cli)
- [Quickstart (Docker Compose)](#quickstart-docker-compose)
- [Sail CLI Commands](#sail-cli-commands)
- [Useful Commands](#useful-commands)

---

## Features
- SSH server out-of-the-box
- User management (CRUD) via the `sail` Bash CLI
- Docker-in-Docker support
- Preinstalled: Git, Vim, Neovim, Nano, curl, gh

---

## Quickstart (Docker CLI)

### Pull the image
```bash
docker pull ghcr.io/maxmielchen/sailbox:latest
```

### Start a container
```bash
docker run --name box -p 201:22 -v /var/run/docker.sock:/var/run/docker.sock -d ghcr.io/maxmielchen/sailbox:latest
```

### Add a user
```bash
docker exec -it box sail user create --username OUR_USERNAME --password OUR_PASSWORD -r -s
```

### Restart the container
```bash
docker restart box
```

---

## Quickstart (Docker Compose)

### Example `docker-compose.yml`
```yaml
services:
  sailbox:
    image: ghcr.io/maxmielchen/sailbox:latest
    container_name: box
    ports:
      - "201:22"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
```

### Start
```bash
docker compose up -d
```

### Add a user
```bash
docker exec -it box sail user create --username OUR_USERNAME --password OUR_PASSWORD -r -s
```

---

## Sail CLI Commands

With the `sail` command (a symlink to the Bash script), you can manage users:

- **Create a user:**
  ```bash
  sail user create --username USER --password PASS [-r] [-s]
  # -r: Create user as root
  # -s: Generate SSH key
  ```
- **Delete a user:**
  ```bash
  sail user delete --username USER
  ```
- **Show help:**
  ```bash
  sail help
  ```

---

## Useful Commands

- **Show logs:**
  ```bash
  docker logs box
  ```
- **Stop the container:**
  ```bash
  docker stop box
  ```
- **Remove the container:**
  ```bash
  docker rm box
  ```

---

For more information, see the documentation in the `sail-bash/` folder.
