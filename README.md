# Currency Converter

A lightweight command-line currency converter written in Rust for a university project at Technische Hochschule Deggendorf (THD).

## Features

- Convert between multiple currencies using real-time exchange rates
- Simple command-line interface
- Fast and efficient Rust implementation
- Error handling for invalid inputs and network issues

## Prerequisites

- Rust and Cargo (latest stable version recommended)
- Internet connection (for fetching exchange rates)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/ravindu47/currency_converter.git
cd currency_converter
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
cargo run
```

## Usage

The application will guide you through the conversion process interactively:

1. Enter the amount you want to convert
2. Select the source currency
3. Select the target currency
4. View the converted amount

Example:
```
Enter amount: 100
Select source currency (USD, EUR, GBP, JPY, etc.): USD
Select target currency (USD, EUR, GBP, JPY, etc.): EUR
100 USD = 92.50 EUR
```

## Project Structure

```
currency_converter/
├── src/
│   ├── main.rs          # Main application logic
│   ├── converter.rs     # Currency conversion functions
│   └── api.rs           # Exchange rate API integration
├── Cargo.toml          # Rust project configuration
└── README.md           # This file
```

## Dependencies

- `reqwest` - HTTP client for fetching exchange rates
- `serde` - Serialization/deserialization for JSON handling
- `tokio` - Async runtime for network requests

## Development

To run in development mode:
```bash
cargo run
```

To run tests:
```bash
cargo test
```

To build for release:
```bash
cargo build --release
```

## Contributing

This is a university project, but feedback is welcome. Please open an issue for any bugs or suggestions.

## License

This project is created for educational purposes at THD Deggendorf. Check the repository for specific licensing information.

## Author

Developed as part of a university project at Technische Hochschule Deggendorf.

## Notes

- Exchange rates are fetched from a public API (check source code for specific provider)
- Rates are updated with each conversion request
- This is a learning project and not intended for production financial decisions

---
*Created for THD Deggendorf University Project*
