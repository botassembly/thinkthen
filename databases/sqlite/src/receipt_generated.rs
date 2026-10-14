// Generated from the canonical Rust result schema; do not edit.
use crate::receipt::{Field, Projection, Scalar, Shape};
pub(crate) const PROJECTIONS: &[Projection] = &[
    Projection {
        name: "annotation",
        union: false,
        fields: &[
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "answers",
                shape: Shape::Mapping("named_answer"),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "value",
                shape: Shape::Mapping("named_field_value"),
            },
        ],
    },
    Projection {
        name: "annotation_answer",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "failure",
                shape: Shape::Object("failure"),
            },
            Field {
                name: "failure_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "observations",
                shape: Shape::Array("observation"),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "question_sources",
                shape: Shape::Array("question_source"),
            },
            Field {
                name: "request",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "usage",
                shape: Shape::Object("token_usage"),
            },
            Field {
                name: "value",
                shape: Shape::Object("answer_value"),
            },
        ],
    },
    Projection {
        name: "answer",
        union: false,
        fields: &[
            Field {
                name: "confidence",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "level",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "pick",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "probabilities",
                shape: Shape::Mapping("named_probability"),
            },
            Field {
                name: "probability",
                shape: Shape::Scalar(Scalar::Real),
            },
        ],
    },
    Projection {
        name: "answer_value",
        union: true,
        fields: &[
            Field {
                name: "array_value",
                shape: Shape::Array("answer_value_array_value"),
            },
            Field {
                name: "boolean_value",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "number_value",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "string_value",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "answer_value_array_value",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "attempt",
        union: false,
        fields: &[
            Field {
                name: "ordinal",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "outcome",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "request_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "request_sha256",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "sdk_request_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "server_ms",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "status",
                shape: Shape::Scalar(Scalar::Integer),
            },
            Field {
                name: "wall_ms",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "batch_setting",
        union: true,
        fields: &[
            Field {
                name: "integer_value",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "string_value",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "batch_warning",
        union: false,
        fields: &[
            Field {
                name: "running",
                shape: Shape::Object("batch_setting"),
            },
            Field {
                name: "tuned_for",
                shape: Shape::Object("batch_setting"),
            },
        ],
    },
    Projection {
        name: "boundary_proposal",
        union: false,
        fields: &[
            Field {
                name: "end",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "length",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "probability",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "start",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "text",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "boundary_reading",
        union: false,
        fields: &[
            Field {
                name: "end",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "kept",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "length",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "probability",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "selected",
                shape: Shape::Object("selected_span"),
            },
            Field {
                name: "span_probability",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "start",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "strength",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "text",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "candidate_probability",
        union: false,
        fields: &[
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Real),
            },
        ],
    },
    Projection {
        name: "choice",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "images",
                shape: Shape::Array("image"),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "input_file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "question_name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "decision",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "images",
                shape: Shape::Array("image"),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "input_file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "question_name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Data),
            },
        ],
    },
    Projection {
        name: "declared_property",
        union: false,
        fields: &[
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "value",
                shape: Shape::Object("property_declaration"),
            },
        ],
    },
    Projection {
        name: "entity",
        union: false,
        fields: &[
            Field {
                name: "end",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "length",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "start",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "strength",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "text",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "entity_edge",
        union: false,
        fields: &[
            Field {
                name: "either",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "probability",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "relation",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("entity"),
            },
            Field {
                name: "target",
                shape: Shape::Object("entity"),
            },
        ],
    },
    Projection {
        name: "entity_probabilities",
        union: false,
        fields: &[
            Field {
                name: "edges",
                shape: Shape::Mapping("named_probability"),
            },
            Field {
                name: "end",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "kinds",
                shape: Shape::Mapping("named_probability"),
            },
            Field {
                name: "start",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "error",
        union: false,
        fields: &[
            Field {
                name: "cause",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "estimated_input_denial",
                shape: Shape::Object("input_budget_denial"),
            },
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "message",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "retryable",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "send_budget_denial",
                shape: Shape::Object("send_budget_denial"),
            },
            Field {
                name: "stopped",
                shape: Shape::Object("stopped"),
            },
        ],
    },
    Projection {
        name: "event",
        union: false,
        fields: &[
            Field {
                name: "detail",
                shape: Shape::Object("question_detail"),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "inputs",
                shape: Shape::Array("observed_input"),
            },
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "member",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "position",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "stage",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Data),
            },
        ],
    },
    Projection {
        name: "facts",
        union: false,
        fields: &[
            Field {
                name: "attempts",
                shape: Shape::Array("attempt"),
            },
            Field {
                name: "cache_answers",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "call_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "estimated_cost_usd",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "held_model_mismatch",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "input_tokens",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "largest_request_bytes",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "largest_request_estimated_input_tokens",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "model",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "output_tokens",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "records",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "requests_sent",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "seconds",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "token_estimate_method",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "usage_persistence",
                shape: Shape::Object("usage_persistence"),
            },
        ],
    },
    Projection {
        name: "failed_value",
        union: false,
        fields: &[Field {
            name: "failed",
            shape: Shape::Object("failure"),
        }],
    },
    Projection {
        name: "failure",
        union: false,
        fields: &[
            Field {
                name: "cause",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "field_value",
        union: true,
        fields: &[
            Field {
                name: "array_value",
                shape: Shape::Array("field_value_array_value"),
            },
            Field {
                name: "boolean_value",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "number_value",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "object_value",
                shape: Shape::Object("failed_value"),
            },
            Field {
                name: "string_value",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "field_value_array_value",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "filtered",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "images",
                shape: Shape::Array("image"),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "input_file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "question_name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Boolean),
            },
        ],
    },
    Projection {
        name: "find_answer",
        union: false,
        fields: &[
            Field {
                name: "confidence",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "pick",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "probabilities",
                shape: Shape::Mapping("candidate_probability"),
            },
        ],
    },
    Projection {
        name: "find_candidate",
        union: false,
        fields: &[
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "probability",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
        ],
    },
    Projection {
        name: "find_question",
        union: false,
        fields: &[
            Field {
                name: "batch",
                shape: Shape::Object("batch_setting"),
            },
            Field {
                name: "context_schema",
                shape: Shape::Object("input_declaration"),
            },
            Field {
                name: "item_schema",
                shape: Shape::Object("input_declaration"),
            },
            Field {
                name: "label_details",
                shape: Shape::Array("label"),
            },
            Field {
                name: "model",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "none",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "on",
                shape: Shape::Array("find_question_on"),
            },
            Field {
                name: "profile",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "text",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "verb",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "wording_version",
                shape: Shape::Scalar(Scalar::Integer),
            },
        ],
    },
    Projection {
        name: "find_question_on",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "find_selection",
        union: false,
        fields: &[Field {
            name: "selected",
            shape: Shape::Scalar(Scalar::Integer),
        }],
    },
    Projection {
        name: "found",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("find_answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "candidates",
                shape: Shape::Array("find_candidate"),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "question",
                shape: Shape::Object("find_question"),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "threshold",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Data),
            },
        ],
    },
    Projection {
        name: "image",
        union: false,
        fields: &[
            Field {
                name: "base64",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "height",
                shape: Shape::Scalar(Scalar::Integer),
            },
            Field {
                name: "media",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "width",
                shape: Shape::Scalar(Scalar::Integer),
            },
        ],
    },
    Projection {
        name: "indexed_source",
        union: false,
        fields: &[
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
        ],
    },
    Projection {
        name: "input_budget_denial",
        union: false,
        fields: &[
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_status",
                shape: Shape::Scalar(Scalar::Integer),
            },
            Field {
                name: "limit",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "input_declaration",
        union: false,
        fields: &[
            Field {
                name: "properties",
                shape: Shape::Mapping("declared_property"),
            },
            Field {
                name: "required",
                shape: Shape::Array("input_declaration_required"),
            },
            Field {
                name: "type",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "input_declaration_required",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "kind_description",
        union: false,
        fields: &[
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Data),
            },
        ],
    },
    Projection {
        name: "label",
        union: false,
        fields: &[
            Field {
                name: "description",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "located_edge",
        union: false,
        fields: &[
            Field {
                name: "either",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "probability",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "relation",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("located_endpoint"),
            },
            Field {
                name: "target",
                shape: Shape::Object("located_endpoint"),
            },
        ],
    },
    Projection {
        name: "located_endpoint",
        union: false,
        fields: &[
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "ordinal",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "record",
                shape: Shape::Scalar(Scalar::Data),
            },
        ],
    },
    Projection {
        name: "metadata",
        union: false,
        fields: &[
            Field {
                name: "answered_by",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "attempts",
                shape: Shape::Array("attempt"),
            },
            Field {
                name: "batch_setting",
                shape: Shape::Object("batch_setting"),
            },
            Field {
                name: "batch_warning",
                shape: Shape::Object("batch_warning"),
            },
            Field {
                name: "cached",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "context_sha256",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "failed_questions",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "model",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "observations",
                shape: Shape::Array("observation"),
            },
            Field {
                name: "origin",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "profile_warning",
                shape: Shape::Object("profile_warning"),
            },
            Field {
                name: "question_sha256",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "question_sources",
                shape: Shape::Array("question_source"),
            },
            Field {
                name: "questions_sha256",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "requests",
                shape: Shape::Array("metadata_requests"),
            },
            Field {
                name: "requests_sent",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "tool",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "url",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "usage",
                shape: Shape::Object("token_usage"),
            },
        ],
    },
    Projection {
        name: "metadata_requests",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "named_answer",
        union: false,
        fields: &[
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "value",
                shape: Shape::Object("annotation_answer"),
            },
        ],
    },
    Projection {
        name: "named_field_value",
        union: false,
        fields: &[
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "value",
                shape: Shape::Object("field_value"),
            },
        ],
    },
    Projection {
        name: "named_probability",
        union: false,
        fields: &[
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Real),
            },
        ],
    },
    Projection {
        name: "named_ranked_member",
        union: false,
        fields: &[
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "result",
                shape: Shape::Object("ranked_member_result"),
            },
        ],
    },
    Projection {
        name: "observation",
        union: false,
        fields: &[
            Field {
                name: "failure_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "observation_id",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "observed_input",
        union: false,
        fields: &[
            Field {
                name: "images",
                shape: Shape::Array("image"),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
        ],
    },
    Projection {
        name: "observed_probability",
        union: false,
        fields: &[
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "probability",
                shape: Shape::Scalar(Scalar::Real),
            },
        ],
    },
    Projection {
        name: "place",
        union: false,
        fields: &[
            Field {
                name: "end",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "start",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "plan",
        union: false,
        fields: &[
            Field {
                name: "estimated_bytes",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "estimated_input_tokens",
                shape: Shape::Object("token_band"),
            },
            Field {
                name: "first_body_utf8",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "largest_request_bytes",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "largest_request_estimated_input_tokens",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "records",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "requests",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "token_estimate_method",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "upper_bound",
                shape: Shape::Scalar(Scalar::Boolean),
            },
        ],
    },
    Projection {
        name: "position",
        union: false,
        fields: &[
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "images",
                shape: Shape::Array("position_images"),
            },
            Field {
                name: "last",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "position_images",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "probability_distribution",
        union: false,
        fields: &[
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "value",
                shape: Shape::Object("probability_values"),
            },
        ],
    },
    Projection {
        name: "probability_values",
        union: true,
        fields: &[
            Field {
                name: "array_value",
                shape: Shape::Array("observed_probability"),
            },
            Field {
                name: "number_value",
                shape: Shape::Scalar(Scalar::Real),
            },
        ],
    },
    Projection {
        name: "profile_warning",
        union: false,
        fields: &[
            Field {
                name: "running",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "tuned_for",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "property_declaration",
        union: false,
        fields: &[
            Field {
                name: "items",
                shape: Shape::Object("string_declaration"),
            },
            Field {
                name: "type",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "question",
        union: false,
        fields: &[
            Field {
                name: "batch",
                shape: Shape::Object("batch_setting"),
            },
            Field {
                name: "context_schema",
                shape: Shape::Object("input_declaration"),
            },
            Field {
                name: "false",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "item_schema",
                shape: Shape::Object("input_declaration"),
            },
            Field {
                name: "label_details",
                shape: Shape::Array("label"),
            },
            Field {
                name: "labels",
                shape: Shape::Array("question_labels"),
            },
            Field {
                name: "levels",
                shape: Shape::Array("question_levels"),
            },
            Field {
                name: "model",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "on",
                shape: Shape::Array("question_on"),
            },
            Field {
                name: "options",
                shape: Shape::Array("question_options"),
            },
            Field {
                name: "profile",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "text",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "true",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "verb",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "wording_version",
                shape: Shape::Scalar(Scalar::Integer),
            },
        ],
    },
    Projection {
        name: "question_detail",
        union: false,
        fields: &[
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "cached",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "confidence",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "failed_questions",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "failure",
                shape: Shape::Object("failure"),
            },
            Field {
                name: "failure_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "input_source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "input_sources",
                shape: Shape::Array("indexed_source"),
            },
            Field {
                name: "inputs",
                shape: Shape::Array("question_detail_inputs"),
            },
            Field {
                name: "model",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "observations",
                shape: Shape::Array("observation"),
            },
            Field {
                name: "probabilities",
                shape: Shape::Object("probability_distribution"),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "question_sha256",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "question_sources",
                shape: Shape::Array("question_source"),
            },
            Field {
                name: "raw_pick",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "reported_usage",
                shape: Shape::Object("reported_usage"),
            },
            Field {
                name: "requests",
                shape: Shape::Array("question_detail_requests"),
            },
            Field {
                name: "requests_sent",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "url",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "usage",
                shape: Shape::Object("token_usage"),
            },
            Field {
                name: "value",
                shape: Shape::Object("answer_value"),
            },
        ],
    },
    Projection {
        name: "question_detail_inputs",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Data),
        }],
    },
    Projection {
        name: "question_detail_requests",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "question_labels",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "question_levels",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "question_on",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "question_options",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "question_source",
        union: false,
        fields: &[
            Field {
                name: "answered_by",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "batch_size",
                shape: Shape::Scalar(Scalar::Integer),
            },
            Field {
                name: "origin",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "ranked",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "images",
                shape: Shape::Array("image"),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "input_file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "members",
                shape: Shape::Array("named_ranked_member"),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "question_name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "threshold",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "ranked_member_result",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "images",
                shape: Shape::Array("image"),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "threshold",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "recognition_probabilities",
        union: false,
        fields: &[
            Field {
                name: "names",
                shape: Shape::Array("entity_probabilities"),
            },
            Field {
                name: "pairs",
                shape: Shape::Array("relation_probabilities"),
            },
            Field {
                name: "pieces",
                shape: Shape::Array("token_probabilities"),
            },
            Field {
                name: "proposals",
                shape: Shape::Array("boundary_reading"),
            },
        ],
    },
    Projection {
        name: "recognition_question",
        union: false,
        fields: &[
            Field {
                name: "batch",
                shape: Shape::Object("batch_setting"),
            },
            Field {
                name: "context_schema",
                shape: Shape::Object("input_declaration"),
            },
            Field {
                name: "entity_definition",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "instructions",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "item_schema",
                shape: Shape::Object("input_declaration"),
            },
            Field {
                name: "kinds",
                shape: Shape::Mapping("kind_description"),
            },
            Field {
                name: "label_details",
                shape: Shape::Array("label"),
            },
            Field {
                name: "mode",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "model",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "on",
                shape: Shape::Array("recognition_question_on"),
            },
            Field {
                name: "profile",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "relation_threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "relations",
                shape: Shape::Array("relation_rule"),
            },
            Field {
                name: "snippet_pieces",
                shape: Shape::Scalar(Scalar::Integer),
            },
            Field {
                name: "stage_context",
                shape: Shape::Object("recognition_stage_context"),
            },
            Field {
                name: "threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "verb",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "wording_version",
                shape: Shape::Scalar(Scalar::Integer),
            },
        ],
    },
    Projection {
        name: "recognition_question_on",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "recognition_stage_context",
        union: false,
        fields: &[
            Field {
                name: "boundary",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "kind_edge",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "relation",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "recognized",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("recognition_probabilities"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "question",
                shape: Shape::Object("recognition_question"),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "value",
                shape: Shape::Object("recognized_value"),
            },
        ],
    },
    Projection {
        name: "recognized_value",
        union: false,
        fields: &[
            Field {
                name: "entities",
                shape: Shape::Array("entity"),
            },
            Field {
                name: "mode",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "proposals",
                shape: Shape::Array("boundary_proposal"),
            },
            Field {
                name: "relations",
                shape: Shape::Array("entity_edge"),
            },
        ],
    },
    Projection {
        name: "record_order",
        union: false,
        fields: &[
            Field {
                name: "ordinal",
                shape: Shape::Scalar(Scalar::Integer),
            },
            Field {
                name: "result_index",
                shape: Shape::Scalar(Scalar::Integer),
            },
            Field {
                name: "completed",
                shape: Shape::Scalar(Scalar::Boolean),
            },
        ],
    },
    Projection {
        name: "related",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("relation_answers"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "input_sources",
                shape: Shape::Array("indexed_source"),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "question",
                shape: Shape::Object("relation_question"),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "value",
                shape: Shape::Array("located_edge"),
            },
        ],
    },
    Projection {
        name: "related_entity",
        union: false,
        fields: &[
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "relation_answers",
        union: false,
        fields: &[Field {
            name: "questions",
            shape: Shape::Array("relation_reading"),
        }],
    },
    Projection {
        name: "relation_endpoint",
        union: false,
        fields: &[
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "relation_fields",
        union: false,
        fields: &[
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "relation_probabilities",
        union: false,
        fields: &[
            Field {
                name: "probability",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "relation",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("place"),
            },
            Field {
                name: "target",
                shape: Shape::Object("place"),
            },
        ],
    },
    Projection {
        name: "relation_question",
        union: false,
        fields: &[
            Field {
                name: "batch",
                shape: Shape::Object("batch_setting"),
            },
            Field {
                name: "context_schema",
                shape: Shape::Object("input_declaration"),
            },
            Field {
                name: "fields",
                shape: Shape::Object("relation_fields"),
            },
            Field {
                name: "item_schema",
                shape: Shape::Object("input_declaration"),
            },
            Field {
                name: "label_details",
                shape: Shape::Array("label"),
            },
            Field {
                name: "model",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "on",
                shape: Shape::Array("relation_question_on"),
            },
            Field {
                name: "profile",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "relations",
                shape: Shape::Array("relation_rule"),
            },
            Field {
                name: "threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "verb",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "wording_version",
                shape: Shape::Scalar(Scalar::Integer),
            },
        ],
    },
    Projection {
        name: "relation_question_on",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "relation_reading",
        union: false,
        fields: &[
            Field {
                name: "accepted",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "answer",
                shape: Shape::Object("answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "direction",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "failure",
                shape: Shape::Object("failure"),
            },
            Field {
                name: "failure_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "method",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "observations",
                shape: Shape::Array("observation"),
            },
            Field {
                name: "probability",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "question_sources",
                shape: Shape::Array("question_source"),
            },
            Field {
                name: "reads",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "relation",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "request",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("related_entity"),
            },
            Field {
                name: "target",
                shape: Shape::Object("relation_endpoint"),
            },
            Field {
                name: "threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "usage",
                shape: Shape::Object("token_usage"),
            },
        ],
    },
    Projection {
        name: "relation_rule",
        union: false,
        fields: &[
            Field {
                name: "either",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "reads",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "single",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "source",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "target",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "reported_usage",
        union: false,
        fields: &[
            Field {
                name: "input_tokens",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "output_tokens",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "score",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "images",
                shape: Shape::Array("image"),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "input_file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "question_name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "threshold",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "value",
                shape: Shape::Scalar(Scalar::Real),
            },
        ],
    },
    Projection {
        name: "selected_span",
        union: false,
        fields: &[
            Field {
                name: "end",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "start",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "send_budget_denial",
        union: false,
        fields: &[
            Field {
                name: "kind",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_status",
                shape: Shape::Scalar(Scalar::Integer),
            },
        ],
    },
    Projection {
        name: "source_location",
        union: false,
        fields: &[
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "stopped",
        union: false,
        fields: &[
            Field {
                name: "at",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "cause",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "retryable",
                shape: Shape::Scalar(Scalar::Boolean),
            },
            Field {
                name: "status",
                shape: Shape::Scalar(Scalar::Integer),
            },
        ],
    },
    Projection {
        name: "string_declaration",
        union: false,
        fields: &[Field {
            name: "type",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "tags",
        union: false,
        fields: &[
            Field {
                name: "answer",
                shape: Shape::Object("answer"),
            },
            Field {
                name: "answer_id",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "first_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "images",
                shape: Shape::Array("image"),
            },
            Field {
                name: "index",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "input",
                shape: Shape::Scalar(Scalar::Data),
            },
            Field {
                name: "input_file",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "last_line",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "meta",
                shape: Shape::Object("metadata"),
            },
            Field {
                name: "position",
                shape: Shape::Object("position"),
            },
            Field {
                name: "question",
                shape: Shape::Object("question"),
            },
            Field {
                name: "question_name",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "schema",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "source",
                shape: Shape::Object("source_location"),
            },
            Field {
                name: "threshold",
                shape: Shape::Object("threshold"),
            },
            Field {
                name: "value",
                shape: Shape::Array("tags_value"),
            },
        ],
    },
    Projection {
        name: "tags_value",
        union: true,
        fields: &[Field {
            name: "value",
            shape: Shape::Scalar(Scalar::Text),
        }],
    },
    Projection {
        name: "threshold",
        union: true,
        fields: &[
            Field {
                name: "number_value",
                shape: Shape::Scalar(Scalar::Real),
            },
            Field {
                name: "string_value",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
    Projection {
        name: "token_band",
        union: false,
        fields: &[
            Field {
                name: "lower",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "upper",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "token_probabilities",
        union: false,
        fields: &[
            Field {
                name: "end",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "start",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "tags",
                shape: Shape::Mapping("named_probability"),
            },
        ],
    },
    Projection {
        name: "token_usage",
        union: false,
        fields: &[
            Field {
                name: "input_tokens",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
            Field {
                name: "output_tokens",
                shape: Shape::Scalar(Scalar::Unsigned),
            },
        ],
    },
    Projection {
        name: "usage_persistence",
        union: false,
        fields: &[
            Field {
                name: "advice",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "observed_at",
                shape: Shape::Scalar(Scalar::Text),
            },
            Field {
                name: "state",
                shape: Shape::Scalar(Scalar::Text),
            },
        ],
    },
];
pub(crate) const COLUMN_COUNT: usize = 1275;
