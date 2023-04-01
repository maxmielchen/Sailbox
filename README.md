![repo](https://img.shields.io/github/repo-size/maxmielchen/sailbox?style=flat-square)
![count](https://img.shields.io/github/directory-file-count/maxmielchen/sailbox?style=flat-square)
![top](https://img.shields.io/github/languages/top/maxmielchen/sailbox?style=flat-square)
![count](https://img.shields.io/github/languages/count/maxmielchen/sailbox?style=flat-square)
![sailbox](https://img.shields.io/github/actions/workflow/status/maxmielchen/sailbox/docker-publish.yml?label=sailbox%3Alatest&style=flat-square)

# Sailbox
Sailbox is a Docker image that simplifies the process of setting up a remote development environment. It includes basic tools for programming and provides CRUD options for managing SSH users. With Sailbox, you can easily set up a remote environment for any IDE or IDE-less coding setup.

## Basic usage

Pull latest release
```Bash
docker pull ghcr.io/maxmielchen/sailbox:latest && docker image tag ghcr.io/maxmielchen/sailbox:latest sailbox:latest
```

Pull from main
```Bash
docker pull ghcr.io/maxmielchen/sailbox:latest && docker image tag ghcr.io/maxmielchen/sailbox:main sailbox:latest
```

Run instance
```Bash
docker run --name box -p 201:22 -d sailbox:latest
```

Add user
```Bash
docker exec -it box sail user create --username OUR_USERNAME --password OUR_PASSWORD -r -s
```

Restart instance
```Bash
docker start box
```
