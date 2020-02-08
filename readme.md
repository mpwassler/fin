Just a test project to try out rust. Will add more to it...

Finance tools CLI

Build with `cargo build`
Run with `cargo run -- [options]`
Test with `cargo test`

Can calculate the total ammount and 
monthly payments of a loan based on the term

```BASH
./target/debug/fin interest \
	-p 2500 \
	-i 7 \
	-m 60
```