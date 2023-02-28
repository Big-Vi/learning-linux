## Terraform state

When running `terraform apply` command, it creates state(.tfstate) file. Terraform uses it as single source of truth or blueprint.  

State files may endup with sensitive data. So it should be stored in secure remote state backend(S3 or Terraform Cloud).  

To disable the state refresh.  
`terraform apply -refresh=false`  
