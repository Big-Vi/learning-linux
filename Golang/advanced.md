## Concurrency

Sequential/Parallel/Concurrent processing
 
### Go-routines
Lightweight thread managed by Go runtime scheduler.

Syntax:
`go limit()`

main function that exectures go routine won't wait for that go routine to be completed. We can use wait groups to overcome this use case.

### Anonymous go-routine

`go func() {}(args...)`

## Go runtime scheduler

## Go-routines vs Threads

## WaitGroups

`var <var-name> sync.WaitGroup`
Three methods:
wg.Add(int)
This method adds counter of (int)
wg.Wait()
Block main function until all the go routing finished.
wg.Done()
Done method informs the counter that go routine is completed.

## Channels

via channels go-routines communicate.

Syntax:
`var <channel-name> chan string`
`c := make(chan string)`

Sending a value to channel:
`ch<-value`
if value is string, ch needs to be string data type.

Receiving a value from channel:
`value := <-ch`

Closing a channel:
`close(ch)`

Querying buffer of a channel:
`cap(ch)`

Querying length of a channel:
`len(ch)`

Unbufferd Channel
channel does not have any buffer to store any value. so the channel is going to block the execution of its go routine until there is another go routine to receive this value.

sent on unbuffered channel is blocked until receive happens on that channel in some other go routine and vice versa.

## Buffered Channels
It has capacity to hold data. sending to channel blocks the go routine only if the buffer is full. receiving from channel blocks only when the channel is empty.

Syntax:
`c := make(chan <data-type>, capacity)`
`c := make(chan int, 10)`

## Closing channel

no more data  can be sent to that channel.
Syntax:
value, ok := <- ch
if ok variable is true, the channel is opened.

## Panic situations

panic is like an exception, which arises at runtime.

## Channel - for-range

```
for value := range <channel-name> {
    fmt.Println(value)
}
```

## Select statement

Similar to switch statement but for channels. it lets a go routine wait on multiple communication operations.

Syntax:
```
select{
    case <channel-send-or-receive>:
    // Code
    case <channel-send-or-receive>:
    // Code
    default:
    // Optional
}
```
ouput of select statement is non deterministic since it executes whichever go routine executes first.

default statement allow the select statement not to be blocked.

Best practices:
-   Closing go routine
make sure to exit the go routine once it finished. otherwise it leads to go routine leak. because it would block I/O.

- Spawning go routine closure in a loop
```
func main() {
    var wg = sync.WaitGroup
    wg.Add(10)
    for i := 1; i <=10; i++ {
        go func(i int) {
            fmt.Println(i)
            wg.Done()
        }(i)
    }
    wg.Wait()
}
```
if you don't pass the i into anonymous go routine, it would print the value of i at the time of when go routine starts. main go routine may wait for the cpu to be available. i is defined at the point when calling the go routine.

- Buffered and Unbuffered channel
need to know when to use buffered or unbuffered channel.

- Time out code
func After(d Duration) <- chan Time
```
select {
    case value := <ch:
        fmt.Println(value)
    case <-time.After(1 * time.Second):
        fmt.Println("this will execute after time out period")
}
```
this is useful to unblock select statement aftet the specified time period.


## Modules & Packages

Modules is one project which contain collection of packages.

`go mod init <module-path>`
<module-path> needs to be global unique identifier.

go.mod contains all the packages.
`go mod tidy`
This will add the any missing or required packages or dependencies into go.mod file.

if go.mod file present in the root directory, it becomes module.

function(identifier) name needs to be in capital letter when imported from different package. if the name starts with _ or lower case, it can be only accesed inside the package.
best practice:
make sure you really want to expose the identifier with capital letter since it's get exposed.
document all the exported identifiers and make them backward compatible on new release.

to import local module which is not pushed to git yet, we can use replace directive.
`go mod edit -replace <module-path-to-be-replaced> <local-module-path-to-be-replaced-with>`

other useful commands:
go run main.go
go build - compile and create executable in current dir.
go install - compile and move the executable to  $GOPATH/bin. so it can be run from any where from terminal. 
add the GOPATH to PATH to run the compiled package.
go env GOPATH
go get - downloads source code into module cache and updates go.mod. can be used to upgrade and downgrade packages.

godoc tool parses go code and produce html or plain output.
go doc <package-name>
go doc <package-name.identifier-name>


## Core packages

### Strings
- Contains
- ReplaceAll
- Count

Strings are immutable. 

### Input/Output
IO interfaces:
- The Reader interface
    Reader has only one method(Read). 
- The Writer interface
    Writer has only one method(Write).

```
type Reader interface {
    Read(p []byte)(n int, err error)
}
type Writer interface {
    Write(p []byte)(n int, err error)
}
```
To view how the function implemented:
`go doc <package> <method>`
`go doc io Copy`

### File handling

Libraries:
OS - this package provides api interface for file handling which is unifrom across all os.
IO
filepath - to parse and construct file path based on os.
fmt - format I/O with functions to read and write to standard input and output.

filepath
go doc filepath - to view all methods or identifiers
go doc filepath Join/Dir/Base/IsAbs/Ext

OS
go doc os Stat/ReadFile/Open/OpenFile/WriteString

read data in chunks, use os.Open

### Error handling
errors package
New()
Errorf() - error formatter

go doc errors New
go doc fmt Errorf

### Logging
log package


logging framework 
makes it easy to standardize log data and easier to read and understand.
- glog
- logrus

logrus is popular and Docker using it. standard log library doesn't have levels.
logrus supports (7)log levels. so you can set level using SetLevel method.

### Sort

### Hashes and cryptography
cryptography is secure communication.  crypto package used for this.

## Testing







## Timers
## Tickers
## Context
## Mutexes
## Worker pool
## Rate limit
## Atomic
## Stateful goroutines
## Generics


what is workspace and how to create it?
why go file start with package?
