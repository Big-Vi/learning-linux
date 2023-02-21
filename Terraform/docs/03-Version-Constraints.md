## Version constraints

To constraint the Terraform provider plugin using Terraform block.  

```
terraform {
	required_providers {
		local = {
			source = "hashicorp/local"
			version = "1.4.0"
		}
	}
}
```

Use comparision operator(!=, <, >) for version constraints.  

tilda greater than(~>) operator means equal to that version or any incremental version(minor or patch).("~>1.4", "~>1.4.0")  
