# [Repo Clone](https://jxmked.github.io/Repo_Clone/)

### About
Clone Github repository in a manageable way

### Features
- Clone Github repository with `.git` folder.
- Clone Github repository without `.git` folder and save internet data charges.
- Clone Github same repository from different branch
- Direct pasting copied URL from browser to CLI
- Clone your private repository using `-w` flag
- Pull (Update) using `-p` flag

### Requirements
- `git` CLI must be installed on machine.
- Internet connection required.

### Installation and Use
[Download](https://github.com/jxmked/Repo_Clone) and run `bash clone.sh <github repository url> [-w]`

### Parameters
| Commands | Basic Description | is Required |
| :---:| :---: | :---: |
| %URl% | Github repository to clone | Required |
| -w | Clone with `.git` folder for later use | Optional |
| -p | Pull (Update) Existing Repository in your local machine from Github Repository | Optional |
| -clear | Clear Temporary Folder | Optional |

## Notes:
- Github token is required to clone repository with data and private repository.
- Set your own directory inside of the `clone.sh` file
    - Must be change to already existing directory.
- `log.txt` is stored in `.xio` folder.
- Master/Main branch is default

```bash
# No .git folder or git commit history
bash clone.sh https://github.com/jxmked/Repo_Clone 
# Or
# Download with .git folder or commit history.
# It also allow to download private repository
bash clone.sh https://github.com/jxmked/Repo_Clone -w
```

### **About .git folder**
> "The `.git` folder contains all information that is necessary for the project and 
> all information relating commits, remote repository address, etc. It also contains 
> a log that stores the commit history. This log can help you to roll back to the 
> desired version of the code." - Google

#### Tested in...
- Android Termux.

#### Downloaded repositories will look like this...
- __Downloads__
    - **jxmked**
        - Repo_Clone (master)
        - C-language (master)
    
    - **Username A**
        - Repo (...)
        - Repo (...)
    
    - **Username B**
        - Repo (...)
        - Repo (...)
        - Repo (...)

### Example links can be clone...
- ```https://github.com/jxmked/Repo_Clone``` - Main branch
- ```https://github.com/jxmked/Repo_Clone/tree/gh-pages``` - gh-pages branch
- ```https://github.com/jxmked/Script-Thing``` - Main branch

### Function has been used.
- `mkdir`
- `curl`
- `git`
- `mv`

#### **Created with love and frustration** by [Jovan](https://facebook.com/deguia25)
###### Model:SH-CSA-0009 - Advance Repository Cloner API
