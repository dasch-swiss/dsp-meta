# API Documentation

The pubic API of the DSP Metadata Browser can be accessed under `https://meta.dasch.swiss/api`.

## Endpoints

Get all projects with paging: `GET /v1/projects`
The optional query params `_limit` (default value `100`) and `_page` (default value `1`) allow you to define the page.
The response contains the `x-total-count` header with the total amount of projects.

Get a project by ID: `GET /v1/projects/{shortcode}`

!!! info

    In the future, OpenAPI documentation may be provided.
