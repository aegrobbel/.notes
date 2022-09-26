# Cargo and Meta

Update Rust:
```bash
rustup update
```

To auto format:
```bash
rustfmt
```

If a lock file exists, a crate won't be updated if there is a new version available. To update, we would run `cargo update`

# Basics

## Result

- `Result` is just an enumeration, but it's used to return multiple variants, but in general the purpose is to encode either the correct value or error information
- Possible values are `Ok` and `Err`
  - `Ok` means the operation succeeded, and wraps the resulting value
  - `Err` means the operation failed, and wraps the error information like response code/failure message
- Because we can define methods on enums, `Result` has some methods we can use, like `expect`
  - If the Result is an error, then expect will cause the program to crash and display the message passed to the expect method. Otherwise, it will return the wrapped value.
 
__Getting a value__

- "unwrap" to ignore and potentially panic
- "expect" to panic with error message (usually preferred)
- "unwrap_or" to define a fallback if possible
- "uwrap_or_else" to define fallback w/ closure
- "match" to handle each case
- To panic with ok values, unwrap_err and expect_err will panic with ok values instead of err
- unwrap_or will allow a default to be set

## Iteration

- To iterate over a list, we can use `map`
Map to convert to another type, retaining Result
Map_err to convert err to certain type, retaining result

To combine
And returns error if either or is Err, and returns the passed result value
Or returns Ok value if either is Ok

Option
You can use ? to access the value of an Option, returning None if it fails: 
Ok_or_else to convert an option to a Result
Match to pull out whatever is inside
If you’re ever matching against an option and returning a Some or None, consider mapping the option instead: some_option.map(Other::from)
Traits
Implementing `From` gives both `From` and `Into` implementation. Implementing `Into` only gives `Into`.
You can deref a borrowed value if it implements the copy trait instead of using to_owned() or clone() 
Serde
You can use #[serde_as] to deserialize something that doesn’t implement the Serialize or Deserialize trait


Channels
Using channels to kill a thread when it receives a message:
use std::io::{self, BufRead};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;
 
fn main() {
    println!("Press enter to terminate the child thread");
    let (tx, rx) = mpsc::channel();
 
    thread::spawn(move || loop {
        println!("Working...");
        thread::sleep(Duration::from_millis(500));
        match rx.try_recv() {
            Ok(_) | Err(TryRecvError::Disconnected) => {
                println!("Terminating.");
                break;
            }
            Err(TryRecvError::Empty) => {}
        }
    });
 
    let mut line = String::new();
    let stdin = io::stdin();
    let _ = stdin.lock().read_line(&mut line);
 
    let _ = tx.send(());
}

