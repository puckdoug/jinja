# jinja - a command-line jinja2 template processor

`jinja` provides a minimal a CLI wrapper around [minijijna](https://docs.rs/minijinja/latest/minijinja/) to allow processing jinja templates from the command-line. This allows simple substitutions like

```shell
MYVAR="foo" jijna < template.j2 > newfile
```

The goal is to make it simple to do quick substitutions from the command-line or in shell scripts as needed.

## Command-line syntax

```
jinja [OPTIONS] <TEMPLATES>...

ARGUMENTS:
  <TEMPLATES>...  Template file(s) to process

OPTIONS:
  --input <INPUT>              Input file (defaults to stdin)
  --output <OUTPUT>            Output file (defaults to stdout)
  --substitutions <FILE>       Substitutions file (KEY=value format)
  --verbose                    Enable verbose output
  --debug                      Enable debug mode
  -h, --help                   Print help
  -V, --version                Print version
```

## Examples

Process a template with environment variables:
```shell
FOO=bar jinja template.j2 < input.txt
# or piped:
echo 'Hello {{ FOO }}!' | FOO=bar jinja template.j2
```

Process a template with a substitutions file:
```shell
# vars.txt contains:
# NAME=World
# AGE=30
jinja template.j2 --substitutions vars.txt < input.txt
```

Write output to a file:
```shell
echo 'Template content' | jinja template.j2 --output result.txt
```

Use --input and --output:
```shell
jinja template.j2 --input template.txt --output result.txt
```
