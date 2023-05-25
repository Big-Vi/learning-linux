Golang is Compiled programming language meaning that code compiles machine-readable binary code.  

> main.go
```
func main() {
	fmt.Println("Hello")
}
```
The main function is the entry point for the executable program. It's not mandatory to name the file as main but it's convention.  

## Datatypes

string, number(integer and float), boolean, array, slices & maps.  

Datatypes are needed for memory allocation. 
 
int - signed integer -> Contains both negative and positive numbers -> int8, int16, int32, etc...  
uint - unsigned integer -> Contains only positive numbers.  
float - Uses 16 bytes of memory.  
boolean - 1byte  

static/strongly vs dynamic/loosely/weakly typed languages:  
Compiler enforces the types. go compiler can implicitly assume the data type.  

## Variables

`var <variable-name> <data-type> = <value>`  

`var myinteger int = 324`  
Or  
`myinteger := 324`  
Or  
```
var (
	myfirstint = 1
	mysecondint = 2
)
```

## Variable Scope

Inner & Outer blocks:  
```
{
	// Outer block
	{
		// Inner block
	}
}
```
The inner block can access variables declared outside of its block. But the outer block can't access the variable declared inside the inner block.  

Local vs Global variables:  
Similar to the inner and outer blocks.  

`%T` or `reflect.TypeOf` - To find variable type.  

## Zero Values

A variable declared and not assigned values which will be given default value. This is called zero values.  

bool -> false  
int -> 0  
float -> 0.0  
string -> ""  
pointers, functions, interfaces, maps -> nil  

## User Input

`fmt.Scanf("%<format specifier>, arg)`
`fmt.Scanf("%s", &name)`  

`Scanf` returns two values -> count and err  
`count, err := fmt.Scanf("%s", &name)`

## Constants

`const <const-name> <data-type> = <value>`
E.g.,  
`const myConstant int = 100`

Typed constant:   
`const age int = 29`  
Untyped constant:  
`const age = 29`  

`const` value needs to be assigned when declared and also shorthand won't work with constant.  

## Printing

fmt methods -> print, println & printf(format specifier) and other.  

## Type Casting

int to float:  
```
var i int = 90
var f float64 = float64(i)
```

Converting float to int would lose precise value.

`strconv` package:

`Itoa()` -> int to string  
```
var i int = 42
var s string = strconv.Iota(i)
```
Atoi() -> string to int  

## Operators & Operands

### Comparision
== | != | < | <= | > | >=  

### Arithmetic
+ | - | * | / | % | ++ | --  

### Assignment
= | += | -+ | *= | /= | %=  

### Bitwise
& | | | ^(XOR) | >> | <<  

#### Bitwise &
12 = 00001100(Decimal to binary - Keep dividing the number by 2 until 0 remainder makes up the binary format)

25 = 00011001

0 0 0 0 1 1 0 0  &
0 0 0 1 1 0 0 1
__________________
0 0 0 0 1 0 0 0 = 8

#### Left shift(<<)

212 = 11010100  

E.g., `212 << 1` - Shift the bit by 1 position which means it adds zero at the vacated position.  

11010100(0) = 424  

#### Right shift(>>)

It's the same concept as the left shift. but it shifts it to the right.  

### Logical

&& | || | !  

## Control flow

- If-Else Statement

- Switch Statement

	- `fallthrough` keyword is to force it to execute successive case blocks.  
	- switch has an implicit break so if the first case is satisfied, it'll exit.  

- For loop

	- `break` statement ends the loop immediately when it is encountered.  
	- `continue` statement skips the current iteration of the loop and continues with the next iteration.  

## Arrays

Collection of similar data types stored at Contiguous (next in sequence) memory locations.  

Syntax:
`var <array-name> <size> <data-type>`
`var array [5]int`

Array initialization:  
`var array [5]int = [5]int{1, 2, 3, 4, 5}`
Or  
`array := [5]int{1, 2, 3, 4, 5}`

Using ellipses:  
With ellipses, the array doesn't need a size specified. It will be calculated based on the value supplied.  
`array := [...]int{1,2,3}`

Multidimensional arrays:  
`array := [3][2]int{{3,4}, {4,5}, {5,6}}`

## Slices

Slice is more flexible than array and it's not a fixed size.  

Components:
Pointer, Length, and Capacity.  

Syntax:  
`<slice-name> := []<data-type>{<values>}`
`slice := []int{1, 2, 3}`

To slice from an array:  
`slice := <array>[<start-index>:<end-index>]`

A slice is a reference to the underlying array. so if the underlying array changes slice that referencing it changes too.  

Another way to declare slice:  
`slice := make([]<data-type>, length, capacity)`

Append to a slice:  
`slice = append(slice, value1, value2)`
 
Append slice to slice:  
`slice = append(slice, anotherslice...)`

Copy from slice to slice using the built-in copy method:  
`num := copy(dest_slice, src_slice)`

## Maps

Syntax:  
`var <map-name> map[<key-data-type>]<value-data-type>`
`var my-map map[string]int`

The zero value of the map is nil.  

Initialize and assign map:  
`<map-name> := map[<key-data-type>]<value-data-type>{<key-value-pairs>}`
`mymap := map[string]int{"key1": 1, "key2": 2}`

Using the make function:  
`<map-name> := make(map[<key-data-type>]<value-data-type>, <initial-capacity>)` 
`mymap := make(map[string]int)`

## Function
```
func myfunction(a int, b int) int {
	// Content of the function
}
```

Parameters vs Arguments:  
Parameters are listed in the function definition and arguments are the real values passed to when calling the function.  

Two types of parameters are input and return parameters.  

Multiple return values:  
```
func myfunction(a int, b int) (int, int) {
	// Content of the function
	sum = a + b
	diff = a-b
	return sum, diff
}
```
Named return values:  
```
func myfunction(a int, b int) mynumber int {
	// Content of the function
	mynumber = a + b
	return
}
```

Variadic function:  
The function that accepts a variable number of arguments.  

`func addNumbers(numbers ...int) int {}`

Numbers argument would be passed in as a slice. the variadic argument is always placed at the end.  

Blank identifier(_):  
Used as an anonymous placeholder.  
```
for _, value := range mymap {
	fmt.println(value)
}
```

Recursive functions:  
The function calls itself until it reaches the desired value.  

Anonymous functions:  

`anonyfunc := func (a int, b int) int {}`
`anonyfunc(2, 3)`
Or  
`anonyfunc := func (a int, b int) int {}(2, 3)`

Higher order functions:  
Receive function as an argument or return another function as output.  

Defer statement:  
Delay the execution of a function until the surrounding function returns.  

## Pointers

When a program runs, the variable is allocated memory. The pointer holds the memory address of another variable.  

| memory address | memory |
| --- | --- | 
| 0x0ce301 | 1 |
| 0x0302 | 0x0ce301 | 

Address and dereference operators:  
`&` - Address of operator
`*` - Dereference operator returns the value stored in that address

`x int := 1`
`y *int := &x`
`*(0x0ce301) or *(&x) = 1`

Declaring and initializing pointer:  
`var <pointer-name> *<data-type>`
`var pointer *int`

The pointers type is declared using `*`. Above mentioned pointer is an integer-type pointer.  

The zero value of the pointer is nil.  
`var <pointer-name> *<data-type> = &<variable-name>`
Or  
`var <pointer-name> = &<variable-name>`
Or  
`<pointer-name> := &<variable-name>`

`i := 12`
`var pointer *int = &i`
Or  
`var pointer = &i`
Or  
`pointer := &i`

Dereferencing a pointer:  
`*<pointer-name>`
`*<pointer-name> = 200`

If dereferenced pointer value changes, the reference also gets changed.  

Passing by value in function:  
These data types(int, float, bool, string, array, & struct) are passed by value by default. it won't change the original variable value but it assigns a new value in memory.  

Passing by reference in function:  
Address of the parameter passed to the function. When the value is set to a dereferenced pointer, it would change the original value.  

slices, maps and channel are passed by reference by default.  

## Struct

Struct is a user-defined data type.  

Syntax:
```
type <struct-name> struct {
	<field1> string
	<field2> int
	<field3> []int
	<field4> map[string]int
}
```

To initialize struct:  
`<variable-name> := new(<struct-name>)`

Initialised struct variable would point to the struct pointer.  

Another way to initialize a struct with values:  
```
<variable-name> := <struct-name>{
	<field>: <value>
}
```

To access the struct fields:  
`<variable-name>.<field>`

Pass struct to functions:  
It's passed the value by default which means changing the struct would not affect the original struct. If you want to change the original struct value, pass it by reference.  

## Methods

The method is a function that has a receiver.  

`func <receiver> <method-name>(<parameters>) <return-params> {}`

Function + Struct = Method  

An instance of the struct would have access to the method. The receiver can be either instance of a struct or a pointer to a struct.  

`func (o *Order) makeOrder() {}`
Or   
`func (o Order) makeOrder() {}`

Method sets:  
To encapsulate functionalities.  

## Interfaces

It specifies a method set and is a way to introduce modularity. It's a blueprint for a method set.  
```
type <interface-name> interface{
	// Method signatures
}
```

To implement an interface:  
No explicit declaration is needed for implementing the interface. By implementing all the methods set defined in the interface definition, type becomes an interface.  
