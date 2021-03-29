# cao

[![Build Status](https://travis-ci.com/Qinka/cao.svg?token=xhmxxSyRqbMx5AwatyVL&branch=main)](https://travis-ci.com/Qinka/cao)

Dynamic DNS tool for DnsPod etc.

## Usage

Show help:
```
$ cao --help
```
or
```
$ cao help
```
or
```
$ cao subcommand --help
```

List all the ip with interface name:
```
$ cao interface
{698DC41D-9989-4B4F-ABCA-74448FB86660}: V4(Ifv4Addr { ip: 10.198.75.60, netmask: 0.0.0.0, broadcast: None })
{11D10073-CED6-4539-AF08-438C56DCBBDA}: V4(Ifv4Addr { ip: 172.20.80.1, netmask: 255.255.31.0, broadcast: Some(172.20.240.255) })
{A0B361C7-A7CD-469C-B7F1-2C07F70FD1C5}: V4(Ifv4Addr { ip: 172.19.86.53, netmask: 255.255.0.0, broadcast: Some(172.19.255.255) })
{2E9330A1-FDBC-11E7-8037-806E6F6E6963}: V6(Ifv6Addr { ip: ::1, netmask: ffff:ffff:ffff:ffff:ffff:ffff:ffff:ffff, broadcast: None })
{2E9330A1-FDBC-11E7-8037-806E6F6E6963}: V4(Ifv4Addr { ip: 127.0.0.1, netmask: 255.0.0.0, broadcast: Some(127.255.255.255) })
```

### Domain record action

```
cao record [add|modify|list|delete] [options]
```

#### Add:
```
cao record -d DOMAIN -k KEY -p PROVIDER add -l RECORD_LINE -t RECORD_TYPE -s SUD_DOMAIN -v VALUE
```
or
```
cao record -d DOMAIN -k KEY -p PROVIDER add -l RECORD_LINE -t RECORD_TYPE -s SUD_DOMAIN -if INTERFACE_NAME
```

#### Modify:
```
cao record -d DOMAIN -k KEY -p PROVIDER modify -i ID -l RECORD_LINE -t RECORD_TYPE -s SUD_DOMAIN -v VALUE
```
or
```
cao record -d DOMAIN -k KEY -p PROVIDER modify -i ID -l RECORD_LINE -t RECORD_TYPE -s SUD_DOMAIN -if INTERFACE_NAME
```

#### List:
```
cao record -d DOMAIN -k KEY -p PROVIDER list -i ID -o OFFSET -l LENGTH -s SUD_DOMAIN
```

#### Delete:
```
cao record -d DOMAIN -k KEY -p PROVIDER delete -i ID
```




