# Keep each key line. Keep only the names, and on relation lines only the edges, whose kinds $kinds lists.
.value.entities |= map(select(.kind | IN($kinds[])))
| if has("relations") then .relations |= map(select((.source.kind | IN($kinds[])) and (.target.kind | IN($kinds[])))) else . end
