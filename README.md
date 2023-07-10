# DaSCH-Software-Platform-Meta
DaSCH Software Platform Meta Repository

## 1 Usage

Flags:
- `-h, --help`
- `-v, --verbose`

Sub-commands:
- `dsp-meta verify [file]`
  - [file]: provide the path to the `.toml` file
- `dsp-meta convert`





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
Inside a block, you define attributes using the key-value syntax. The key and value are separated by an equal sign (=).
Attributes define the properties or settings associated with the block.

```hcl
block_type {
  key = "value"
}
```

### Strings
HCL supports string values, which are enclosed in double quotes ("). Strings can contain alphanumeric characters,
symbols, and spaces. To include a double quote within a string, you can escape it with a backslash (\).

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

### Maps
Maps in HCL are similar to dictionaries or key-value pairs in other languages. They are represented by curly braces ({})
and consist of key-value pairs. Keys are written without quotes.

```hcl
person = {
  name = "John Doe"
  age = 30
  email = "john@example.com"
}
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

### Variable Interpolation
HCL allows variable interpolation, where the value of a variable can be inserted into a string using the "${}" syntax.
This enables dynamic configurations.

```hcl
name = "John Doe"
greeting = "Hello, ${name}!"
```

### Expressions
HCL supports expressions that can be used to perform computations or manipulate values within the configuration.
Expressions are enclosed in parentheses.

```hcl
result = (4 + 2) * 3
```

### Conditional Statements
HCL supports conditional statements for making decisions within the configuration. Conditions are written using the
"if" and "else" keywords.

```hcl
if condition {
  # Code to execute if the condition is true
} else {
  # Code to execute if the condition is false
}
```

## Metadata HCL: Domain Specific Configuration Language for DSP Metadata

### Style Conventions
- All block types are lowercase
- Indent two spaces for each level of nesting
- When multiple arguments with single-line values appear on consecutive lines, align their equals signs (=)
- When both arguments and blocks appear together inside a block body, place all of the arguments together at the top
and then place nested blocks below them. Use one blank line to separate the arguments from the blocks.


### Project

The project block type takes one required label, which is the project identifier. The project identifier is a 4 digit
hexadecimal number. The project block type can contain the following attributes:

```hcl
project "0000" {
  # Project specific attributes and blocks
}
```

### Dataset

```hcl
dataset "0804:dataset-001" {
  # Dataset specific attributes and blocks
}
```


### Person

```hcl
person "0803:lothar_schmitt" {
  # Person specific attributes and blocks
}
```

### Organization

```hcl
organization "0803:lothar_schmitt" {
  # Organization specific attributes and blocks
}
```

### Grant

```hcl
grant "0803:lothar_schmitt" {
  # Grant specific attributes and blocks
}
```
