import { NATIVE_PLATFORMS } from './platforms.mjs';

export default {
    slug: 'shell', name: 'Bash', deckHeading: null,
    versions: { runtime: 'No language runtime required', platforms: NATIVE_PLATFORMS },
    backends: 'cli',
    lang: 'bash', tab: 'Bash',
    blurb: 'Pipe text in, read the answer out, and branch on the exit code.',
    unsureWord: '`null`, and exit code 3',
    facts: '`--facts` prints one line of run facts on standard error, after the answers.',
    install: [
      ['curl -fsSL https://thinkthen.dev/install.sh | sh', 'Download script.'],
      ['brew install botassembly/thinkthen/thinkthen', 'Homebrew, an option on a Mac.'],
    ],
    windows: [
      'On Windows x86-64, download `https://thinkthen.dev/install.ps1`, read it, and run `.\\install.ps1` in Windows PowerShell 5.1 or PowerShell 7. It installs `thinkthen.exe` and its receipt in `%LOCALAPPDATA%\\Programs\\thinkthen`, or in the folder `THINKTHEN_INSTALL_DIR` names. That folder must be an absolute path on a local NTFS drive.',
      'The script checks the archive against its checksum and checks the command\'s version. A checksum does not prove who published the archive.',
      'The script changes no execution policy, profile, registry or runtime configuration. It prints the command\'s path. When the install folder is not on `PATH`, it also prints a `PATH` command for the current session.',
      'To remove ThinkThen, delete `thinkthen.exe`, `thinkthen.install.json` and `.thinkthen-install.lock` from the install folder.',
    ],
    uninstall: {
      text: 'To remove the download, delete the binary and its receipt. If you set `THINKTHEN_INSTALL_DIR`, delete the same two files there. With Homebrew, run `brew uninstall thinkthen`.',
      code: 'rm ~/.local/bin/thinkthen \\\n  ~/.local/bin/thinkthen.install.json',
    },
    particular: [
      'Standard input carries the evidence. Standard output carries the answer and nothing else.',
      'On one piece of evidence, the exit code is the answer. `if` and `case` read it directly.',
      '`--jobs` sets how many requests run at once.',
      '`--plan` prints the plan and needs no key.',
    ],
  };
