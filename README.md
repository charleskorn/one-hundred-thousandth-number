# one-hundred-thousandth-number

The August Shokunin challenge:

> There are a large number of 9 digit integers in the range 123456789 to 987654321 where each digit only appears once.
>
> Your mission should you choose to accept is to identify the 100,000th number in this sequence.
>
> The first number is the easiest to find - 123456789, the second is 123456798, the third is 123456879 and so on.
> No digit may repeat so 122345675 is not a valid number in this sequence.

## Prerequisites

* Docker (I have tested it against Docker 1.12.1, other versions may work)

Run `./go.sh setup` to pull down the Docker image used for development tasks.

## Running

Run `./go.sh run <number>` to compile and run the application, where `<number>` is the number in the sequence you'd like to find.

For example, to find the third number in the sequence, run `./go.sh run 3`.

If `<number>` is omitted, it defaults to 100000.

## Testing

Run `./go.sh test` to run the tests.

## How does it work?

The code here does *not* enumerate all the items in the sequence, as that would take a long time.
Instead, it does it in O(1) by taking advantage of some maths:

The first thing to note is that there are nine choices for the first digit, then eight remaining
choices for the second digit, seven for the third digit and so on, for a total of 9! (362,880)
possible combinations of digits - each of which is a valid number in the sequence.

This means that there are 8! numbers in the sequence that start with any given digit, and that there
are 7! numbers in the sequence that start with any given two-digit combination and so on. Another
way to think about it is by using a decision tree and visualising the number of nodes under each
branch.

We can take advantage of this to go from a sequence index (n) to the number in the sequence.
(For example, for n = 1, the number is 123456789, and for n = 2, the number is 123456798.)

* Any number with n between 1 and 8! will start with a 1

* Any number with n between 8! + 1 and 2×8! will start with a 2

* Any number with n between 2×8! + 1 and 3×8! will start with a 3

  ...and so on

As we saw before, there are now 8 remaining digits to choose from for the second digit, which
together mean that we have the possibility of forming 8! different numbers with the first digit
we just chose. So, to pick the second digit, we consider n % 8! (n modulo the number of
possibilities left to choose from) in a similar way to the first digit.

* Any number with n % 8! between 1 and 7! will have a second digit equal to the first remaining
  digit (eg. if the first digit was 1, then this would be 2, since the remaining digits are
  2,3,4,5,6,7,8,9)

* Any number with n % 8! between 7! + 1 and 2×7! will have a second digit equal to the second
  remaining digit (eg. if the first digit was 1, then this would be 3)

* Any number with n % 8! between 2×7! + 1 and 3×7! will have a second digit equal to the third
  remaining digit (eg. if the first digit was 1, then this would be 4)

    ...and so on

Then we repeat a similar process for the remaining seven digits (of course, the last digit will just
be whatever the last unused digit is, but the maths still works out OK).

### Example: 100,000th number

Let's say we're working out what the 100,000th number is.

The first digit is 3, because n is between 2×8! + 1 (= 80641) and 3×8! (= 120960).

The second digit is 5, because n % 8! (19360) is between 3×7! + 1 (= 15121) and 4×7! (= 20160),
which means we pick the fourth remaining digit (1,2,4,5,6,7,8,9).

...and so on, until we get the answer of 358926471.
