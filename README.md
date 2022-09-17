# git-change-announcements
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
git-file-subscriptions targets day-to-day work in your local source code. Don't ask, get told when important things change.

The main focus is to improve collaboration via doc as code.

### Imagine
You checkout a repository.
You see the README.md that tells you all about project setup.
You work in the project, get accustomed and never look back.
One morning, you do your usual pull and an info pops up that the README.md changed.
You take a look an notice that a new service got added that requires extra setup.
You follow the steps and you're happy.

### But wait! Isn't that exactly git diff?
you might say. That's displayed anyway on pull. What's the benefit?

* Read less, now more. Let's be honest. Do you always read the whole diffs? Just find out the important bits!
* Focus on IDE integration and tracking
* Focus on multi branching work. I've read a change on another branch? I'm even the author? Get out of my way



