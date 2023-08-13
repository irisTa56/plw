# Learn Actix-Leptos

## Setup and run end-to-end testing

```shell-session
$ cd end2end
$ npm install
$ npx playwright install
$ cd ..
$ cargo leptos end-to-end
```

## Build and run Docker container

```shell-session
$ docker-buildx build -t "<image>:<tag>" .
$ docker run -it -p 8080:8080 --rm <image>
```
