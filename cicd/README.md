# How to deploy the object storage

The deployment consists of a minio with a nginx reverse proxy in front of it.

## 0. Prerequisites

- Vm to deploy the object storage
- Domain for the object storage

## 1. Create the selfsigned cert 

Clone the repository,

switch to `./cicd/nginx` and run 

```
openssl req -nodes -new -x509 -keyout nginx.key -out nginx.crt
```

## 2. Deploy the object storage

Fill out the templated values `<...>` in the docker-compose.yaml,

switch to `./cicd` and run 

```
docker-compose up -d
```

`-d` is optional and runs the docker command in the background.