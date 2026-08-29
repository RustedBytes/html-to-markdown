# fast_h2m

High-performance HTML to Markdown converter.

See the workspace `README.md` for usage examples and project details.

## Optional asm_tl backend

`rustedbytes-tl` is the default runtime parser. Enable support for the
API-compatible, assembly-accelerated
[`asm_tl`](https://github.com/RustedBytes/asm_tl) backend with:

```toml
fast_h2m = { version = "0.4", features = ["asm-tl"] }
```

Then set `ConversionOptions::parser_backend` to either
`ParserBackend::RustedBytesTl` or `ParserBackend::AsmTl` for each call. The
assembly backend supports x86-64 Linux, AArch64 Linux, RISC-V 64 Linux, and
x86-64 Windows MSVC. Choose `TierStrategy::Tier2` or `FastDom` when the parser
must run for every conversion; `Auto` uses it only on Tier-2 fallback.
