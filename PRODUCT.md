# Pickle Product Context

## Register

product

## Users

Developers and coding-agent users who run local automations and need a durable
way for those agents to ask for human decisions. The first user is Callum, using
Tickle jobs, repo-local ops files, mdbase connect, and an Android phone.

## Purpose

Pickle is a personal agent inbox for structured requests, approvals, annotations,
and automation handoffs. Agents file a request, the user responds asynchronously,
and later automation can continue from the recorded response.

Pickle is not a chat app, task manager, or generic message queue. It is the
human checkpoint in a local automation system.

## Product Principles

- Human responses are durable automation inputs, not ephemeral notifications.
- The mdbase collection remains the system of record, whether its authority is
  local or hosted.
- Android is a thin mdbase connect client with no Pickle-specific server
  address, bearer token, or local record replica.
- Push is an opaque wake-up signal delivered through mdbase connect. Request
  paths and content never pass through the control plane.
- Schemas should be simple enough for agents to emit and clients to render.
- Every request should remain inspectable as portable Markdown and through the
  UI.

## Tone

Small, precise, grounded, and calm. Pickle should feel like a quiet operations
desk: pending decisions are obvious, context is easy to scan, and approval
actions are deliberate.

## Anti-References

Avoid enterprise workflow suites, generic SaaS inboxes, noisy chat timelines,
mascot-heavy productivity tools, decorative dashboards, and opaque automation
platforms.
