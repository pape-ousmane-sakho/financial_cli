# Finance CLI 🦀📈

A blazing fast command-line tool built in Rust that fetches real-time financial data for investors and traders directly in your terminal.

## Features

- Fetches real-time stock quotes from Alpha Vantage API
- Displays price, change, volume and latest trading day
- Clean and readable terminal output
- Secure API key handling via environment variables
- Zero bloat — just the data you need, instantly

## Usage

```bash
cargo run -- AAPL TSLA MSFT
```

## Example Output

```
Symbol:       AAPL
Price:        $247.99
Change:       -0.9700
Change %:     -0.3896%
Volume:       88331081
Last trading: 2026-03-20
```

## Setup

### 1 — Get a free API key

Sign up at [Alpha Vantage](https://www.alphavantage.co/support/#api-key) — free, no credit card required.

### 2 — Set your API key as an environment variable

**Windows PowerShell:**
```powershell
$env:ALPHA_VANTAGE_KEY="your_key_here"
```

**Mac/Linux:**
```bash
export ALPHA_VANTAGE_KEY="your_key_here"
```

### 3 — Run the program

```bash
cargo run -- AAPL
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `ALPHA_VANTAGE_KEY` | Your Alpha Vantage API key |

See `.env.example` for reference.

## Built With

- [Rust](https://www.rust-lang.org/) — systems programming language
- [reqwest](https://crates.io/crates/reqwest) — HTTP client
- [serde](https://crates.io/crates/serde) — JSON deserialization

## Roadmap

- [ ] Multiple tickers at once
- [ ] Async requests with tokio
- [ ] Historical data with ASCII price chart
- [ ] Save watchlists to a local file
- [ ] Color output (green for gains, red for losses)

## License

MIT
