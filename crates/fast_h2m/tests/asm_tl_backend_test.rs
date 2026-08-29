#![cfg(all(
    feature = "asm-tl",
    any(
        all(target_arch = "x86_64", target_os = "linux"),
        all(target_arch = "aarch64", target_os = "linux"),
        all(target_arch = "riscv64", target_os = "linux"),
        all(target_arch = "x86_64", target_os = "windows", target_env = "msvc")
    )
))]

use fast_h2m::{ConversionOptions, ParserBackend, TierStrategy, convert};

#[test]
fn asm_tl_backend_converts_tier2_dom() {
    let html = r#"
        <article>
            <h1>Assembly backend</h1>
            <p>Nested <strong>content</strong> with an <a href="https://example.com">attribute</a>.</p>
            <ul><li>First</li><li>Second</li></ul>
        </article>
    "#;
    let options = ConversionOptions {
        tier_strategy: TierStrategy::Tier2,
        parser_backend: ParserBackend::AsmTl,
        extract_metadata: false,
        ..ConversionOptions::default()
    };

    let result = convert(html, options).expect("asm_tl-backed Tier-2 conversion should succeed");
    let markdown = result.content.expect("conversion should return Markdown");

    assert!(markdown.contains("# Assembly backend"));
    assert!(markdown.contains("**content**"));
    assert!(markdown.contains("[attribute](https://example.com)"));
    assert!(markdown.contains("- First"));
    assert!(markdown.contains("- Second"));
}

#[test]
fn asm_tl_and_rustedbytes_tl_can_be_selected_at_runtime() {
    let html = "<h1>Runtime choice</h1><p>Same process, different parser.</p>";
    let convert_with = |parser_backend| {
        convert(
            html,
            ConversionOptions {
                tier_strategy: TierStrategy::Tier2,
                parser_backend,
                extract_metadata: false,
                ..ConversionOptions::default()
            },
        )
        .expect("selected parser backend should convert")
        .content
        .expect("conversion should return Markdown")
    };

    let rustedbytes = convert_with(ParserBackend::RustedBytesTl);
    let assembly = convert_with(ParserBackend::AsmTl);

    assert_eq!(assembly, rustedbytes);
}

#[test]
fn asm_tl_backend_converts_fast_dom() {
    let result = convert(
        "<h2>Fast DOM</h2><p>Assembly parser</p>",
        ConversionOptions {
            tier_strategy: TierStrategy::FastDom,
            parser_backend: ParserBackend::AsmTl,
            ..ConversionOptions::default()
        },
    )
    .expect("asm_tl-backed FastDom conversion should succeed");

    let markdown = result.content.expect("conversion should return Markdown");
    assert!(markdown.contains("## Fast DOM"));
    assert!(markdown.contains("Assembly parser"));
}
