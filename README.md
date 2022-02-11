# Small Fibonacci Sequence Library

This library is tiny and has a single purpose only.

> A library with no dependencies that can compute and display big fibonacci numbers.

Please take big with a grain of salt. Currently, it's taking a few seconds to display
stuff in the 1-2 million range. Still, if you don't need to go that crazy, you'll
get your answers quick.

# Features

A very cool struct called "Sequence" that will produce the fibonacci sequence as an
iterator!

```rs
for (n, value) in fib_sequence::Sequence::new().enumerate().take(100) {
    println!("f({}) = {}", n, value);
}
```

As you see, it's by default an infinite iterator (meaning it'll stop as soon as your
computer starts frying). So use something like the take method to make it finite.

Oh, and if for some reason, you only want to know the 69420th fibonacci number, you
don't have to go through all those terms in the sequence for that. I got a much faster
option:

```rs
println!("{}", fib_sequence::nth(69420));
```

The nth function follows the mathematical convention for the sequence. That is,

f(0) = 0; f(1) = 1; f(2) = 1; ...

I really advise you to avoid going to the millions and beyond. This function is currently
quadratic (and it might stay that way since I can't be bothered to implement subtraction,
which is needed if I wanted to make a fancy karatsuba multiplication to go faster). So
going from 1 to 2 million was a jump from 2.36 to ~10 seconds on my machine!

## Ughh, why is it returning strings??

Because I said so!

Don't get me wrong, I love numbers and number-involving structs. However, I only intend for
this library to be used so people can manipulate fibonacci numbers in base 10. So I figured,
why would I rely on this numerical abstraction when I can just use strings, which is something
everyone knows how to use, and can easily work with? You can divide it into a vector of chars,
count how many 6's appear, count how many digits it has with the len() function, etc.

## Will you add [insert feature here]?

Probably not. This library does everything I need it to do.

## Okay, but can I still suggest?

Yes! If it's simple enough, I might add it in future versions. Just gimme your suggestion in
the github repo.