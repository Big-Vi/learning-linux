## Network Address Translation - NAT

NAT is network layer function. Because of the IPv4 address shortage, private IPv4 address range created. Internal network can be built using this. NAT is also invented to solve IPv4 address problem(shortage).

When the external device sends back response to the internal local devices, it can't send to private IP address(10.0.0.10). Hence NAT invented. External device sends the response to NAT IP address and NAT then consult NAT table and forward that to private local device.

## Dynamic Host Configuration Protocol - DHCP

It's application layer protocol. DHCP server automatically configures IP address, Subnet Mask, Default Gateway and DNS Server.

### DHCP Binding

Table gets created to map assigned IP address to MAC address(Layer 2) of the device.

DHCP allows to add static IP to the device.
DHCP lease - 7 days default

DHCP server can be hosted outside of the local network.

## Domain Name System - DNS

DNS is application layer protocol. UDP is used to resolve host name to IP.

Uniform Resource Locator - URL

Top Level Domain - TLD
E.g., .com .edu .org .au .nz .net and so on

Second Level Domain
E.g., google.com facebook.com

Third Level Domain
E.g., Hostname  - www.google.com
      Subdomain - www.blog.google.com

It gives the DNS a hierachy to find IP address for specific URL.

### Forward DNS Lookup

If configured DNS server does not know the IP address of the domain name, it search the top level domain server which has record of top level domain. DNS server caches it.

### Reverse DNS Lookup

Reverse DNS Lookup helps to get the domain for the IP.

### DNS Record type

A     - IPv4 Record
AAAA  - IPv4 Record
CNAME - Canonical Name Record - alias
MX    - Mail Exchange Record
NS    - Identifies Authoritative Name Server
PTR   - Pointer Record - Reverse Lookup
SRV   - Service Record

E.g.,
| Name(_service._protocol.domain) | Value(Priority Weight Port Target) |
| --- | --- |
| _imaps._tcp.gmail.com. | 5 0 993 imap.gmail.com |

TXT   - Text Record - Used for authentication - E.g., SPF, Google Domain Verification and DKIM

Internal vs External DNS
Linux Bind Utility - Internal DNS
Internal DNS used in enterprise level.

Google - External DNS
