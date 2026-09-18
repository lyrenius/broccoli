# Public scoreboard

Administrators can choose **Show Public Scoreboard** from the contest list's
menu or the contest administration panel. The link opens
`/contests/<id>/rankings?view=public` in a new tab.

## Publish a scoreboard

Contest visibility and score visibility are separate settings. Making a
contest public does not silently publish live scores. Existing contest
configuration and defaults are preserved.

For anonymous spectators, make the contest public and keep it within its
activation/deactivation window. Then open the contest's **Configure** menu:

- For ICPC, enable **Public Live Scoreboard** (`public_standings`). It defaults
  to false, which keeps the full board hidden before and during the contest.
  Configure **Scoreboard Freeze (minutes)** as needed. The freeze remains in
  force after the contest until an organizer reveals the board.
- For IOI, use the existing scoreboard visibility setting. Public view follows
  that setting and the configured publication phase, without the organizer's
  visibility override or a logged-in contestant's own-row exception.
- Codelink already uses the same standings for all viewers with contest access;
  its existing access checks and qualification rules remain unchanged.

An administrator can preview scoring visibility for a private or inactive
contest, but the URL does not give anonymous users access to it. A private
scoreboard may be empty in public view until its publication phase.

## View contract

The ranking page passes `publicView` to scoreboard slots. ICPC and IOI append
`view=public` to the scoreboard request and use a distinct query cache key.
Only the scoreboard read path derives an unprivileged viewer; contest access
checks still use the original authenticated request. Reveal and other writes
retain their existing authorization and are not triggered by opening the view.

In the ICPC public view, **every** row follows the freeze, including the logged-in
viewer's own row. The Reveal control is hidden. Normal ranking behavior is
unchanged; this feature does not add a floating live personal row.

Custom contest plugins are not rendered as a public scoreboard until they
implement this contract and are added to the supported-format list.
