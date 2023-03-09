## Dynamic blocks

```
variable "ingress_ports" {
	type = list
	default = [22, 80]
}

resource "aws_security_group" "web_SG" {
	name = "web_SG"
	dynamic "ingress" {
		iterator = port
		for_each var.ingress_ports
		content {
			from_port = port.value
			to_port = port.value
			protocol = "tcp"
			cidr_blocks = ["0.0.0.0/0"]
		}
	}
}
```
