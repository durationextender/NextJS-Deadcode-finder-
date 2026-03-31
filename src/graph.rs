use crate::Lexer;
use crate::MyParser;
use crate::parser::ExportStatement;
use crate::parser::ImportStatement;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug)]
struct Node {
    imports: Vec<ImportStatement>,
    exports: Vec<ExportStatement>,
}

pub struct Graph {
    nodes: HashMap<PathBuf, Node>,
}

fn resolve_import(current_file: &PathBuf, source: &str, root_path: &Path) -> Option<PathBuf> {
    let (base_path, relative_source) = if source.starts_with("@/") {
        (root_path, &source[2..])
    } else {
        (current_file.parent()?, source)
    };

    let joined = base_path.join(relative_source);
    let extensions = ["ts", "tsx", "js", "jsx"];
    let index_dir = ["index.tsx", "index.ts", "index.js", "index.jsx"];
    for ext in index_dir {
        let with_index_ext = joined.join(ext);
        if let Ok(canonical) = std::fs::canonicalize(&with_index_ext) {
            println!("index ext: {:?}", with_index_ext);
            return Some(canonical);
        }
    }
    for ext in extensions {
        let with_ext = joined.with_extension(ext);
        if let Ok(canonical) = std::fs::canonicalize(&with_ext) {
            return Some(canonical);
        }
    }

    None
}

pub fn find_dead_exports(graph: &Graph, root: &Path) -> Vec<(PathBuf, String)> {
    let mut import_set: HashSet<(String, String)> = HashSet::new();
    let mut dead_exports: Vec<(PathBuf, String)> = Vec::new();
    let nextjs_exports_files = [
        "page.tsx",
        "layout.tsx",
        "not-found.tsx",
        "loading.tsx",
        "error.tsx",
    ];
    let nextjs_exports_names = [
        "metadata",
        "dynamic",
        "revalidate",
        "generateStaticParams",
        "generateMetadata",
        "default",
    ];
    for (file_name, node) in &graph.nodes {
        for imports in &node.imports {
            for names in &imports.names {
                if let Some(resolved) = resolve_import(file_name, &imports.source, root) {
                    import_set.insert((resolved.to_string_lossy().into_owned(), names.to_string()));
                }
            }
        }
    }
    for (file_name, node) in &graph.nodes {
        for exports in &node.exports {
            let is_match_file = nextjs_exports_files.iter().any(|e| {
                file_name
                    .file_name()
                    .map(|name| name.to_string_lossy() == *e)
                    .unwrap_or(false)
            });

            for name in &exports.names {
                let is_match_file_name_exports = nextjs_exports_names.iter().any(|e| name == *e);
                if is_match_file_name_exports {
                    continue;
                }
                let is_route_file = file_name
                    .file_name()
                    .map(|n| n.to_string_lossy() == "route.ts")
                    .unwrap_or(false);

                let route_exports = ["GET", "POST", "PUT", "DELETE", "PATCH", "nextAuth"];

                if is_route_file && route_exports.iter().any(|e| name == *e) {
                    continue;
                }
                let is_middleware_file = file_name
                    .file_name()
                    .map(|n| n.to_string_lossy() == "middleware.ts")
                    .unwrap_or(false);

                let middleware_exports = "config";

                if is_middleware_file && middleware_exports == name {
                    continue;
                }

                if is_match_file && nextjs_exports_names.iter().any(|e| name == *e) {
                    continue;
                }

                if let Ok(canonical) = std::fs::canonicalize(file_name) {
                    let canonical_str = canonical.to_string_lossy().into_owned();
                    if !import_set.contains(&(canonical_str, name.clone())) {
                        dead_exports.push((file_name.clone(), name.clone()));
                    }
                }
            }
        }
    }
    dead_exports
}

pub fn build_graph(scanned_directories: Vec<PathBuf>) -> Graph {
    let graph_nodes: HashMap<PathBuf, Node> = scanned_directories
        .iter()
        .filter_map(|path| {
            let content = read_to_string(path).ok()?;

            let mut lexer = Lexer::new(content.chars().collect());
            let tokens = lexer.tokenize().ok()?;

            let mut parser = MyParser::new(tokens);
            let (imports, exports) = parser.parse();

            Some((path.clone(), Node { imports, exports }))
        })
        .collect();
    let graph = Graph { nodes: graph_nodes };

    graph
}
