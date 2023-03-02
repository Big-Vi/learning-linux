## Lifecycle rules

Lifecycle rules determines how terraform creates and destroys resources.  

```
resource "aws_security_group" "web_SG" {
name = "web_SG"
tags = {
	Name = "Web_SG"
}

lifecycle {
	create_before_destroy = true
	prevent_destroy = true
	ignore_changes = [tags]
}
}
```
`create_before_destroy` - Create a resource before destroy the old resource.   
`prevent_destroy` - To prevent it from accidentally deleted.  
`ignore_changes` - Ignore creating resources if the change is small.  
