# pom-cli

A simple program to lock your screen and leave a desktop notification <message> after some number of <minutes>

## Setup

`git clone git@github.com:Melvillian/pom-cli.git`

## Running

`cargo run <message> <minutes> &`

For example:

`cargo run "go to standup meeting" 30 &`


> Note: The `&` above will allow the `pom-cli` process to run in the background on all Unix-like systems (mac & Linux). If you are working on Windows, I would appreciate a PR telling me how to run Windows processes in the background!



