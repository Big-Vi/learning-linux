## Terraform Commands

`terraform validate`  
`terraform fmt` - To format  

`terraform show`  
`terraform providers`  

`terraform output`  
`terraform plan` or `terraform refresh`  

`terraform graph` - Visual representaion of dependencies and configuration  

To view and manipulate state file.  
`terraform state <subcommand> [options] [args]`  

E.g,  
`terraform state show aws_s3_bucket.web`  
`terraform state list`  
`terraform state mv [options] <source> <destination>`  
`terraform state pull` - To pull it from remote state backend.  
`terraform state rm <resource-address>`  

E.g,  
`terraform state rm aws_s3_bucket.web`  

`terraform state push ./terraform.tfstate` - To push the local state to remote state.  
