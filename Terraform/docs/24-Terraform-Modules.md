## Terraform modules

Terraform has one root module by default and it can have child module.  

Below configuration refer child module from root module.   

```
module "web" {
	source = "<child-module-path>"
	// Variable goes here
}
```

source meta argument from the above block can be from terraform registry and it's version controlled.   
