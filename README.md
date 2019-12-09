# BF8
Brainfuck of octal number.  

## Usage
`cargo run example/hello.bf`
> `Hello World!`

## Commands
| Octal number | Original | Meaning |
| :-: | :-: | --- |
| 000 |  >  | increment the data pointer (to point to the next cell to the right). |
| 001 |  <  | decrement the data pointer (to point to the next cell to the left). |
| 010 |  +  | increment (increase by one) the byte at the data pointer. |
| 011 |  -  | decrement (decrease by one) the byte at the data pointer. |
| 100 |  .  | output the byte at the data pointer. |
| 101 |  ,  | accept one byte of input, storing its value in the byte at the data pointer. |
| 110 |  [  | if the byte at the data pointer is zero, then instead of moving the instruction pointer forward to the next command, jump it forward to the command after the matching ] command. |
| 111 |  ]  | if the byte at the data pointer is nonzero, then instead of moving the instruction pointer forward to the next command, jump it back to the command after the matching [ command. |

## References
1. [https://en.wikipedia.org/wiki/Brainfuck](https://en.wikipedia.org/wiki/Brainfuck)  
2. [http://hakugetu.so.land.to/program/brainfuck/1-4.php > FizzBuzz](http://hakugetu.so.land.to/program/brainfuck/1-4.php)  
