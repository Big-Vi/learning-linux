## Variables

```
variable "separator" {
	default = "|"
}
```

E.g,

```
resource "aws_instance" "web" {
	ami = "ami-0c443jnorerg"
	instance_type = "t3.micro"
	separator = var.separator
}
```

If default value is not specified, you will prompted to enter values when running `terraform apply`.   

or  

It can be passed as command line flag variables.  

`terraform apply -var "ami=ami-0h98ff4e4440"`  
 
or  

Environment variables can be used too.  

`export TF_VAR_instance_type = "t3.micro"`  

or  

Variable definition file  

`<name>.tfvars`  


Variable Definition Precedence,highest to lowest:  

	1 - Command line flags  

	2 - *.auto.tfvars  

	3 - Terraform tfvars file  

	4 - Environment variable  


```
variable "ami" {
	default = ""
	description = ""
	type = string
	sensitive = true
	validation {
		condition = substr(var.ami, 0, 4) == "ami-"
		error_message = "The AMI should start with ami-"
	}
}
```

`sensitive` means variable value would be suppressed when running `terraform apply` or other commands.  

## Data types:  

int -> 1  

string -> "string"  

boolean -> true/false  

object  

list -> ["string", "string"]  

set -> ["string", "string1"]  

map -> "dev" = "t3.micro"  

tuple -> ["web", 5]  

Set is similar to list but it can't have duplicate elements.  

Object data type is complex data types and it can contain all other data types.  

Tuple is similar to list but it can have different data types.  


Type constraints
```
variable "servers" {
	default = ["web", "db"]
	type = list(string)
}
```  
