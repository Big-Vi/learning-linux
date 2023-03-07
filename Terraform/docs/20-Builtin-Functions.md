## Builtin functions

`terrafrom console` - To get into interactive console and run following functions.  

`file(/main.tf)`  
`toset(var.region)` - To convert list to set hence removes duplicates.  

Numeric functions:  
max, min, ceil, floor  

E.g,
```
variable number {
	type = set(number)
	default = [45, 50, 60]
}

variable map {
	type = map
	default = {
		"dev" = t3.micro
		"prod" = t3.large
	}
}

max(var.number...)
```

String functions:  
split, join, lower, upper, title, substr  

Collection functions:  
`length(var.number)`  
`index(var.number, 50)` - To find index of element in the list.  
`element(var.number, 2)` - To get the particular element in the list.  
`contains(var.number, 50)` - To find if the element exist or not.  

Map functions:  
`keys(var.map)` - To just get keys of map in list.  
`values(var.map)` - To get the values of map in list.  
`lookup(var.map, "dev")` - To lookup the value of key in a map.  
