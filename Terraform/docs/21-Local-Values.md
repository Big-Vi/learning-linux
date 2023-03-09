## Local values

To simplify repeated configuration, use local values.  

```
locals {
	common_tags = {
		Department = "IT"
	}
}

resource "aws_instance" "web" {
	ami = "ami-0c443jnorerg"
	instance_type = "t3.micro"
	tags = local.common_tags
}
```
