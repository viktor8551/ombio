# Issue tracker: GitHub

Issues and specs live in GitHub Issues in `viktor8551/ombio`.
Use the `gh` CLI.

## Conventions

- Create: `gh issue create --title "..." --body-file <file>`.
- Read: `gh issue view <number> --json number,title,body,labels,comments`.
- List: `gh issue list --state open --json number,title,body,labels`.
- Comment: `gh issue comment <number> --body-file <file>`.
- Apply or remove labels: `gh issue edit <number> --add-label "..."`
  or `--remove-label "..."`.
- Close: `gh issue close <number>`.
- Pass `--repo viktor8551/ombio` when outside this clone.
- For multiline bodies, write the text to a temporary file and use
  `--body-file`.

“Publish to the issue tracker” means create a GitHub issue.
“Fetch the relevant ticket” means read the GitHub issue.

## Pull requests as a triage surface

PRs as a request surface: no.

## Wayfinding

Keep the map in an issue labelled `wayfinder:map`.
Link child tickets using GitHub sub-issues where available; otherwise
use a task list in the map and `Part of #<map>` in each child.

Use `wayfinder:<type>` labels for research, prototype, grilling, and task.
Track blockers using native issue dependencies where available;
otherwise add `Blocked by: #<number>` to the issue body.

Select the first open, unassigned child with no open blockers.
Claim it by assigning yourself. Resolve it by commenting with the
answer, closing it, and adding a decision pointer to the map.
