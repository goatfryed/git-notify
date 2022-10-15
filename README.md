# git-notify [![CI](https://github.com/goatfryed/git-notify/actions/workflows/CI.yml/badge.svg)](https://github.com/goatfryed/git-notify/actions/workflows/CI.yml)
Find out, when your most relevant files change. once.

# Project goals
- Improve team collaboration by raising awareness for source code documentation.
- Be up to date, but not ahead of date.

This project is for you and your team, if the following feels familiar:
- Did you ever write documentation in the source that ended up unread by your colleagues?
- Did you have to ping all your colleagues in stand up / slack / .. about your latest to project setups?
- Did you add a new, cool helper, but nobody noticed?
- Did you scim through the main branch changes and git blame manually your usual suspects?
- Did your colleague tell you about his hot upcoming change but then you just waited until its merged?

Instead of security filters (e.g. CODEOWNERS),
or some high level general file tracking with email alerts on your remote's main,
git-notify targets day-to-day work in your local source code. Don't ask, get told when important things change.

The main focus is to improve collaboration via doc as code.

## With IDEs in mind
The goal is to implement a reusable core as a git extension that lets you track and retrieve changes based on your (controlled) view history.

On top of that, we want to implement IDE plugins, that visualize and manage the core
