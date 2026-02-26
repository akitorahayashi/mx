pub fn resolve_alias(key: &str) -> Option<&'static str> {
    match key {
        "tk" => Some("tasks.md"),
        "atk" => Some("additional_tasks.md"),
        "tko" => Some("tasks_outline.md"),
        "is" => Some("issue.md"),
        "rq" => Some("requirements.md"),
        "rv" => Some("review.md"),
        "df" => Some("diff.md"),
        "pdt" => Some("pending/tasks.md"),
        "pdr" => Some("pending/requirements.md"),
        "wn" => Some("warnings.md"),
        "er" => Some("error.md"),
        "if" => Some("info.md"),
        "aif" => Some("additional_info.md"),
        "rf" => Some("reference.md"),
        "rp" => Some("report.md"),
        "pl" => Some("plan.md"),
        "sg" => Some("suggestion.md"),
        "sm" => Some("summary.md"),
        "cg" => Some("changes.md"),
        _ => None,
    }
}
