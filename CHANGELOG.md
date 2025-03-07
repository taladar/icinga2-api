## 0.2.13

update dependencies

## 0.2.12

update dependencies

## 0.2.11

update dependencies

## 0.2.10

update dependencies

## 0.2.9

update dependencies

## 0.2.8

update dependencies

## 0.2.7

update dependencies

## 0.2.6

update dependencies

## 0.2.5

upgrade dependencies
update deny.toml to new format

## 0.2.4

upgrade dependencies

## 0.2.3

upgrade dependencies

## 0.2.2

upgrade dependencies

## 0.2.1

upgrade dependencies

## 0.2.0

update dependencies including some incompatible ones

## 0.1.17

fix SPDX license expression in Cargo.toml
replace dotenv (unmaintained) with dotenvy
update dependencies to get new versions of tracing-subscriber and pretty-assertions without unmaintained dependencies

## 0.1.16

fix private fields in action response types that were not supposed to be private

## 0.1.15

allow all uppercase Compat deserialization names for notification type enum

## 0.1.14

fix deserialization of state field in StateChange event stream event

## 0.1.13

missed adding QueryableObject implementations in the macro generating implementations
without joins originally, this version fixes that

## 0.1.12

add derived Copy instances to enums

## 0.1.11

add method to CustomVarHolder trait to deserialize the serde\_json::Value directly
into a user supplied type

## 0.1.10

add trait CustomVarHolder to make retrieving custom vars easier since they
are often in a relatively deeply nested field

## 0.1.9

add optional Enumoid instances to enums (feature flag enumoid; default off)

## 0.1.8

add PartialEq and Eq instances to more types

## 0.1.7

add Queryable trait to allow writing of generic code for list queries without
any filtering (ListHosts for all hosts, ListServices for all services,...)

## 0.1.6

optionally split config loading and use to allow using config that was previously
loaded or config that wasn't stored in a TOML file at all

## 0.1.5

Add Hash, PartialEq, Eq, PartialOrd, Ord to enums and other types where appropriate/possible

## 0.1.4

Make clients Clone to support cloning to use them in multiple async tasks

## 0.1.3

Add Hash, PartialEq, Eq, PartialOrd, Ord to *Name types

## 0.1.2

Add Clone instances

## 0.1.1

Adjust visibility of fields in Icinga2 and Icinga2Async

## 0.1.0

Initial Release
