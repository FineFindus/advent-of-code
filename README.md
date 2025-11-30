# 🎄 Advent of Code 🎄

## Used Languages
 - 2022 - [`Rust`](https://github.com/FineFindus/advent-of-code/blob/main/2022/)
 - 2023 - [`C`](https://github.com/FineFindus/advent-of-code/blob/main/2023/)
 - 2024 - [`Rust`](https://github.com/FineFindus/advent-of-code/blob/main/2024/)

## Setup

### Installing `aoc-cli`

This repository relies on using [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) for downloading the input files. It requires a `.adventofcode.session` file in the home directory. To retrieve the session cookie, press F12 anywhere on the Advent of Code website to open the browser developer tools and copy the session cookie value under the Storage tab.

### README stars

#### 1. Creating a private leaderboard

Navigate to the leaderboard page of the desired year and click on _Private Leaderboard_. If you haven't created a leaderboard yet, do so by clicking _Create It_. Your leaderboard should be accessible at `https://adventofcode.com/{year}/leaderboard/private/view/{aoc_user_id}`.

#### 2. Set repository secrets

Go to the _Secrets_ tab in your repository settings and create the following secrets:

 - `AOC_USER_ID`: Go to [the settings](https://adventofcode.com/settings) and copy the user id. It's the number behind the `#` symbol in the first name option. Example: `3031`
 - `AOC_SESSION`: an active session. [See how to install `aoc-cli` on how to get session cookie](#installing-aoc-cli)
 - Set the workflow permission of actions to _Read and write_
