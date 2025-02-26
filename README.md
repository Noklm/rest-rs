# rest-rs
REST API in rust

## Development
First start the dev database:
```sh
podman run -d \
    --name dbdev \
    --publish 5432:5432 \
    --env POSTGRES_USER=axum \
    --env POSTGRES_DB=dbdev \
    --env POSTGRES_PASSWORD=password \
    --mount type=volume,src=db-vol,destination=/var/lib/postgresql/data,rw=true,relabel=private \
	postgres:alpine
```

Remove database volume:
```sh
podman volume rm db-vol
```

Watch and run dev server with [Bacon](https://dystroy.org/bacon/)
```sh
bacon run-long
```
