# 0253 cache binding design acceptance

Fresh independent High design review accepted `b88799641d2a95b70dc4364abfc2e88399d08b48`. The coordinator approves the bounded read-only status field and fixed whole-folder recovery instruction within the original source issue. Status observes the marker; the existing writer gate still owns admission. The additive status field retains the existing unreleased schema and field meanings. Public documentation stays held.

The first review found that the existing marker reader checked the path and then used plain `File::open`. A replacement FIFO could block before identity validation, and a replacement symlink could be followed. The corrected design requires the existing cache-entry no-follow/nonblocking open pattern and a controlled pre-open replacement test. It retains bounded parsing and before/opened/after identity checks. This is a concrete reader correction, not permission to weaken the marker or add automatic removal.

The reviewer checked the Unix/Windows source pattern and existing FIFO fixture pattern without running tests or editing files. Implementation and fresh High code review remain required. No product item closes at this design acceptance.
