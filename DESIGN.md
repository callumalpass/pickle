# Pickle Design Notes

## Product Scene

The user is glancing at an Android phone during the day or evening to answer a
specific agent request without losing the thread. The interface should be quiet,
high-contrast, and optimized for rapid triage.

## Visual Direction

Use the same pocket standards-notebook language as TaskNotes mobile: paper-like
surfaces, blue-black ink, Atkinson Hyperlegible text, Azeret Mono metadata,
precise rules, and compact request rows instead of cards. Accent blue indicates
focus and selection. Green is reserved for an explicit positive decision.
Dark mode uses deep blue-black paper rather than generic infrastructure navy.

## Interaction Principles

- The inbox should make pending, answered, and failed states obvious at a glance.
- Request detail should lead with the action needed, then show source context.
- Structured responses should render as native controls, not raw JSON.
- Approval buttons should be close to the requested context but visually stable.
- Offline or disconnected state should be visible and actionable. Because the
  application has no record replica, it should not imply that stale data is
  available offline.
