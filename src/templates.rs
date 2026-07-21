pub const REQUEST_TYPE: &str = "pickle_request";
pub const APPROVAL_RESPONSE_TYPE: &str = "pickle_response_approval";
pub const ACK_RESPONSE_TYPE: &str = "pickle_response_ack";

pub const MDBASE_CONFIG: &str = r#"spec_version: "0.3.0"
name: Pickle
description: Local mdbase collection for Pickle requests and responses.
settings:
  types_folder: "_types"
  record_extensions: [md]
  validation: error
  exclude:
    - ".git"
    - "node_modules"
    - ".mdbase"
    - "attachments/**"
  include_subfolders: true
  explicit_type_keys: [type, types]
"#;

pub const PICKLE_REQUEST_TYPE: &str = r#"---
kind: mdbase.type
name: pickle_request
version: 1
description: Async request that needs a human response.
schema:
  dialect: json-schema-2020-12
  value:
    $schema: "https://json-schema.org/draft/2020-12/schema"
    type: object
    additionalProperties: true
    required: [title, response_type]
    properties:
      type:
        const: pickle_request
      id:
        type: string
      title:
        type: string
        minLength: 1
      source:
        type: string
      message:
        type: string
      kind:
        enum: [approval, choice, input, notice, message]
      status:
        enum: [pending, answered, cancelled]
        description: Legacy lifecycle marker. Response links are authoritative for answered state.
      priority:
        enum: [low, normal, high, urgent]
      response_type:
        type: string
      created_at:
        type: string
        format: date-time
      due_at:
        type: string
        format: date-time
      dedupe_key:
        type: string
      tags:
        type: array
        items:
          type: string
      links:
        type: array
        items:
          type: object
          additionalProperties: false
          properties:
            label:
              type: string
            url:
              type: string
            path:
              type: string
      attachment_paths:
        type: array
        items:
          type: string
      metadata:
        type: object
        additionalProperties: true
      context:
        type: object
        additionalProperties: false
        properties:
          cwd:
            type: string
          repo:
            type: string
          task:
            type: string
collection:
  display:
    name_field: title
  unique:
    - field: id
      scope: collection
lifecycle:
  on_create:
    set:
      created_at:
        now: true
---
"#;

pub const PICKLE_APPROVAL_RESPONSE_TYPE: &str = r#"---
kind: mdbase.type
name: pickle_response_approval
version: 1
description: Approve, reject, or request revision for a Pickle request.
schema:
  dialect: json-schema-2020-12
  value:
    $schema: "https://json-schema.org/draft/2020-12/schema"
    type: object
    additionalProperties: true
    required: [request, decision]
    properties:
      type:
        const: pickle_response_approval
      id:
        type: string
      request:
        type: string
      decision:
        enum: [approve, reject, revise]
      comment:
        type: string
      responded_at:
        type: string
        format: date-time
      responder:
        type: string
      attachment_paths:
        type: array
        items:
          type: string
collection:
  display:
    name_field: decision
  links:
    request:
      target_type: pickle_request
      validate_exists: true
  unique:
    - field: id
      scope: collection
lifecycle:
  on_create:
    set:
      id:
        ulid: true
      responded_at:
        now: true
---
"#;

pub const PICKLE_ACK_RESPONSE_TYPE: &str = r#"---
kind: mdbase.type
name: pickle_response_ack
version: 1
description: Acknowledge that a Pickle message was read.
schema:
  dialect: json-schema-2020-12
  value:
    $schema: "https://json-schema.org/draft/2020-12/schema"
    type: object
    additionalProperties: true
    required: [request]
    properties:
      type:
        const: pickle_response_ack
      id:
        type: string
      request:
        type: string
      message:
        type: string
      responded_at:
        type: string
        format: date-time
      responder:
        type: string
      attachment_paths:
        type: array
        items:
          type: string
collection:
  display:
    name_field: message
  links:
    request:
      target_type: pickle_request
      validate_exists: true
  unique:
    - field: id
      scope: collection
lifecycle:
  on_create:
    set:
      id:
        ulid: true
      responded_at:
        now: true
---
"#;

pub const MDBASE_CONFIG_V02: &str = r#"spec_version: "0.2.1"
name: Pickle
description: Local mdbase collection for Pickle requests and responses.
settings:
  types_folder: "_types"
  default_validation: "error"
  default_strict: false
  exclude:
    - ".git"
    - "node_modules"
    - ".mdbase"
    - "attachments/**"
  include_subfolders: true
  explicit_type_keys: ["type", "types"]
  cache_folder: ".mdbase"
"#;

pub const PICKLE_REQUEST_TYPE_V02: &str = r#"---
name: pickle_request
description: Async request that needs a human response.
display_name_key: title
fields:
  id:
    type: string
    generated: ulid
    unique: true
  title:
    type: string
    required: true
  source:
    type: string
  message:
    type: string
  kind:
    type: enum
    values: [approval, choice, input, notice, message]
  status:
    type: enum
    description: Legacy lifecycle marker. Response links are authoritative for answered state.
    values: [pending, answered, cancelled]
  priority:
    type: enum
    values: [low, normal, high, urgent]
  response_type:
    type: string
    required: true
  created_at:
    type: datetime
    generated: now
  due_at:
    type: datetime
  dedupe_key:
    type: string
  tags:
    type: list
    items:
      type: string
  links:
    type: list
    items:
      type: object
      fields:
        label:
          type: string
        url:
          type: string
        path:
          type: string
  attachment_paths:
    type: list
    items:
      type: string
  metadata:
    type: object
  context:
    type: object
    fields:
      cwd:
        type: string
      repo:
        type: string
      task:
        type: string
---
"#;

pub const PICKLE_APPROVAL_RESPONSE_TYPE_V02: &str = r#"---
name: pickle_response_approval
description: Approve, reject, or request revision for a Pickle request.
display_name_key: decision
fields:
  id:
    type: string
    generated: ulid
    unique: true
  request:
    type: link
    target: pickle_request
    validate_exists: true
    required: true
  decision:
    type: enum
    values: [approve, reject, revise]
    required: true
  comment:
    type: string
  responded_at:
    type: datetime
    generated: now
  responder:
    type: string
  attachment_paths:
    type: list
    items:
      type: string
---
"#;

pub const PICKLE_ACK_RESPONSE_TYPE_V02: &str = r#"---
name: pickle_response_ack
description: Acknowledge that a Pickle message was read.
display_name_key: message
fields:
  id:
    type: string
    generated: ulid
    unique: true
  request:
    type: link
    target: pickle_request
    validate_exists: true
    required: true
  message:
    type: string
  responded_at:
    type: datetime
    generated: now
  responder:
    type: string
  attachment_paths:
    type: list
    items:
      type: string
---
"#;
