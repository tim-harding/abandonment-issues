# Abandonment Issues

Is your hard drive bursting with half-finished dev projects that you abandoned for the next shiny idea you'll _definitely_ complete this time? This tool will soothe your shame by finding and deleting old build and package files that take up space.

## Install

Download a [release](releases/latest) or install with Cargo:

```
cargo install abandonment-issues
```

## Develop

Build and run the project with `cargo run`. The tool currently only supports NPM and Cargo projects, so feel free to fork and submit a PR if you add more languages :3

## Usage

Specify directories to search for projects in using `-d`. Multiple can be listed. Use `--depth` to limit the search depth and `--recent` to specify how many days a project must have sat untouched to be removed. For example, to remove projects more than a month stagnant, use the following:

```
abandonment-issues --directory ~/projects --depth 5 --recent 30
```
