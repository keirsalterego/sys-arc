
# Minigrep Tool - Rust Implementation

## Overview
A command-line search utility that finds lines containing a query string in a file.

## Arguments
1. `query` - Search term
2. `filename` - File to search
3. `case` - Optional flag (set via `CASE` env var)

## Core Logic

### 1. Config Struct
```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        
        let query = args.next().ok_or("not enough arguments")?;
        let filename = args.next().ok_or("not enough arguments")?;
        
        let case = env::var("CASE").is_ok();
        
        Ok(Config {
            query,
            filename,
            case,
        })
    }
}
```

### 2. Run Function
```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(&config.filename)?;
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line?;
        let matches = if config.case {
            line.to_lowercase().contains(&config.query.to_lowercase())
        } else {
            line.contains(&config.query)
        };
        
        if matches {
            println!("{}", line);
        }
    }
    
    Ok(())
}
```

## Usage Example

```bash
$ CASE=1 minigrep "to" poem.txt
```

This searches for lines containing "to" in `poem.txt` with case-insensitive matching enabled.

## Key Features

- Efficient line-by-line filtering
- Lifetime annotations ensure no dangling references
- Environment variable configuration
- Error handling with Result types
- Support for both case-sensitive and case-insensitive searches
## Performance Considerations

- The `search` functions use iterators and closures for efficient memory usage without allocating intermediate collections until the final `collect()` call
- Each line is processed only once during the filter operation
This documentation explains that the case-insensitive variant implementation converts all input strings to lowercase before performing comparisons. While this approach introduces a minor performance cost due to the string conversion process, it maintains a clean architectural separation by keeping the case-insensitive logic isolated from other program logic.
- The case-insensitive variant converts strings to lowercase for comparison, adding a small performance overhead but maintaining clean separation of concerns