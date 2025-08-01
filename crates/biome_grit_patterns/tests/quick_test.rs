use biome_grit_parser::parse_grit;
use biome_grit_patterns::{
    GritQuery, GritQueryResult, GritTargetFile, GritTargetLanguage, JsTargetLanguage,
};
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsFileSource;

// Use this test to quickly execute a Grit query against a source snippet.
#[ignore]
#[test]
fn test_query() {
    let parse_grit_result = parse_grit(
        "
        `import $what from $where`
        ",
    );
    if !parse_grit_result.diagnostics().is_empty() {
        panic!("Cannot parse query:\n{:?}", parse_grit_result.diagnostics());
    }

    let query = GritQuery::from_node(
        parse_grit_result.tree(),
        None,
        GritTargetLanguage::JsTargetLanguage(JsTargetLanguage),
        Vec::new(),
    )
    .expect("could not construct query");

    if !query.diagnostics.is_empty() {
        println!("Diagnostics from compiling query:\n{:?}", query.diagnostics);
    }

    let body = r#"import { PrismaClient } from "@prisma/client/runtime";"#;

    let parsed = parse(body, JsFileSource::js_module(), JsParserOptions::default());

    let file = GritTargetFile::new("test.js", parsed.into());
    let GritQueryResult { effects, logs, .. } =
        query.execute(file).expect("could not execute query");

    println!("Effects: {effects:#?}");

    if !logs.is_empty() {
        println!(
            "\n## Logs\n\n{}",
            logs.iter()
                .map(|log| format!(
                    "Message: {}Syntax: {}",
                    log.message,
                    log.syntax_tree.as_deref().unwrap_or_default()
                ))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
