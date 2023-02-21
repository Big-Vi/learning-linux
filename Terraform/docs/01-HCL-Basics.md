## HCL Basics

HCL is made up of blocks.  

E.g,

```
resource "aws_instance" "web" {

	ami = "ami-0c443im590fg6"

	instance_type = "t3.micro"

	tags = {

		Name = "web-${<resourc-type>.<resource.name>.id}

	}

}
```

block name -> resource  
resource type -> aws(provider)_instance(_resource)  
resource name -> web  

Arguments are defined inside block.  
Tags argument in above block is using `resource attribute reference` method.  

`terrafrom init/plan/show/apply/destroy`  

`init` command downloads neccessary plugins and `plan` command is to show the preview of what is going to happen when the terraform `apply` command runs.  

`show` command inspects the terraform state and displays the details.  
