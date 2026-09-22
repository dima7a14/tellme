# tellme

A simple command-line tool that translates a word and explains its meaning — definitions, part of speech, and synonyms — in one call.

## Usage

```bash
tellme <word> --to <lang>
```

## Example

```bash
$ tellme resilient --to uk
```

**Translation (uk):** стійкий

**Definition:** able to recover quickly from difficulties; tough.

**Synonyms:** tough, hardy, strong, adaptable

## Features

- Translate any word into a selected language.
- Pulls definitions, part of speech, and example usage.
- Suggests synonyms for richer vocabulary.
- Stores all searched words with the context for learning practice.

## Install

```bash
cargo install tellme
```
