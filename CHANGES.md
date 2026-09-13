# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased - 2026-09-13
### Changed
- Validate all social handles with upstream-specific constraints and 1–100
  fallback length bounds. Replace `From` conversions with fallible `TryFrom`.
- Normalize displayed `@` prefixes and lowercase-only usernames; preserve
  spelling for case-preserving upstreams while comparing, ordering, and hashing
  case-insensitively.
- Validate LinkedIn handles after strict UTF-8 percent-decoding.
- Apply handle validation to Serde, GraphQL scalars/cursors, and SQLx decoding.

## 0.1.4 - 2026-09-13

## 0.1.3 - 2026-09-12

## 0.1.2 - 2026-09-12

## 0.1.1 - 2026-09-12

## 0.1.0 - 2025-05-09

## 0.0.0 - 2025-05-08
