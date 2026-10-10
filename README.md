# tellme

A simple command-line tool that translates a word and explains its meaning — definitions, part of speech, and synonyms — in one call.

## Usage

```bash
tellme -w <word> -t <lang>
```

## Example

```bash
$ tellme -w resilient -t uk
```

**Translation (uk):** стійкий
**Definition:** able to recover quickly from difficulties; tough.

## Features

- Translate any word into a selected language.
- Pulls definitions, part of speech, and example usage.
- Suggests synonyms for richer vocabulary.
- Stores all searched words with the context for learning practice.

## Install

```bash
cargo install tellme
```
