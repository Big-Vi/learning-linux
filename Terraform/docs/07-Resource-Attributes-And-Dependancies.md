## Resource Attributes and Dependancies

To see exported attribute by resource, use `terraform show`.    

Resources attributes can be referenced by below:  

`<resource-type>.<resource-name>.<attribute>`  
E.g,  
`aws_instance.web.key_name`  

Two types of dependancies - implicit and explicit(depends_on argument)

```
resource "aws_instance" "web" {
	ami = "ami-0c443jj78d44g"
	instance_type = "t3.micro"
	depends_on = [
		aws_instance.db
	]
}
```
