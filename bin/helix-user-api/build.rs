use void_budi::*;
fn main() {
    BuildDataInjector::new()
        .with_git_last_commit_revision_hash()
        .with_git_last_commit_short_revision_hash()
        .with_git_last_commit_message()
        .with_git_last_commit_date()
        .with_version_name();
}
