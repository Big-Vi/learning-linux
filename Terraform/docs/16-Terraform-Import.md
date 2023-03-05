## Terraform import

To import the existing aws resources into terraform state file.  
`terraform import <resource-type>.<resource-name> <attribute>`  

E.g,
`terraform import aws_instance.web <instance-id>`   

> Note: Terraform imports the resource into state file and doesn't update configuration file. So Configuration file need to be created manually.  
