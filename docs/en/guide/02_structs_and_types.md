# Structures and Types

## Comments

Comments use similar comments to compiled languages:
```faml
// Single line comment
/* Multi-line comment */
```

## Basic Structure

The basic structure is similar to yaml, but note that identifiers are similar to programming languages, can only be combinations of underscores, letters or numbers, and only allow underscores or letters to start.

### Single Object Configuration
Use square brackets `[]` to define a single object:

```faml
[group]
field = 123
```

Corresponding JSON structure:
```json
{
  "group": { "field": 123 }
}
```

### Object Array Configuration
Use double square brackets `[[]]` to define object arrays:

```faml
[[groups]]
field1 = 111

[[groups]]
field2 = 111
```

Corresponding JSON structure:
```json
{
  "groups": [
    { "field1": 111 },
    { "field2": 111 }
  ]
}
```

## Core Features

### Conditional Judgment

Use the `@if` directive to control whether fields take effect based on conditions:

```faml
[group]
value = 12

@if value == 12
conditional_field = "this field is active"

@if value != 12
inactive_field = "this field is not active"
```

When `value` equals 12, the generated JSON structure:
```json
{
  "group": {
    "value": 12,
    "conditional_field": "this field is active"
  }
}
```

### Referencing Other Configuration Items

You can directly reference other fields in the same configuration file:

```faml
[group]
base_value = 10
derived_value = base_value + 5  // 15
```

Corresponding JSON structure:
```json
{
  "group": {
    "base_value": 10,
    "derived_value": 15
  }
}
```

## Data Types

Data types have great expansion compared to yaml:

### Basic Types

```faml
[group]
bool_field = true
int_field = 123
float_field = 123.456
str_field = "hello world"
```

Corresponding JSON structure:
```json
{
  "group": {
    "bool_field": true,
    "int_field": 123,
    "float_field": 123.456,
    "str_field": "hello world"
  }
}
```

### Complex Types

```faml
[group]
// Array type
array_field = [1, 2, 3, 4, 5]

// Mapping type
map_field = { foo: "bar", baz: 123 }
```

Corresponding JSON structure:
```json
{
  "group": {
    "array_field": [ 1, 2, 3, 4, 5 ],
    "map_field": { "baz": 123, "foo": "bar" }
  }
}
```

### Special Types

```faml
[group]
// Quantified number type, actual value is numeric type
quantified_float_field = 123.456 KB

// Duration type
duration_field = 123.456 seconds

// Distance type
distance_field = 123.456 meters
```

Corresponding JSON structure:
```json
{
  "group": {
    "quantified_float_field": 126418.944,
    "duration_field": "2.0576 mins",
    "distance_field": "123.456 meters"
  }
}
```

### Quantified Number Type Units

Support the following units:
- `KB` (kilobytes)
- `MB` (megabytes)
- `GB` (gigabytes)
- `TB` (terabytes)

### Duration Type Units

Support the following time units:
- `nanoseconds` (nanoseconds)
- `microseconds` (microseconds)
- `milliseconds` (milliseconds)
- `seconds` (seconds)
- `mins` (minutes)
- `hours` (hours)
- `days` (days)
- `weeks` (weeks)
- `months` (months)
- `years` (years)

### Distance Type Units

Support the following distance units:
- `nanometers` (nanometers)
- `micrometers` (micrometers)
- `millimeters` (millimeters)
- `meters` (meters)
- `kilometers` (kilometers)
- `megameters` (megameters)