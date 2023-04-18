## OSI Model - Open System Interconnect

OSI Model provides a standard for different computers to be able to communicate with each other.

### Physical Layer(1)

Cables are used to connect the local computer to router to the modem(ISP) to the internet to the remote computer.

Physical layer protocols define how these cables are constructed.

Cable types:

- twisted pair - copper cable twisted together to achieve efficiency.
- CoAx cable(modem)
- Fiber optics(internet)

### Data Link Layer(2)

It's where traffic happens locally from one device to another device in short hops.

DOCSIS3 protocol - Data or cable service interface specification - used by ISP

Ethernet cable allows to communicate from one device to another device.

### Network Layer(3)

For long hops, the network layer allows to move data from one side of the internet to another side of the internet using IP addresses.

IP(Internet Protocol) Addressing - It's similar to your house address.
IP routing allows us to send data from one IP to another IP.

### Transport Layer(4)

The session needs to be set up before any communication happens. Transmission control protocol(TCP) sets up the session.

### Session Layer(5)

To keep conversations separate to prevent mix-up of data.

Steps:
Setup -> Maintain -> Tear down

Protocols/Devices:
- H.203 -> Streaming audio/video. E.g., Facebook Or Skype. Uses Real Time Protocol(RTP).
- NetBios -> To share files over a network.

### Presentation Layer(6)

ASCII(American Standard Code for Information Interchange) - Converts letters from our keyboard into hexadecimal values. It's a coding standard for encoding text.

IBM had EBCDIC(Extended Binary Coded Decimal Interchange Code) encoding system. IBM's encoding system needed to be translated to the ASCII encoding system. This is where the presentation layer has some protocol to allow this to happen.

So encryption and formatting of pictures used to happen at the presentation layer. In modern networks, it happens within the application layer. EBCDIC is dead now.

### Application Layer(7)

HTTP(HyperText Transfer Protocol) allows to transfer data from server to browser.

HTTPS is an encrypted version.

Hypertext is simply a file.
