use fast_h2m::{ConversionOptions, ParserBackend};

#[cfg(not(all(
    feature = "asm-tl",
    any(
        all(target_arch = "x86_64", target_os = "linux"),
        all(target_arch = "aarch64", target_os = "linux"),
        all(target_arch = "riscv64", target_os = "linux"),
        all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")
    )
)))]
use fast_h2m::{ConversionError, TierStrategy, convert};

#[test]
fn rustedbytes_tl_is_the_default_runtime_backend() {
    assert_eq!(
        ConversionOptions::default().parser_backend,
        ParserBackend::RustedBytesTl
    );
}

#[cfg(not(all(
    feature = "asm-tl",
    any(
        all(target_arch = "x86_64", target_os = "linux"),
        all(target_arch = "aarch64", target_os = "linux"),
        all(target_arch = "riscv64", target_os = "linux"),
        all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")
    )
)))]
#[test]
fn unavailable_asm_tl_backend_returns_a_configuration_error() {
    let result = convert(
        "<p>Unavailable backend</p>",
        ConversionOptions {
            tier_strategy: TierStrategy::Tier2,
            parser_backend: ParserBackend::AsmTl,
            ..ConversionOptions::default()
        },
    );

    assert!(matches!(result, Err(ConversionError::ConfigError(_))));
}
