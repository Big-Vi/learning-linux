## Provisioners

Provisioners enables us to run scripts in remote machine.  
Remote Exec provisioner would run commands after resource is created.  

Local machine needs to be connected with remote resources. Use security group & SSH to connect.  

Use connection block to ssh into remote server.  

```
resource "aws_instance" "web" {
	ami = "ami-0c443jnorerg"
	instance_type = "t3.micro"
	provisioner "remote-exec" {
		inline = [
			"sudo apt update",
			"sudo apt install nginx -y"
		]
	}

	connection {
		type = "ssh"
		host = self.public_ip
		user = <user>
		private_key = file(<file-path>)
	}
	key_name = ""
	vpc_security_group_ids = ""
}
```

Local Exec provisioner:  
To run task on the local machine.  

Create time provisioner:  
By default provisoner runs after resource is created.

Destroy time provisioner:  
To run provisioner before resource gets destroyed.  

Failure behavior:  

Terraform recommends to use provisioner as last resort. Make use of provider supported options. - For example use `user data` for AWS.  
