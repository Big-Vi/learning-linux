## Count and For each 

To create multiple resources, meta arguments(count and for each) can be used.  

```
resource "aws_instance" "web" {
	ami = "ami-0c443j9o4449j"
	instance_type = "t3.micro"
	count = 3
}
```

Or  

```
variabel "webservers" {
	type = list
	default = ["web1", "web2", "web3"]
}

resource "aws_instance" "web" {
	ami = "ami-0c443jnorerg"
	instance_type = "t3.micro"
	count = length(var.webservers)
	tags = {
		Name = var.webservers[count.index]
	}
}
```

Or  

```
variabel "webservers" {
	type = set
	default = ["web1", "web2", "web3"]
}

resource "aws_instance" "web" {
	ami = "ami-0c443jnorerg"
	instance_type = "t3.micro"
	for_each = var.webservers
	tags = {
		Name = each.value
	}
}
```

Last method creates resources as map, so removing specific resource from the default value, would remove the correct resource which is not the case for count.  
