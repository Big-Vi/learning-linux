## Splat expressions(*)

Replacement for `for` loop. `[*]` symbol iterates over all the elements of the list to its left and access the attribute on the right.  

```
output "to_ports" {
	value = aws_security_group.web_SG.ingress[*].from_port
}
```
