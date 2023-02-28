## Data sources

To use an existing resources or resources created by other resources(ansible, cloudformation and etc), use data block.  

```
data "aws_key_pair" "web" {
	key_name = "<existing-key-name>"
}
```
