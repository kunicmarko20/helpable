Helpable
========

https://github.com/kunicmarko20/lendabot converted into a CLI tool.


```
USAGE:
    helpable <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    approve           Approve pull request
    config            Set or view config values
    merge             Merge pull request
    release           Create a Release pull request
    search            Search pull requests by term and open chosen one in default browser
    sha               Newest Commit Sha
    update-release    Update Release pull request name
```

## Config

By running `helpable config list`, you are able to see list of current configs,
you will see something like:

```
github_access_token:1234567890
repository_name:kunicmarko20/helpable
development_branch:development
release_branch:master
ticket_prefix:TEST
```

Most of these values are mandatory. If a value is missing and you run a command,
you will be asked to provide it, but only for the first time, after that it will
be saved for future.

## Commands

* [approve](#approve)
* config
    * [list](#list)
    * [set](#set)
* [merge](#merge)
* [release](#release)
* [search](#search)
* [sha](#sha)
* [update-release](#update-release)


### approve
```
Approve a pull request

USAGE:
    helpable approve [pull-request-number]

ARGS:
    <pull-request-number>    Number of the pull request to update
```

If `pull-request-number` is not provided, it will show a list of all pull requests,
and allow you to choose one.

### config

#### list

```
Show all current config values

USAGE:
    helpable config list
```

#### set

```
Set or replace a config value

USAGE:
    helpable config set <key> <value>

ARGS:
    <key>      Name of the config that is modified
    <value>    New value of the config
```

### merge

Merge is executed based on some rules, lets go over terms.

Release pull request - If base branch equals `release_branch` value from config and
head equals `development_branch`.

Back-merge pull request - If base branch equals `development_branch` value from config and
head equals `release_branch`.

If a pull request is either Release or a Back-merge, merge method will be `merge` to preserve
history, else merge method will be `squash`.

```
Merge a pull request

USAGE:
    helpable merge [pull-request-number]

ARGS:
    <pull-request-number>    Number of the pull request to update
```

If `pull-request-number` is not provided, it will show a list of all pull requests,
and allow you to choose one.

### release

A Release pull request - If base branch equals `release_branch` value from config and
head equals `development_branch`.

```
Create a Release pull request

USAGE:
    helpable release
```

### search

```
Search pull requests by term and open chosen one in default browser

USAGE:
    helpable search <term>

ARGS:
    <term>    Term to search for
```

### sha

```
Get newest Commit Sha

USAGE:
    helpable sha [branch]

ARGS:
    <branch>    Branch to fetch the latest sha from
```

If `branch` is not provided, it will use the `release_branch` value from config.

### update-release

Updates the Release Pull Request title with all ticket numbers from commits,
based on `ticket_prefix` value from config. (This happens automatically when you run [release](#release) command)

This command assumes you are following `[TEST-123] My new feature` commit naming, where `TEST`
is `ticket_prefix`.

```
Update Release pull request name

USAGE:
    helpable update-release [pull-request-number]

ARGS:
    <pull-request-number>    Number of the pull request to update
```

If `pull-request-number` is not provided, it will show a list of all pull requests,
and allow you to choose one.
