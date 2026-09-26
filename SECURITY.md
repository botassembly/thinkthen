# Security

Report a vulnerability privately through GitHub: open the repository's Security tab and choose "Report a vulnerability". Do not open a public issue for it.

Include the command or library call, the version from `thinkthen --version`, and what an attacker gains. Never include an API key.

`thinkthen` sends the evidence you give it to the backend address you name, with the key from `THINKTHEN_API_KEY`. A report about where evidence or the key can travel is in scope. So is a report about a file the tool writes outside the paths `README.md` names.
