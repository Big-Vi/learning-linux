## Aliases

Aliases can be used to define multiple configuration for the same provider.  

E.g,

```
provider "aws" {

	region = "ap-southeast-1"

}

provider "aws" {

	region = "us-west-1"

	alias = "usa"

}

resource "aws_instance" "web" {

	ami = "ami-0c443j89eke00"

	instance_type = "t3.micro"

	provider = "aws.usa"

}
```
