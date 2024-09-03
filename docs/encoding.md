# Encoding

In this file, you will find the details about our encoding system for the grid.

This encoding is used to transfer the game state from the backend to the frontend. The frontend has no logic, it simply displays the data received from the backend and returns which move has been played.

## Requirements
The encoding system must:
- Allow for a grid of infinite depth.
- Use as few bits as possible (because why not?).

## Explanation

### Grid
A grid is composed of 9 cells.

### Cell
A cell can be in 5 differents states:
- Empty
- Cross
- Circle
- Both (Cross and Circle)
- Sub-grid

To represent these 5 states, we use 3 bits of memory. We have chosen the following values:
- `0b000` => Blank
- `0b001` => Circle
- `0b010` => Cross
- `0b011` => Both
- `0b100` => Sub-grid

### Simple example

Consider this grid:
```
 |X|O
-----
 |O|B 
-----
O|X| 
```

This grid is encoded as follow:
```
0b000 0b010 0b001
0b000 0b001 0b011   => 0b000010001000001011001010000
0b001 0b010 0b000
```

### Handling sub-grids
In the example above, we saw all the 4 simple states of a cell.

Now let's take a closer look at the sub-grid. We saw earlier how we mark a cell as a sub-grid, but we have to describe the content of this sub-grid.

We concatenate the sub-grid's cells in a breadth-first manner. This approach allows for truncating the byte string if it becomes too long, a common occurrence in deep games.

#### Complete example

Let's see how we can encode this grid (see image below).
![complex grid](https://github.com/vdusart/ultimate-ttt/assets/43795504/3fb689b9-4cac-4b94-ae87-a1c55dc7d51f)


```
#########################
######## Level 1 ########
#########################

O|S|S     0b001 0b100 0b100
-----
O|S|X  => 0b001 0b100 0b010  => 0b001100100001100010010100001
-----
X|S|O     0b010 0b100 0b001

#########################
######## Level 2 ########
#########################

 | |      0b000 0b000 0b000
-----
 | |   => 0b000 0b000 0b000  => 0b000000000000000000000000100
-----
 | |S     0b000 0b000 0b100


 |X|      0b000 0b010 0b000
-----
 | |O  => 0b000 0b000 0b001  => 0b000010000000000001000010000
-----
 |X|      0b000 0b010 0b000


 | |      0b000 0b000 0b000
-----
O| |   => 0b001 0b000 0b000  => 0b000000000001000000000000000
-----
 | |      0b000 0b000 0b000


 | |O     0b000 0b000 0b001
-----
 | |   => 0b000 0b000 0b000  => 0b000000001000000000000000000
-----
 | |      0b000 0b000 0b000

#########################
######## Level 3 ########
#########################

O|X|      0b001 0b010 0b000
-----
 |O|O  => 0b000 0b001 0b001  => 0b001010000000001001000010000
-----
 |X|      0b000 0b010 0b000
```

After concatenating all the values we end up with the encoded grid.

```
Data:
0b001100100001100010010100001000000000000000000000000100000010000000000001000010000000000000001000000000000000000000001000000000000000000001010000000001001000010000

Length: 6 * 27 = 162 bits
```

### Remarks
- The data size decreases as the game progress.
- Minimum size: 27 bits
- Maximum size (for depth n): $(9^n + 1) * 27$ bits


# Decoding

This section covers the decoding algorithm.

### Axioms:
- The data is a byte string
- Length % 27 == 0

### Decoding Process

1. Loop through the byte string in 27-bit chunks, representing each grid, but in reverse order (right to left).
2. Within each chunk, read 3 bits at a time, representing each cell, again in reverse order (from the bottom-right cell to the top-left).
3. For each 3-bit sequence, decode the cell.
4. Once the current grid is fully loaded, push it to the end of a queue. Then move on to decode the next subgrid.
5. When encounter a `0b100` value (indicating a subgrid), pop the first element from the queue, which gives us the corresponding subgrid value.

This approach ensures that by the time we need to place a subgrid, it has already been decoded, allowing us to insert it directly without deferring the decoding process.

You can find an animated explanation below:

https://github.com/user-attachments/assets/c70df260-3db9-48df-8b6c-749ebceb7278
