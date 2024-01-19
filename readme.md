# dbman

Create and drop databases, and an associated role for postgres in kubernetes.

You will need a seperate postgres server/cluster to host the databases.

## Installing with Helm
```
helm install oci://ghcr.io/hef/charts/dbman
```

## Usage

Create 2 resource: a DatabaseServer to point to a postgres cluster, and a Database to create a pg database and pg role on that cluster

e.g.

```yaml
apiVersion: dbman.hef.sh/v1alpha2
kind: DatabaseServer
metadata:
  name: postgres
  namespace: database
spec:
  conn_string: "host=postgres-rw.database"
  credentials:
    basicAuthSecretRef: superuser-secret
```

```yaml
apiVersion: dbman.hef.sh/v1alpha3
kind: Database
metadata:
  name: db1
  namespace: database
spec:
  credentials:
    basicAuthSecretRef: db1-credentials
  databaseName: db1
  databaseServerRef:
    namespace: database
    name: postgres
```

dbman will create a database with the name db1, and a role with username and password specified in db1-credentials


## Heritage

dbman will add a pg comment to any database or role that it creates, and will refuse to modify or delete a database or
role that does not have these comments, or that the comment doesn't match the k8s resource that is being reconciled.

Error Messages should contain the expected comment when the comment is missing or incorrect, you may apply it yourself 
in order to adopt a database or role into dbman's management.

## `prune` Flag

by default, dbman will delete a database and role when the k8s resource gets deleted.  If you want to delete the 
resource without delete the database and role, set the `prune: false` flag before deleting the resource.

e.g.

```yaml
apiVersion: dbman.hef.sh/v1alpha2
kind: Database
metadata:
  name: db1
  namespace: database
spec:
  credentials_secret: db1-credentials
  database_name: db1
  database_server_ref:
    namespace: database
    name: postgres
  prune: false
```

# Breaking Changes

## v0.119.0 introduces a number of breaking changes, and a new version of the CRD.

### Database/v1alpha2 -> Database/v1alpha3

The following spec fields have been renamed:

 * `credentials_secret` -> `credentials.basicAuthSecretRef`
 * `database_name` -> `databaseName`
 * `database_server_ref` -> `databaseServerRef`

### DatabaseServer/v1alpha1 -> DatabaseServer/v1alpha2

The following spec fields have been renamed:

 * `credentials_secret` -> `credentials.basicAuthSecretRef`
 * `conn_string` -> `connString`

## Testing
The integration tests require a kind cluster.

```
kind create cluster
cargo test
```

## Similar Projects

 * https://github.com/crossplane-contrib/provider-sql
 * https://github.com/movetokube/postgres-operator
 * https://github.com/bonsai-oss/external-db-operator
