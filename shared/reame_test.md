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


# CHECK TRAITS
# find definition
grep -r "pub trait TRAITNAME\|pub struct TYPENAME"

# find implementations
grep -r "impl TRAITNAME\|impl.*for TYPENAME"

# find methods
grep "pub fn\|pub async fn" path/to/file.rs

# find where something is used
grep -r "TYPENAME\|TRAITNAME" path/ | grep "fn\|impl\|use" | head -20

# when compiler says "method not found" — find where it IS defined
grep -r "fn METHOD_NAME" ~/.cargo/git/checkouts/CRATE/src/ | head -10
