## Transport Layer Addressing

### Port range
0 - 65,535

### Server Port Numbers
Well-known(0-1023) / Registered(1024-49151) port numbers.

### Client Port Numbers
Ephemeral(49152-65535) port numbers.

### Application layer protocol dependency

Layer7 protocols assigned with layer4 port numbers.

| http | https | ftp | sftp | smb | pop3 | imap | smtp | ldaps | ldap | tftp |
| ---  | ---   | --- | --- | --- | --- | --- | --- | --- | --- | --- | 
| 80   | 443   | 20,21 | 22 | 445 | 110,995 | 143,993 | 25,587 | 636 | 389 | 69 |



| Telnet | ssh | rdp | DNS | sip | H.323 | snmp | dhcp | ntp | 
| ----- | ----- | --- | --- | --- | --- | --- | --- | --- |
|23 | 22 | 3389 | 53 | 5060 | 1719 | 161 | 68,69 | 123 |
