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

None of the types listed under Features in the Icinga Documentation are supported
at the moment.

Creation, modification and deletion of config objects is not supported.

All actions are supported but they have not been tested as extensively as the
query API.

Event Streams are supported in the async version of the client, supporting all
event types. There is no handling of connection loss within this crate at this
time.

Retrieving status data is not supported yet.

Configuration management is not supported yet.

Retrieving information on configuration object types is not supported yet.

Querying config templates is not supported yet.

Querying global variables is not supported yet.

The Debug Console is not supported yet.
