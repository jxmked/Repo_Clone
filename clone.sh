#!/bin/bash

# Developed by Jovan De Guia.
# License Under MIT License

#Downloaded files will go here
_FOLDER='/storage/emulated/0/@webpage/git clone'

_DATAFOLDER=".xio"

TOKEN="<Your Github Token else cloning with data and private repo will not work>"

#Temporary folder
TMP="tmp"

USERNAME=""
REPO=""

# ----------------------------------------
# Do we have a parameter?

if [[ ! $1 ]]; then
    echo "No parameter found"
    exit 1
fi

# ----------------------------------------
# Get URL Index from arguments
function getURLIndex(){
    local args=$@
    local index=0
    
    # Any Valid URL
    # https://stackoverflow.com/questions/3183444/check-for-valid-link-url
    #local regex='(https?|ftp|file)://[-[:alnum:]\+&@#/%?=~_|!:,.;]*[-[:alnum:]\+&@#/%=~_|]'
    
    # Github URL ~ Modified Slightly
    # https://serverfault.com/questions/417241/extract-repository-name-from-github-url-in-bash
    local regex='^(https|git)(:\/\/|@)?(github)([^\/:]+)[\/:]([^\/:]+)\/(.+)(.git)?$'
    
    for i in ${args[@]}; do
        if [[ "${i}" =~ $regex ]]; then
            echo $index
            break
        fi
        ((index++))
    done
}

# ----------------------------------------
# Re-create parameters
params=()

for arg in ${@}; do
    params+=($arg)
done
# ----------------------------------------
# Get Github URL index
index=$(getURLIndex ${params[@]}) # Index

if [[ ! $index ]]; then
    echo "No Valid Github URL Found"
    exit 1
fi

_REPO=("${params[$index]}") # REPO

# ----------------------------------------
# Pop valid url from Parameters
new_arr=()
for val in ${params[@]}; do
    if [[ ! "${val}" == "${_REPO}" ]]; then
        new_arr+=($val)
    fi
done

_ARGS=( "${new_arr[@]}" )
unset new_arr

# I Re-create the array to make sure their is no gap

# ----------------------------------------

# Build the function if have a valid Url
# For Faster Performance

function getFromArgs(){
    # @param 1st index -> Target
    # @param[] 2nd... index -> Search from array
    local KEY=$1
    set -- "${@:2}" # Remove First Index
    
    local found=0
    
    for V in ${@}; do
        if [[ "${V}" == "${KEY}" ]]; then
            found=1
            break
        fi
    done
    echo $found
}

# ----------------------------------------
# Get Flags if exists and change state

IS_PRIVATE=0
WITH_DATA=0

if [[ $_ARGS ]]; then
    # If We Need Tk Clone Private Repository
    # We need its `.git` data. It is required
    IS_PRIVATE=$(getFromArgs '-p' ${_ARGS[@]})
    
    # Need `.git` folder?
    WITH_DATA=$(getFromArgs '-w' ${_ARGS[@]})
fi

# ----------------------------------------

function createDir(){
    #Create if when not exists
    if [[ ! -d "${1}" ]]; then
        mkdir "${1}"
    fi
}

function onError(){
    echo -e "\e[1;31mSomething went wrong\e[0m"
    echo "Exiting..."
    exit 1 #Terminate
}

function checkExistence(){
    if [[ -d "${1}/${2} (${3})" ]]; then
        echo -e "\e[1;31mFailed: ${RES} does exists on local machine.\e[0m"
        echo "Exiting..."
        exit 1 #Terminate
    fi
}

BRANCH="master" #Default
TR=""

if [[ $(echo $_REPO | cut -d'/' -f 1) == 'https:' ]]; then 
    USERNAME=$(echo $_REPO | cut -d'/' -f 4)
    REPO=$(echo $_REPO | cut -d'/' -f 5)
    
    TR=$(echo $_REPO | cut -d'/' -f 6)
    
    #Check for branch
    if [[ $TR == "tree" ]]; then
        
        cn=-1
        IFS='/' read -ra TMPB <<< "$_REPO"
        
        for i in "${TMPB[@]}"; do
            cn=$(($cn + ${#i} + 1))
            if [[ $TR == $i ]]; then
                BRANCH=${_REPO:($cn + 1)}
                break
            fi
        done
    fi
else
    USERNAME=$(echo $_REPO | cut -d'/' -f 1)
    REPO=$(echo $_REPO | cut -d'/' -f 2)
fi

RES="${USERNAME}/${REPO}"

MM='Cloning...'

#Check for existence of Github Repo
if ! curl --output /dev/null --silent --head --fail "https://github.com/${RES}"; then
    #If were cloning private repository, the code above will fail
    if [[ $WITH_DATA -eq 0 && $IS_PRIVATE -eq 0 ]]; then
        echo -e "\e[1;31mFailed: https://github.com/${RES} does not exists.\e[0m"
        echo "Maybe it is a private repository."
        echo "Use 'clone.sh %url% -w -p' to clone" 
        echo "Exiting..."
        exit 1 #Terminate
    fi
    MM='Cloning private repository...'
fi

echo $MM
printf "\n"

cd "${_FOLDER}"
createDir "${_FOLDER}/${_DATAFOLDER}"
createDir "${USERNAME}"

cd "${_FOLDER}/${USERNAME}"
cd "${_FOLDER}"

#Check if folder existing on local Machine
checkExistence $_FOLDER $RES $BRANCH

#Output folder under username
OUTPUT="${REPO} (${BRANCH})"

createDir "${_FOLDER}/${_DATAFOLDER}/${TMP}"
createDir "${_FOLDER}/${USERNAME}/${OUTPUT}"

#Download files to tmp folder
cd "${_FOLDER}/${_DATAFOLDER}/${TMP}"
    
#Just enter anything from param 2 to clone with data
if [[ $WITH_DATA -eq 1 ]]; then
    #When cloning with data
    if [[ $TR == 'tree' ]]; then
        git clone --single-branch --branch $BRANCH https://${TOKEN}@github.com/$RES.git || { onError 1; }
    else
        #Without defined branch
        git clone "https://${TOKEN}@${_REPO:8}.git" || { onError 1; }
    fi
    
    MSG="\e[1;32mCloned with data\e[0m"
else
    #Download Repo without .git folder
    
    #Init download
    curl -L "https://github.com/${RES}/tarball/${BRANCH}" | tar xz || { onError 1; }
    
    MSG="\e[1;32mCloned without data\e[0m"
fi

#Move downloaded folder to...
F="${_FOLDER}/${_DATAFOLDER}/${TMP}/$(ls)"
    
if [[ ! -d "${F}" ]]; then
    onError 1
fi

#desired location
mv -T "${F}" "${_FOLDER}/${USERNAME}/${OUTPUT}" || {
    echo -e "\e[1;31mCheck tmp folder\e[0m"
    onError 1; 
}

echo -e $MSG


#Lets create Timestamp
#User > Branch > Repository
echo "$(date) | $USERNAME > $BRANCH > $REPO" >> "$_FOLDER/${_DATAFOLDER}/log.txt"


echo

#printf "Downloaded content size: "
#du -hs "${_FOLDER}/${RES}"

echo $USERNAME" > "$REPO

echo "Directory: ${_FOLDER}/${USERNAME}/${OUTPUT}"

printf "\n"

# Developed with Love and Frustration by Jovan De Guia
# License under MIT License
# Github Username: jxmked
#Model:SH-CSA-0005 - Advance Repository Cloner API