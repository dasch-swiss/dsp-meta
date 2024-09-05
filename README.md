# DaSCH Service Platform Metadata Repository

## How should I write my commits?

We are using [Conventional Commit messages](https://www.conventionalcommits.org/).

The most important prefixes you should have in mind are:

- `fix:` which represents bug fixes, and correlates to a [SemVer](https://semver.org/)
  patch.
- `feat:` which represents a new feature, and correlates to a SemVer minor.
- `feat!:`, or `fix!:`, `refactor!:`, etc., which represent a breaking change
  (indicated by the `!`) and will result in a SemVer major.

Here is the complete list of prefixes used in this repository with
corresponding sections in the changelog:

- 'build' -> section: 'Maintenances'
- 'chore' -> section: 'Maintenances'
- 'data' -> section: 'Project Data'
- 'docs' -> section: 'Documentation'
- 'feat' -> section: 'Enhancements'
- 'fix' -> section: 'Bug Fixes'
- 'perf' -> section: 'Performance Improvements'
- 'refactor' -> section: 'Maintenances'
- 'test' -> section: 'Tests'
- 'style' -> section: 'Styles'

## 1 Usage

```
Flags:
- `-h, --help`
- `-v, --verbose`

Sub-commands:
- `dsp-meta verify [file]`
  - [file]: provide the path to the `.toml` file
- `dsp-meta convert`
```

## HCL (HashiCorp Configuration Language) General Syntax

The Metadata Domain Specific Configuration syntax is based on the HCL (HashiCorp Configuration Language) syntax,
with a number of predefined domain specific named blocks. The following is a short introduction to the general syntax.

### Comments

Comments in HCL begin with the "#" symbol and can be placed on their own line or at the end of a line. They are used to
provide explanations or add context to the configuration.

```hcl
# This is a comment in HCL
key = "value" # This is another comment
```

### Blocks

HCL organizes configuration into blocks. Each block starts with the block type followed by a set of braces ({}) that
enclose the block's contents. Blocks can be nested to represent hierarchical structures.

```hcl
block_type {
  key = "value"
  nested_block_type {
    nested_key = "nested_value"
  }
}
```

### Multiple Blocks

HCL allows you to define multiple blocks of the same type within a configuration. Each block instance is separated by a newline.

```hcl
block_type {
  key = "value"
}

block_type {
  key = "another_value"
}
```

### Block Labels

A block has a type (`resource` in this example). Each block type defines how many labels must follow the type keyword.
The resource block type expects two labels, which are `aws_instance` and `example` in the example below. A particular
block type may have any number of required labels, or it may require none as with the nested network_interface block
type.

```hcl
resource "aws_instance" "example" {
  ami = "abc123"

  network_interface {
    # ...
  }
}
```

### Attributes

Inside a block, you define attributes using the key-value syntax. The key and value are separated by an equal sign (`=`).
Attributes define the properties or settings associated with the block.

```hcl
block_type {
  key = "value"
}
```

### Strings

HCL supports string values, which are enclosed in double quotes ("). Strings can contain alphanumeric characters,
symbols, and spaces. To include a double quote within a string, you can escape it with a backslash (`\`).

```hcl
message = "Hello, World!"
path = "/path/to/file"
escaped_string = "This string contains \"quotes\"."
```

### Numbers

HCL supports both integers and floating-point numbers. Numeric values are written without quotes.

```hcl
count = 10
pi = 3.14159
```

### Booleans

Boolean values are represented as either "true" or "false". They are not enclosed in quotes.

```hcl
enabled = true
debug = false
```

### Lists

HCL supports lists, which are represented by square brackets ([]). Elements in a list are separated by commas. Lists
can contain values of different types.

```hcl
fruits = ["apple", "banana", "orange"]
numbers = [1, 2, 3, 4, 5]
```

### Heredoc Syntax

HCL supports heredoc syntax for multiline strings. It is enclosed in triple double quotes ("""). Heredocs preserve
leading indentation and line breaks.

```hcl
multiline = """
This is a multiline string.
It can span multiple lines.
"""
```

## Metadata HCL: Domain Specific Configuration Language for DSP Metadata

### Style Conventions

- All block types are lowercase
- Indent two spaces for each level of nesting
- When multiple arguments with single-line values appear on consecutive lines, align their equals signs (=)
- When both arguments and blocks appear together inside a block body, place all of the arguments together at the top
  and then place nested blocks below them. Use one blank line to separate the arguments from the blocks.

### Project

The project block contains the project specific attributes and blocks.

Example:

```hcl
project {
  created_at = 1637624150548721000
  created_by = "dsp-metadata-gui"
  shortcode  = "0803"
  name       = "Project name"

  alternative_name {
    en = "Alternative name"
  }

  teaser_text = "Teaser text"

  description {
    de = "Description in German."
    en = "Description in English."
    fr = "Description in French."
  }

  url {
    href = "https://example.com"
    label = "Example label of the URL"
  }

  how_to_cite = "How to cite the project"
  start_date = "2008-06-01"
  end_date   = "2012-08-31"

  keyword {
    de = "Basel"
  }

  discipline snf {
    ref_id = "10404"
    description = "Visual arts and Art history"
    url = "https://www.snf.ch/SiteCollectionDocuments/allg_disziplinenliste.pdf"
  }
  discipline skos {
    ref_id = "https://skos.um.es/unesco6/5501"
    description = "Local history"
    url = "https://skos.um.es/unesco6/5501"
  }

  spatial_coverage geonames {
    ref_id = "https://www.geonames.org/2661552"
    description = "Bern"
    url = "https://www.geonames.org/2661552"
  }

  temporal_coverage periodo {
    ref_id = "https://n2t.net/ark:/99152/p06c6g3pvr5"
    description = "Under Mediation act, 1803-1814"
    url = "https://n2t.net/ark:/99152/p06c6g3pvr5"
  }
  temporal_coverage chronontology {
    ref_id = "https://chronontology.dainst.org/period/kqORhO4TGm4n"
    description = "20th Century (1900 - 1999)"
    url = "https://chronontology.dainst.org/period/kqORhO4TGm4n"
  }
  temporal_coverage text {
    de = "1766-1905"
    en = "1766-1905"
    fr = "1766-1905"
  }

  publication {
    text = "Citation of the publication"
  }
}
```

### Dataset

```hcl
dataset {
  # Dataset specific attributes and blocks
}
```

### Person

```hcl
person {
  # Person specific attributes and blocks
}
```

### Organization

```hcl
organization {
  # Organization specific attributes and blocks
}
```

### Grant

```hcl
grant {
  # Grant specific attributes and blocks
}
```
