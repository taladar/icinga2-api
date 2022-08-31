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
