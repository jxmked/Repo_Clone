#!/bin/bash

# Developed by Jovan De Guia.
# License Under MIT License

#Downloaded files will go here
# Change This Directory to your existing Working Directory
_FOLDER='/storage/emulated/0/@webpage/git clone'

_DATAFOLDER=".xio"

TOKEN="<Token Here>"

# ----------------------------------------
# Temporary Working folder
TMP="tmp"
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
    #set -- "${@:2}" # Remove First Index
    
    local found=0
    
    for V in ${_ARGS[@]}; do
        if [[ "${V}" == "${KEY}" ]]; then
            found=1
            break
        fi
    done
    echo $found
}

# ----------------------------------------
# Get Flags if exists and change state

WITH_DATA=0
PULLREQUEST=0
CLEARFS=0
if [[ $_ARGS ]]; then
    # Need `.git` folder?
    WITH_DATA=$(getFromArgs '-w')
    PULLREQUEST=$(getFromArgs '-p')
    CLEARFS=$(getFromArgs '-clear')
fi

# ----------------------------------------
if [[ ${CLEARFS} == 1 ]]; then
    echo "Clearing..."
    echo
    rm -rf "${_FOLDER}/${_DATAFOLDER}/${TMP}" || {
        echo "Maybe its clear"
        exit 0
    }
    
    echo "Cleared"
    exit 0
fi
# ----------------------------------------
# Get Username, Repository Name and Branch
BRANCH="master" #Default
TR=0
USERNAME=""
REPO=""

[[ "${_REPO}" =~ github.com/([-[:alnum:]\+&@#%?=~_|!:,.;]*\/?){2,} ]] && {
    USERNAME=$(echo $BASH_REMATCH | cut -d'/' -f 2) # Username
    REPO=$(echo $BASH_REMATCH | cut -d'/' -f 3) # Repository
    
    # Branch
    [[ "${BASH_REMATCH}" =~ tree\/([-[:alnum:]\+&@#%?=~_|!:,.;]*) ]] && {
        BRANCH=${BASH_REMATCH[1]}
        TR=1
    }
}
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
    exit 1
}

function checkExistenceExit(){
    if [[ -d "${1}" ]]; then # Existing
        if [[ $(ls -A "${1}") ]]; then  # Folder not emoty
            echo -e "\e[1;31mFailed: ${RES} does exists on local machine.\e[0m"
            echo "Use '-p' to pull (Update)"
            echo "Directory: ${1}"
            echo "Exiting..."
            exit 1
        fi
    fi
}

function checkExistencePull(){
    if [[ -d "${1}" ]]; then # Existing
        if [[ $(ls -A "${1}") ]]; then  # Folder not emoty
            git config --global --add safe.directory "${1}" # Incase
            
            cd "${1}"
            
            if [[ "${2}" == "master" ]]; then
                git pull origin || onError 1;
            else
                git pull "${2}" || onError 1; # Need Branch name
            fi
        fi
    fi
}

RES="${USERNAME}/${REPO}"

# Check for existence of Github Repo
if ! curl --output /dev/null --silent --head --fail "https://github.com/${RES}"; then
    # If were cloning private repository, the code above will fail
    if [[ $WITH_DATA -eq 0 ]]; then
        echo -e "\e[1;31mFailed: https://github.com/${RES} does not exists.\e[0m"
        echo "Maybe it is a private repository."
        echo "Use '-w' flag to clone" 
        echo "Exiting..."
        exit 1
    fi
    echo 'Cloning Private Repository...'
else
    echo "Cloning Public Repository..."
fi

printf "\n"

cd "${_FOLDER}"

# Create System Directory
createDir "${_FOLDER}/${_DATAFOLDER}"
createDir "${_FOLDER}/${_DATAFOLDER}/${TMP}"

createDir "${USERNAME}"

cd "${_FOLDER}/${USERNAME}"

OUTPUT="${REPO} (${BRANCH})" # Output Folder

if [[ ${PULLREQUEST} == 1 ]]; then
    
    checkExistencePull "${OUTPUT}" "${BRANCH}"
    
    echo
    echo "Pull Request"
    echo "$(date) | Pull | $USERNAME > $BRANCH > $REPO" >> "$_FOLDER/${_DATAFOLDER}/log.txt"
    
    echo
    
    echo $USERNAME" > "$REPO
    
    echo "Directory: ${_FOLDER}/${USERNAME}/${OUTPUT}"
    
    printf "\n"
    exit 0
    
else
    # Check Directory existing or not empty
    checkExistenceExit "${OUTPUT}"
fi

# ----------------------------------------
# Create 32 hex char temp directory
# Generate 32 char Uppercase Hex String

RDM=$(hexdump -vn16 -e'4/4 "%08X" 1 "\n"' /dev/urandom)
TMP="${TMP}/${RDM}"

createDir "${_FOLDER}/${_DATAFOLDER}/${TMP}"
# ----------------------------------------
createDir "${_FOLDER}/${USERNAME}/${OUTPUT}"

# Download files to tmp folder
cd "${_FOLDER}/${_DATAFOLDER}/${TMP}"
    
# Begin Cloning With Specific Parameters
if [[ $WITH_DATA -eq 1 ]]; then
    # When cloning with data
    if [[ ${TR} -eq 0 ]]; then
        # I'm still looking for the best to get the main branch name from github
        # Without defined branch
        git clone "https://${TOKEN}@github.com/${RES}" || { onError 1; }
    else
        git clone --single-branch --branch "${BRANCH}" "https://${TOKEN}@github.com/${RES}" || { onError 1; }
    fi
    
    echo -e "\e[1;32mCloned with data\e[0m"
else
    # Download Repo without .git folder
    
    # Init download
    curl -L "https://github.com/${RES}/tarball/${BRANCH}" | tar xz || { onError 1; }
    
    echo -e "\e[1;32mCloned without data\e[0m"
fi

# Move downloaded folder to...
F="${_FOLDER}/${_DATAFOLDER}/${TMP}/$(ls)"
    
if [[ ! -d "${F}" ]]; then
    onError 1
fi

# Desired location
mv -T "${F}" "${_FOLDER}/${USERNAME}/${OUTPUT}" || {
    echo -e "\e[1;31mCheck tmp folder\e[0m"
    onError 1; 
}

# ----------------------------------------
# Delete Created random 32 hex char string 

rm -rf "${_FOLDER}/${_DATAFOLDER}/${TMP}"
# ----------------------------------------
# Lets create Timestamp
# User > Branch > Repository
echo "$(date) | Clone | $USERNAME > $BRANCH > $REPO" >> "$_FOLDER/${_DATAFOLDER}/log.txt"


echo

#printf "Downloaded content size: "
#du -hs "${_FOLDER}/${RES}"

echo $USERNAME" > "$REPO

echo "Directory: ${_FOLDER}/${USERNAME}/${OUTPUT}"

printf "\n"

# Developed with Love and Frustration by Jovan De Guia
# License under MIT License
# Github Username: jxmked
# Model:SH-CSA-0009 - Advance Repository Cloner API