# dbman

Create and drop databases for postgres in kubernetes.

You will need a seperate postgres server/cluster to host the databases.

## installing with helm
```
helm install oci://ghcr.io/hef/dbman/dbman
```


# testing
```
kind create cluster
cargo test
```
