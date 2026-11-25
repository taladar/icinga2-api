# icinga2-api

Rust client for the Icinga2 API

[Official Icinga2 API Docs](https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/)

Currently this implements parts of the query API in both blocking and async
versions

Supported:

* hosts
* services
* host groups
* service groups
* user groups
* dependencies
* check commands
* event commands
* notification commands
* API users
* endpoints
* notifications
* scheduled downtimes
* time periods
* users
* zones
* comments
* downtimes

None of the types listed under Features in the Icinga Documentation are
supported at the moment.

Creation, modification and deletion of config objects is not supported.

All actions are supported but they have not been tested as extensively as the
query API.

## Connecting

Clients can be built from an `Icinga2Instance` (URL, CA certificate, username
and literal password) or with `from_parts` from a URL, an optional CA
certificate and an `Icinga2Auth`, which is either HTTP basic auth or a TLS
client certificate (e.g. the certificate of an Icinga agent for an `ApiUser`
with a `client_cn`).

`config::instance_file::Icinga2InstanceFile` reads instance config files that
several tools can share, one TOML file per Icinga master in a directory
(`Icinga2InstanceFile::load_dir`), with the password from a literal
`password`, from a `password_command` (with optional `password_command_env`)
or with `client_certificate` and `client_key` instead of a username:

```toml
url = "https://icinga.example.com:5665"
ca_certificate = "/path/to/ca.pem"
web_base_url = "https://icinga.example.com/monitoring/"
username = "api-user"
password_command = ["pass", "show", "icinga/api-user"]
password_command_env = { PASSWORD_STORE_DIR = "/path/to/store" }
```

## Event Streams

Event Streams are supported in the async version of the client, supporting all
event types. There is no handling of connection loss within this crate at this
time.

Retrieving status data is not supported yet.

Configuration management is not supported yet.

Retrieving information on configuration object types is not supported yet.

Querying config templates is not supported yet.

Querying global variables is not supported yet.

The Debug Console is not supported yet.
