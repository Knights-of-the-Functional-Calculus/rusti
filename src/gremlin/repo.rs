use gremlin_client::process::traversal::{GraphTraversalSource, SyncTerminator};

fn fetchRepo(g: &GraphTraversalSource<SyncTerminator>, label: &str, repo_name: &str) -> RetType {
    if let Ok(vertex) = g.v(()).has_label(label).has(("name", repo_name)).to_list() {
        return vertex;
    } else {
        g.add_v(label);
    }
}
