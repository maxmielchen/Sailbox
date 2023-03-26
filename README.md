![repo](https://img.shields.io/github/repo-size/maxmielchen/sailbox?style=flat-square)
![count](https://img.shields.io/github/directory-file-count/maxmielchen/sailbox?style=flat-square)
![top](https://img.shields.io/github/languages/top/maxmielchen/sailbox?style=flat-square)
![count](https://img.shields.io/github/languages/count/maxmielchen/sailbox?style=flat-square)
![sailbox](https://img.shields.io/github/actions/workflow/status/maxmielchen/sailbox/docker-publish.yml?label=sailbox%3Alatest&style=flat-square)

# Sailbox
Sailbox is a Docker image that simplifies the process of setting up a remote development environment. It includes basic tools for programming and provides CRUD options for managing SSH users. With Sailbox, you can easily set up a remote environment for any IDE or IDE-less coding setup.

## Basic usage

Create instance 
```Bash
docker pull ghcr.io/maxmielchen/sailbox:latest && docker image tag ghcr.io/maxmielchen/sailbox:latest sailbox:latest
docker run --name box -p 201:22 sailbox:latest
```

Add user
```Bash
docker exec box -it sail user create
```

Restart instance
```Bash
docker start box
```
