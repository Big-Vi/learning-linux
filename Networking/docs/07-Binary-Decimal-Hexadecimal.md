## Binary

We count in Base10(0-9). The binary is either 0 or 1. The machine only understands binary. When counting in binary, it's only a power of 2. 

## Convert binary to decimal

| 128s place | 64s place | 32s place | 16s place | 8s place | 4s place | 2s place | 1s place
| --- | --- | --- | --- | --- | --- | --- | --- |
1 | 1 | 0 | 0 | 1 | 0 | 0 | 0
1x2<sup>7</sup> | 1x2<sup>6</sup> | 0x2<sup>5</sup> | 0x2<sup>4</sup> | 1x2<sup>3</sup> | 0x2<sup>2</sup> | 0x2<sup>1</sup> | 0x2<sup>0</sup>

Value = 200  

## Convert decimal to binary

200%2 = 100 (Remainder 0)
100%2 = 50 (Remainder 0)
50%2 = 25 (Remainder 0)
25%2 = 12 (Remainder 1)
12%2 = 6 (Remainder 0)
6%2 = 3 (Remainder 0)
3%2 = 1 (Remainder 1)
1%2 = 1 (Remainder 1)

Value = 11001000

## Hexadecimal
0-15

| Binary | Decimal | Hexadecimal
| ---- | --- | --- |
| 0000 | 0 | 0 |
| 0001 | 1 | 1 |
| ... | ... | ... |
| 1111 | 15 | F
10000 | 16 | 10(Not a decimal 10)
