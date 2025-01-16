# [Repo Clone](https://jxmked.github.io/Repo_Clone/)

### About

Clone Github repository in a manageable way

### Description

This program clone repository and store them in a managable way.
It categorize the repositories you cloned by their user, repository and branch.
It also logs your cloned repository so you can see when or track the repositories you have cloned.

### Requirements

- `git` CLI must be installed on machine
- Token is required when cloning your own private repository
- Internet connection required

### Setup

1. Download the latest release binary from [Releases](https://github.com/jxmked/Repo_Clone/releases) and place it where you want and copy its folder path.
2. Setup your environment variables by inserting the path of your binary into variable `Path` at `System`
3. Create your folder where you want to place your cloned repository.
4. Run `x-clone set output_folder <path/where/your/repo/goes>`
5. This is optional if you just want to clone public repositories. Set token by `x-clone set token <GIT_TOKEN>`

**_Everything is set_**

### Usage

`x-clone https://github.com/jxmked/Repo_Clone`

- use `-w` to clone with remote data
- use `-p` to pull/update your existing local repository

### Parameters

| Commands |                               Basic Description                                | is Required |
| :------: | :----------------------------------------------------------------------------: | :---------: |
|  %URl%   |                           Github repository to clone                           |  Required   |
|    -w    |                     Clone with `.git` folder for later use                     |  Optional   |
|    -p    | Pull (Update) Existing Repository in your local machine from Github Repository |  Optional   |

#### **Created with love and frustration** by [Jovan](https://facebook.com/deguia25)
