# Ternoa CLI

### Description 
This is a Rust command-line tool for the Ternoa chain.

### Features
It currently supports queries on Ternoa chain for specific parameters

### Installation
```cargo install ternoa-cli```

### Usage
```ternoa-cli --help```

##### For count-related queries
```ternoa-cli count --help```

##### For state-related queries
```ternoa-cli state --help```

##### For specifying network use the -n flag.
**Example 1**: To get count of nominators from alphanet:
```ternoa-cli -n alphanet count nominators```

**Example 2**: To get count of nfts from mainnet:
```ternoa-cli -n mainnet count nfts``` or
```ternoa-cli count nfts``` (as *mainnet* is the default network selected)

