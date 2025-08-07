# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with data in this repository.

## Data Structure Overview

The `data/` directory contains metadata for DSP (DaSCH Service Platform) research projects. All project metadata is stored as JSON files in the `json/` subdirectory.

## File Organization

- **json/**: Contains 65+ project metadata files, each representing a research project
- Each JSON file is named with a project identifier (e.g., `beol.json`, `dasch.json`, `nietzsche.json`)
- Files follow a consistent JSON schema defined in `resources/schema-metadata-final.json`

## JSON Schema Structure

Each project metadata file contains:

- **$schema**: Reference to the metadata schema (usually `../../resources/schema-metadata-draft.json`)
- **project**: Core project information including:
  - `__type`: Always "Project"
  - `howToCite`: Citation format for the project
  - `teaserText`: Brief project description
  - `description`: Detailed project description (multilingual support with language codes)
  - `alternativeNames`: Alternative project names (multilingual)
  - `keywords`: Project keywords (multilingual)
  - `disciplines`: Academic disciplines
  - `datasets`: Array of dataset URIs
  - `funders`: Array of funder organization URIs
  - `grants`: Array of grant URIs
  - Additional metadata fields for spatial/temporal coverage, publications, etc.

## Data Management Practices

### Adding New Project Metadata
1. Create a new JSON file in `data/json/` with appropriate project identifier
2. Ensure the file follows the JSON schema in `resources/schema-metadata-draft.json`
3. Use `just` commands to validate: the validator tool can check schema compliance
4. Follow existing naming conventions and structure patterns

### Updating Existing Metadata
- Always validate changes against the schema
- Maintain multilingual support where applicable (en, de, fr language codes)
- Preserve existing URI references and relationships
- Use commit prefix `data:` for metadata changes per conventional commits

### Schema Validation
- JSON files must validate against `resources/schema-metadata-draft.json`
- The validator binary (`dsp-meta-validator`) can be used to check compliance
- Schema defines required fields, types, and structure constraints

## Common Fields and Patterns

- **Multilingual content**: Use language code objects like `{"en": "English text", "de": "German text"}`
- **URI references**: Datasets, funders, grants reference other entities via URIs
- **Date formats**: Follow ISO date formats where applicable
- **Keywords and disciplines**: Use consistent terminology across projects
- **Citations**: Follow academic citation standards in `howToCite` field

## Validation Commands

Use the validator tool to check metadata files:
```bash
# Validate a specific file
dsp-meta-validator verify data/json/project-name.json

# The main server also validates data on startup
just serve
```

## Data addition workflow

### Process

- You will be provided with textual metadata in a semi-structured format.
- You will be asked to convert this metadata into a structured JSON format.
- The JSON should conform to the schema defined in `resources/metadata_schema.json`.
- The JSON files should be placed in the `data/json/` directory with the appropriate filename as provided by the user.

### Pitfalls to avoid

- Projects have a field `status` which can be "ongoing" or "finished". This does not have an influence on the status of the metadata. **Always create the metadata as "draft" first**.
- You may be provided with metadata in multiple languages. These languages MUST be maintained. The following exeptions apply:
  - For project funding, replace "Projektförderung" with "Project Funding" in the English version.
- If only one project URL is provided, use exactly that URL and do not add a second URL.
- If the project url points to "app.dasch.swiss", use the URL text "Discover Project Data".
- If the project URL points to a different URL, use the URL text "External Project Website".

### Commit Messages

When committing changes to the metadata files, use the following commit message format:

```bash
data: Add project metadata for <project-name>
```

### Final Steps

When prompted by the user to finalize the work, follow these steps:

- Ensure all JSON files are validated against the schema.
- Ensure you're not on the `main` branch. If so, create a new branch.
- Git commit the changes with the appropriate commit message.
- Push the changes to the repository.
- Open a **draft** pull request with the title "data: Add project metadata for <project-name>".
