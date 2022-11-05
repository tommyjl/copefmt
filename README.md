# copefmt

`copefmt` is the copium code formatter for languages that don't have any better options.

## How to use

`copefmt` read from stdin and writes to stdout by default.

```
echo '*[]{...,foo->,bar->{...,baz->}}' | copefmt
*[]{
    ...,
    foo->,
    bar->{
        ...,
        baz->
    }
}
```

But filenames can be provided as arguments.

```
copefmt --input data/simple.groq --output formatted_simple.groq
```


## Installation

Install from source: `cargo install --path .`
