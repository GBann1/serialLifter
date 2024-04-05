# Things to know about scales

There are multiple different types of stream formats:

### List of formats
RLWS: Rice Lakes Weigh Systems<br>
CRDNAL: Cardinal stream format<br>
WTRNIX: Avery Weigh-Tronix stream format<br>
TOLEDO: Mettler Toledo stream format<br>
CUSTOM: Hopefully I'll never have to deal with this.

#### Formats
- RLWS    <STX\><POL\><wwwwwww\><UNIT\><G/N\><S\><TERM\>
- CRDNAL  <CR\><POL\><wwwwww\><S\><SP\><UNIT\><SP\><G/N\><SP\><SP\><ETX\>
- WTRNIX  <SP\><G/N\><POL\><wwwwww\><SP\><UNIT\><TERM\>
- TOLEDO  <STX\><SWA\><SWB\><SWC\><wwwwww\><tttttt\><CR\>

### **KEY / VALUE:**
#### RLWS
- STX = The ASCII value 2
- POL = Polarity
    - ' ', '', '+' = Positive (<space\>,<None\>,<+\>)
    - '-' = Negative (<-\>)
- wwwwwww = 7 digits that are right justified, dummy zeroes, decimal point with no leading zeroes except for leading zero immediately preceding teh decimal point. Leading zeroes transmitted as spaces ('   0.0', '80000.0')
    - ^^^^^^^ = Overload
    - ]]]]]]] = Underrange
    - OVERFL = Overflow
- UNIT = Units
    - L = Pounds
    - K = Kilograms
    - T = Tons
    - G = Grams
    - O = Ounces
    - ' ' = None (space)
- G/N = Gross / Net
    - G = Gross
    - N = Net
- S = Status
    - ' ' = Valid (space)
    - I = Invalid
    - M = Motion
    - O = Over/Under range
- TERM = Termination
    - <CR\><LF\> = Carriage Return ('\r', 0x0D, ASCII code 13) and Line Feed('\n', 0x0A, ASCII code 10)
    - <CR> = '\r' aka 0x0D aka ASCII code 13
    
#### CRDNRL
(all above where applicable)
- wwwwww = 6 digits right justified, floating decimal, leading zeros **are present**
- SP = Space char (0x20, ASCII code 32)
- UNIT = Units
    - lb = Pound
    - kg = Kilogram
    - g = Gram
    - tn = Ton (short)
    - t = Ton (metric)
    - oz = Ounce
    - ' ' = None (<space\>)
- G/N = Gross / Net
    - g = Gross
    - n = Net
- ETX = Hex value of 3 (0x03 -> 03, <03>)

#### WTRNIX
(all included in above)
#### TOLEDO
Not doing this. Requires too much seperate logic that will make it difficult to turn into an efficient function