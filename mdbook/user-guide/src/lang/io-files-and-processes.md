# IO, Files, and Processes

Coming soon...

In the meantime, here are some relevant commands:

```
appendf	( <contents> <filename> -- ) Write a string to a file. If a file previously existed, the new content will be appended.
args	( -- [<arg>] ) Produce the arguments provided to the process when it was launched.
cd	( <dirname> -- ) Change the process's working directory.
cwd	( -- <dirname> ) Produce the current working directory.
enl	( -- ) Print a newline to standard error.
ep	( <a> -- ) Print the most recent value to standard error.
epl	( <a> -- ) Print the most recent value and a newline to standard error.
epls	( [<a>] -- ) Print the values of the most recent quote, each followed by a newline, to standard error.
eprint	( <a> -- ) Print the most recent value to standard error.
eprintln	( <a> -- ) Print the most recent value and a newline to standard error.
eprintlns	( [<a>] -- ) Print the values of the most recent quote, each followed by a newline, to standard error.
exec	( <process> -- ) Execute a child process (from a String). When successful, returns stdout as a string. When unsuccessful, prints the child's stderr to stderr, and returns boolean false.
exit	( <exitcode> -- ) Exit with the specified exit code.
interactive?	( -- <bool> ) Determine if the input mode is interactive (a TTY) or not.
ls	( -- [<filename>] ) Produce a quote of files and directories in the process's working directory.
nl	( -- ) Print a newline to standard output.
norm	( -- ) Print a control character to reset any styling to standard output and standard error.
p	( <a> -- ) Print the most recent value to standard output.
pl	( <a> -- ) Print the most recent value and a newline to standard output.
pls	( [<a>] -- ) Print the values of the most recent quote, each followed by a newline, to standard output.
print	( <a> -- ) Print the most recent value to standard output.
println	( <a> -- ) Print the most recent value and a newline to standard output.
printlns	( [<a>] -- ) Print the values of the most recent quote, each followed by a newline, to standard output.
procname	( -- <name> ) Produce the name of the current process. This can be used, for example, to get the name of a shebang script.
readf	( <filename> -- <contents> ) Read a file's contents as a string.
readln	( -- <line> ) Read a string from standard input until newline.
readlns	( -- [<line>] ) Read strings, separated by newlines, from standard input until EOF. (For 
rl	( -- <line> ) Read a string from standard input until newline.
rls	( -- [<line>] ) Read strings, separated by newlines, from standard input until EOF. (For example: until ctrl+d in a Unix-like system, or until a pipe is closed.)
writef	( <contents> <filename> -- ) Write a string as a file. If a file previously existed, it will be overwritten.
```
