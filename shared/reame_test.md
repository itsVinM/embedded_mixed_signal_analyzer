Test run with the followinf cmd (independent from stm32)
```bash
cargo test --target aarch64-apple-darwin
```

# OUTPUT
```bash
running 3 tests
test tests::fail_adc_returns_correct_string ... ok
test tests::fail_clock_returns_correct_string ... ok
test tests::ready_status_returns_correct_string ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```