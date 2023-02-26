## Resource targeting

```
resource "aws_instance" "web" {
	ami = "ami-0c4436yudd70"
	instance_type = "t3.micro"
	tags = {
		Name = "web-${<resourc-type>.<resource.name>.id}
	}
}
```

To apply changes to specific resource target.  

`terraform apply -target <resource-type>.<resource-name>`  
