# Day 17 - Custom Instruction Set

## Part 1

Write a simulation logic and run it. We also add some extra careful steps to give it a max instructions to run before calling it quits (to ensure it does not run forever). I went with the number 200, can be increased, but works for me.

## Part 2

We need to find a register value that will produce the copy of the program through its runtime output.
Now, this problem I found extremely interesting. Simple solution: run with different initial values of register A starting with 0 and keep on incrementing until we find the first one such value where the copy of the program is produced. Minor problem with that is that the first such number that works is 15-digit number.

### Analysis - Getting there

Note there is one instruction that changes the value of A register and that the only jump instruction we have looks whether register A is non-zero in order to perform the jump. Further, the only instruction that changes register A is ADV (opcode 0) and it only has a chance to decrease it or keep it the same (in case of division by 1). So each time we run our short program the only chance of not finishing immediately depends on the initial value of register A, and the larger the initial value the larger the number of cycles we will perform.

So, alternative solution is to roughly estimate what is the smallest value of A that we need in order to have our program output exactly the same amount of times as there are digits in our program. Thus, we will work out the lower bound, it being say something quite substantially big, such as say `2^45`.

We can work out this number by also seeing what is the ADV instruction like - in my case it is ADV 3, meaning, each cycle register A is slashed by division with 8. To output exactly one number out, register A would need to be `[0,8)`. If we want exactly two digits, then we need A to be divided by 8 exactly 2 times before it becomes zero, thus giving us a range `[8,64)`. To output 3 number, same logic, `[64,512)`. Repetition. And we work out that for - say 16 numbers - we need `[8^15,8^16)`.

### Further Analysis and Solution

Okay, that's not bad, but not good enough. We managed to skip a lot of numbers, but our search space is still extremely large. Looking at the input I observed interesting behaviours, and a new idea came to mind. Work out for each cycle what the register A needs to be/ can be, starting from the last cycle.

So we know what value A needs to be on the last cycle: concretely `[1,8)` (it cannot be 0, because then it would have terminated the previous cycle perhaps). So we can run the simulator to work out what number it needs to be to get the last digit of our program. Say that we got that A needs to be, say for the benefit of the multiple possibilities, it needs to be either 3 or 7. Then we know that in the previous round it was either from `[3*8,3*8+8)` or `[7*8,7*8+8)`. And basically using this approach we work out A quite quickly.

Of course, there are certain assumptions that we would need to mention. I will explain those in the next section constraints.

### Constraints

The constraints that are guaranteed to produce a solution (they might not be minimal requirements, probably I added some extra) are the following:

- Only one ADV instruction and its combo operand is literal operand that is not zero.
    - Literal operand guarantees constant rate of reduction of register A e.g. each cycle divided by 8
    - Literal operand 0 would do nothing (register A stays the same - effectively NOP)
    - Only one such instruction makes it easier to work out A's boundaries in previous cycle
- Only one JNZ instruction that happens last in the program and if jump takes place, jumps back to 0.
    - Jumping back to 0 makes it easier to have exact same run cycle-to-cycle.
    - It being at the end ensures that on the last cycle nothing else happens (we can add extra logic to handle any additional instructions after JNZ does not jump, but keeping it simpler this way)
    - Only one such instruction: again, similar to the above argument. Keeps things more easier to solve.
- Registers B and C are derived from register A before they are used in each cycle.
    - Say that register B transforms itself on its own. At that point, from the start of the program, register A might reduce / change making it more difficult to work out how changing the initial value of register A will affect the program's output.

With these constraints, we can look each cycle in isolation and tweak only the relevant bits of register A, and once happy with the relevant bits, we move onto the previous cycle and shift the register A's bits by appropriate amount dictated by ADV instruction.

## Time

Part 1: 105 ms

Part 2: 121 ms