## Terraform workspace

To use the same configuration for both dev and prod env, use workspace.  

`terraform workspace list`  
`terraform workspace new <workspace-name>`  

`terraform console` - To view the current workspace and lookup function details.  

`terraform workspace select <workspace-name>` - To switch to different workspace.  

`terraform.tfstate.d` - For workspace setup, terraform stores the state file here.  
