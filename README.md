# one-hundred-thousandth-number

The August Shokunin challenge:

> There are a large number of 9 digit integers in the range 123456789 to 987654321 where each digit only appears once.
>
> Your mission should you choose to accept is to identify the 100,000th number in this sequence.
>
> The first number is the easiest to find  - 123456789, the second is 123456798, the third is 123456879 and so on.
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
