# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1.0] - 2026-03-29

### Added
- GitHub Actions CI workflow for API and web builds
- Docker build/push workflow for api, web, and docs containers
- Docker Compose setup for local development
- Local file reading support for sync (development mode)
- Test coverage for sync handler, template validation, and pagination

### Changed
- Container renamed to tinyiothub-marketplace
- Docker image names standardized to tinyiothub-marketplace-*

### Fixed
- Axum 0.8 path parameter syntax (`:id` to `{id}`)
- Cron expression format (5-field to 6-field with seconds)
- API Dockerfile moved to api/ directory
- HMAC webhook verification now fails fast when secret is not configured
- SSRF protection on download redirects (validates HTTPS and allowed domains)

### Removed
- Web frontend (API-only deployment)
- build-docs CI job (no documentation content)
