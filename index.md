### Features
- Clone Github repository with `.git` folder.
- Clone Github repository without `.git` folder and save internet data charges.
- Clone Github repository from different branch
- Direct pasting copied URL from browser to CLI

### Requirements
- `git` CLI must be installed on machine.
- Internet connection required.

### Installation and Use
[Download](https://github.com/jxmked/Repo_Clone) and run `bash clone.sh <github repository url> [-w]`

### Parameters
| Params | Commands | Basic Description | is Required |
| :---: | :---:| :---: | :---: |
| $1 | %URl% | Github repository to clone | Required |
| $2 | -w | Clone with `.git` folder for later use | Optional |

## Notes:
- Github token is required to clone repository with `.git` folder.
    -- Must replace `<TOKEN HERE>` with actual token of yours
- Directory where downloaded files will go must be change to already existing directory.
- `log.txt` is stored in `.xio`

```bash
bash clone.sh <%URL%> [-w]
```

### **About the project**
"The `.git` folder contains all information that is necessary for the project and all information relating commits, remote repository address, etc. It also contains a log that stores the commit history. This log can help you to roll back to the desired version of the code." - Google

#### Tested in...
- Android Termux. ("So, what?")

#### Downloaded folders will look like this...
- __Downloads__
    - **Username A**
        - Repo
        - Repo

    - **Username B**
        - Repo
        - Repo
        - Repo

### Example links can be clone...
- ```https://github.com/jxmked/Repo_Clone```
- ```https://github.com/jxmked/Repo_Clone/tree/gh-pages```

### Function has been used.
- `mkdir`
- `curl`
- `git`
- `mv`

#### **Created with love and frustration** by [Jovan](https://facebook/com/deguia25)
###### Model:SH-CSA-0001 - Advance Repository Cloner API