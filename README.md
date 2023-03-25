# Sailbox
Sailbox is a Docker image that simplifies the process of setting up a remote development environment. It includes basic tools for programming and provides CRUD options for managing SSH users. With Sailbox, you can easily set up a remote environment for any IDE or IDE-less coding setup.


## Basic usage

Create instance 
```Bash
docker pull ghcr.io/maxmielchen/sailbox:latest
docker run --name box -p 201:22 ghcr.io/maxmielchen/sailbox:latest
```

Add user
```Bash
docker exec box -it sail user create
```

Restart instance
```Bash
docker start box
```
