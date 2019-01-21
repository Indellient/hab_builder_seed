# Builder Seed

This binary provides a way to seed the builder database with a user account and auth token.

## Usage

```
$ builder_seed --db-host localhost --db-port 5432 --db-user hab --db-name builder --db-pass $( cat
/hab/svc/builder-datastore/files/pwfile ) --keys-dir /hab/svc/builder-api/files seed 'username'
```

## Notes

Homebrew installed `pkg-config`

export PKG_CONFIG_PATH="/usr/local/opt/libarchive/lib/pkgconfig"

