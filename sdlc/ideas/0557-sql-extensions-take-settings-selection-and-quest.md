# SQL extensions take settings selection and question-kind checks from the core

Status: open.

Kind: idea
When: after 0.2.0 ships, or when a SQL extension's settings or question-kind rules drift from the core

## Problem

SQL extensions take settings selection and question-kind checks from the core. removes the SQLite and PostgreSQL copies of settings selection and question-kind checks, such as databases/postgresql/src/forms.rs, that 0551 kept.
