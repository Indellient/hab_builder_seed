# Chef Habitat Builder Depot Seed

This binary provides a way to seed the Chef Habitat Builder Depot database with a user account and auth token.

The general idea and purpose behind this is to link a proxy account so that there is a shift in ownership, no one single
person owns the base origins.

For more information on delivering automation with Chef Habitat Builder Depot we gave a talk at [ChefConf 2019](https://www.indellient.com/blog/bringing-the-chef-habitat-depot-on-premise-in-the-enterprise-2/).

## Usage

For basic user seeding run the following command on the provisioned Chef Habitat Builder Depot instance, where
`<username>` is the username linked to the proxy account in your chosen [oauth system](https://github.com/habitat-sh/on-prem-builder#oauth-application) when provisioning:

```
$ builder_seed --db-host localhost --db-port 5432 --db-user hab --db-name builder --db-pass $( cat
/hab/svc/builder-datastore/files/pwfile ) --keys-dir /hab/svc/builder-api/files seed '<username>'
```

This will print out an auth token, this is the value you need when creating your origins

## Origin Creation

After running the seed command, executing the following curl call will create the core origin. If you'd like to sync
other origins, you will need to create them. You can do so using the same command.

Where `HAB_AUTH_TOKEN` is the output from the above `seed` command.

```
$ curl -vvv http://localhost/v1/depot/origins -H 'content-type: application/json' -H "Authorization: Bearer ${HAB_AUTH_TOKEN}" -d '{"name":"core","default_package_visibility":"public"}'
```
