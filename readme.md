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
  connString: "host=postgres-rw.database"
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


### Heritage

dbman will add a pg comment to any database or role that it creates, and will refuse to modify or delete a database or
role that does not have these comments, or that the comment doesn't match the k8s resource that is being reconciled.

Error Messages should contain the expected comment when the comment is missing or incorrect, you may apply it yourself 
in order to adopt a database or role into dbman's management.

### `prune` Flag

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
  credentials:
    basicAuthSecretRef: db1-credentials
  databaseName: db1
  databaseServerRef:
    namespace: database
    name: postgres
  prune: false
```

### Specifying Credentials

Both the Database CRD and databaseServer CRD have a `credentials` field, which can be used to specify the username and password for the database and role.

```yaml
  credentials:
    basicAuthSecretRef: <secret>
    username: <username>
    usernameConfigMapRef:
      name: <configmap>
      key: <key>
    usernameSecretRef:
      name: <secret>
      key: <key>
    passwordSecretRef:
      name: <secret>
      key: <key>
```

* You can't specify [basicAuthSecretRef](https://kubernetes.io/docs/concepts/configuration/secret/#basic-authentication-secret) and any other field at the same time.
* You can't specify `username` and/or `usernameConfigMapRef` and/or `usernameSecretRef` at the same time,
* You you can't specify both `password` and `passwordSecretRef` at the same time.

### Getting owner from another database CR

You can specify the owner of the database and role by setting the `ownerRef` field to another database CR.

```yaml
---
apiVersion: dbman.hef.sh/v1alpha2
kind: Database
metadata:
  name: db1
  namespace: database
spec:
  databaseName: db1
  credentials:
    username: common-owner
  passwordSecretRef: ... # omitted for brevity
  databaseServerRef: ... # omitted for brevity
---
apiVersion: dbman.hef.sh/v1alpha2
kind: Database
metadata:
  name: db2
  namespace: database
spec:
  databaseName: db2
  ownerRef:
    name: db1
  databaseServerRef: ... # omitted for brevity
  
```

In this example both db1 and db2 will have the same owner role `common-owner`.

## Breaking Changes

### v0.120.0 introduces a number of breaking changes, and a new version of the CRD.

#### Database/v1alpha2 -> Database/v1alpha3

The following spec fields have been renamed:

 * `credentials_secret` -> `credentials.basicAuthSecretRef`
 * `database_name` -> `databaseName`
 * `database_server_ref` -> `databaseServerRef`

#### DatabaseServer/v1alpha1 -> DatabaseServer/v1alpha2

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
