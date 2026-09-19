# The live script does not exist

Found 2026-09-19. `AGENTS.md` says a live call to a paid backend runs only from `sdlc/scripts/live`, by hand, under a cap, with Ian's authorization. No such script exists. The live attempts so far were typed by hand against the built binary and once against the endpoint with `curl`. Each returned status 402 and spent nothing.

The live probe that `sdlc/planning/open-concerns.md` lists needs the script first. The script should read the key from the environment variable the profile names, refuse to run when the spend recorded in `sdlc/planning/plan.md` has reached the limit, write every exchange into a recording folder, and print the input tokens each call used. The ticket that writes the probe also writes the script.
