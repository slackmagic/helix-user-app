use void_budi::*;
fn main() {
    GitDataInjector::new()
        .with_last_commit_revision_hash()
        .with_last_commit_short_revision_hash()
        .with_last_commit_message()
        .with_last_commit_date();
}
