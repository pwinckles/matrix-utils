# matrix-utils

## filter-keys

Filters an encrypted room keys export file to only include keys for the specified rooms. This allows you to share
room keys for specific rooms with users who may have lost their keys.

The filtered file is written to `filtered-KEY_FILE`, where `KEY_FILE` is the name of the input file, and encrypted
using the same passphrase as the input file.

### Usage

```shell
filter-keys KEY_FILE ROOM_ID [ROOM_ID...]
```

For example:

```shell
filter-keys element-keys.txt '!roomid:example.org' '!roomid-2:example.org' 
```
