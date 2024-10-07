# gherkin-formatter

> [!WARNING]
>
> This project is a work in progress. Features may be incomplete, and changes are ongoing. Use at your own risk!

A stand alone binary that replicates how WebStorm formats `.feature` files.

### Behaviour

- Whitespace at the end of the file is not changed. This includes the presence of a trailing newline.
- Indentation is 2 spaces.
- Comments are completely unchanged.
- Tags are indented to the correct level, but their location isn't changed.
