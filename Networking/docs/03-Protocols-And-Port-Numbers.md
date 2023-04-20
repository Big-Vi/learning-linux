## Protocols & Port numbers

## Application layer protocols:

### Transferring Data

HTTP(80) or HTTPS(443)

Files are transferred in the format of hypertext. A formatted document(HTML) is readable by the web browser.

Web browser run software to process this hypertext. The server uses software (apache, nginx) to serve these files.

Using ports, the web server understands where to send traffic to.

### File transfer

To transfer files from client to server and vice versa, the below protocols are used:

- FTP(20/21) - File transer protocol

- sFTP(22) - Secure version

- TFTP(69) - Trivial file transfer protocol - For tiny files - No auth required.

- SMB(445) - For Microsoft or Linux - Server Message Block - Network file share. Anybody can access files in the network.

FTP client - Filezila for Window.

### Email

Transfer files in the form of an email.

The client uses POP3 & IMAP to get mail from the server. SMTP sends mail from the client to the server and forwards it to the recipient.

- POP3(110/995) - Post Office Protocol

- IMAP(143/993) - Internet Message Access Protocol

- SMTP(25/465) - Simple Mail Transfer Protocol

Thunderbird - Microsoft email client.

### Authentication

Lightweight Directory Access Protocol(LDAP). It's used in Microsoft Active Directory.

LDAP(389)

LDAPS(636) - Encrypted

### Network services

Dynamic Host Configuration Protocol(DHCP) -

Dynamically generates IP addresses for devices.

DHCP(67/68)

### Domain Name System(DNS)

DNS allows us to use a simple name in the browser instead of the server's IP. The request will be sent to the DNS server first to get the IP address of that domain name.

DNS(53)

#### DNS over HTTPS(DoH):

DNS resolution is done via HTTPS. So that DNS queries and responses are encrypted and sent over HTTP protocol(443).

### NTP

Network Time Protocol

NTP automatically synchronise the time on devices using NTP Server.

UTC - Coordinated Universal Time

UTC allows us to accommodate different time zones.

How does it work?

The imaginary line passes through the north pole to the south pole of the earth. it has zero marker and it passes through Greenwich England. That line is called Prime Meridian. The time here at midnight is 00:00.

Everything else is measured against UTC.

E.g., India -> UTC +5:00

USA -> UTC -7:00

## Network Management

Utilities:

- Telnet(23)

- SSH(22) - Encrypted

- SNMP(161/162) - Simple Network Management Protocol - SNMP sends requests to all the connected devices(walk the tree) and stores information(E.g., performance, port, heat, etc).

- Syslog(514) - All the events of connected devices can be sent using Syslog to the central Syslog server.

- RDP(3389) - Remote Desktop Protocol - To remotely manage the server using GUI.

- Audio Visual Protocol

- H.323 Protocol(1720) - E.g., Video conferencing.

- SIP(5060/5061) - Session Initiation Protocol - Voice over IP - E.g., Telephone communication

## SQL

- MySQL(3306)

- SQLnet(1521)

- SQL Server(1433)
