# Hello, World!
.data ## Data declaration section
	## String to be printed:
	out_string: .asciiz "\nHello, World!\n"

.text ## Assembly language instructions go in text segment
	main: ## Start of code section
		li $v0, 4 # system call to print string (4)
		la $a0, out_string # load address of string to be printed into $a0
		syscall # call OS to print the string

		li $v0, 10 # terminate program
		syscall
