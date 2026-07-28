# Pickle Architecture

## Core Decision

Pickle is an mdbase-backed request/response inbox. The mdbase files are the
canonical store; the server, CLI, Android client, and automations are views and
writers over those files.

Tickle owns scheduling and job execution. Agents create Pickle requests when
they need a human decision; Tickle jobs later read linked response files and
continue.

## Components

- `src/main.rs`: Rust CLI and daemon entrypoint.
- `src/collection.rs`: mdbase collection adapter and request-state derivation.
- `src/server.rs`: authenticated HTTP API plus WebSocket event stream.
- `src/templates.rs`: built-in mdbase collection/type templates.
- `src/legacy_sqlite.rs`: one-way SQLite migration importer.
- `../pickle-android`: Capacitor application that reads and writes the
  collection only through mdbase connect and receives opaque managed push
  signals.
- `../pickle-obsidian`: Obsidian plugin for browsing/responding through Bases.

## Collection Contract

A collection contains a first-class `pickle.request` `1.0.0` data contract,
one or more implementing request types, and response files. The built-in
`pickle_request` type maps its fields through `implements`; consumers resolve
the contract registry and union every implementation instead of depending on
that concrete type name.

Responses link
to requests through their `request` field. That link is the lifecycle boundary:

1. No linked response means `pending`.
2. One linked response means `answered`.
3. More than one linked response means `conflict`.
4. `status: cancelled` on the request still means `cancelled`.

The request `status` field is retained only as a legacy/cancel marker. Writers
must not set `status: answered`.

## Request Lifecycle

1. An agent runs `pickle ask` or creates a compatible request record.
2. Pickle writes one `requests/*.md` file and captures attachments under
   `attachments/<request-id>/`.
3. mdbase exposes the request and its response type through ordinary collection
   operations. The optional Pickle daemon can still offer its convenience API.
4. Android or Obsidian renders the response form from the mdbase type
   definition.
5. The user submits a structured response.
6. The client creates one linked `responses/*.md` record through mdbase.
7. A later automation calls `pickle response <id>` or `pickle wait <id>`.

## Multi-Collection Selection

`~/.config/pickle/config.json` maps collection names to mdbase roots and stores
the default collection. The CLI accepts global `--collection <name-or-path>`.
The HTTP API accepts `?collection=<name>` or `X-Pickle-Collection`.

This lets one server manage a personal default collection, a TaskNotes
`.ops/_pickle` collection, and any future project-specific collections without
copying daemons.

## Attachments

Attachments are request-scoped captured artifacts. The request frontmatter stores
relative `attachment_paths`; file bytes live inside the mdbase collection.
The Android client shows attachment metadata until mdbase connect exposes
binary reads, then can add previews without changing the request protocol.

## Event Stream

The event stream is derived from the current mdbase state rather than a separate
event table. Each request emits `request.created`; each linked response emits
`request.answered`. Event ids are stable for a given sorted snapshot and suitable
for client catch-up polling.

## Schema Surface

Response schemas are mdbase type definitions. Clients read the portable
JSON Schema 2020-12 descriptor from `describe`; the optional Pickle API embeds
the same definition in each request. Supported field shapes include strings,
enums, booleans, lists, objects, links, datetimes, defaults, generated fields,
and required markers.
