## Terraform taint

Resources tainted by terraform when it fails to create or modify it. When resources tainted, resource gets recreated when running terraform plan/apply.  

You can taint/untaint resources manualy.  
`terraform taint aws_security_group.web_SG`  
`terraform untaint aws_security_group.web_SG`  

Terrafrom replace is prefered over taint.  
`terraform apply -replace=aws_security_group.web_SG`  
