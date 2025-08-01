use biome_formatter::Printed;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_formatter::format_node;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_syntax::JsonRoot;
use biome_rowan::AstNode;
use biome_test_utils::BenchCase;
use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use std::collections::HashMap;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(all(
    any(target_os = "macos", target_os = "linux"),
    not(target_env = "musl"),
))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

// Jemallocator does not work on aarch64 with musl, so we'll use the system allocator instead
#[cfg(all(target_env = "musl", target_os = "linux", target_arch = "aarch64"))]
#[global_allocator]
static GLOBAL: std::alloc::System = std::alloc::System;
fn bench_formatter(criterion: &mut Criterion) {
    let mut all_suites = HashMap::new();
    all_suites.insert("json", include_str!("libs-json.txt"));
    let mut libs = vec![];
    libs.extend(all_suites.values().flat_map(|suite| suite.lines()));

    let mut group = criterion.benchmark_group("json_formatter");

    for lib in libs {
        let test_case = BenchCase::try_from(lib);

        match test_case {
            Ok(test_case) => {
                let code = test_case.code();
                let parsed = parse_json(code, JsonParserOptions::default());
                group.throughput(Throughput::Bytes(code.len() as u64));
                group.bench_with_input(
                    BenchmarkId::from_parameter(test_case.filename()),
                    &code,
                    |b, _| {
                        fn format(root: JsonRoot) -> Printed {
                            let formatted =
                                format_node(JsonFormatOptions::default(), root.syntax()).unwrap();
                            let printed = formatted.print();
                            drop(formatted);
                            printed.expect("Document to be valid")
                        }
                        b.iter(|| {
                            black_box(format(parsed.tree()));
                        })
                    },
                );
            }
            Err(e) => println!("{e:?}"),
        }
    }
    group.finish();
}

criterion_group!(json_formatter, bench_formatter);
criterion_main!(json_formatter);
