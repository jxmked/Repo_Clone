#!/bin/bash

#For personal use only.

#Downloaded files will go here
_FOLDER='/storage/emulated/0/@webpage/git clone'
_DATAFOLDER=".xio"

#Temporary folder
TMP="tmp"

USERNAME=""
REPO=""

#Check for Parameter url
if [[ ! $1 ]]; then
    echo "No parameter found"
    exit 1
fi

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

if [[ $(echo $1 | cut -d'/' -f 1) == 'https:' ]]; then 
    USERNAME=$(echo $1 | cut -d'/' -f 4)
    REPO=$(echo $1 | cut -d'/' -f 5)
    
    TR=$(echo $1 | cut -d'/' -f 6)
    
    #Check for branch
    if [[ $TR == "tree" ]]; then
        
        cn=-1
        IFS='/' read -ra TMPB <<< "$1"
        
        for i in "${TMPB[@]}"; do
            cn=$(($cn + ${#i} + 1))
            if [[ $TR == $i ]]; then
                BRANCH=${1:($cn + 1)}
                break
            fi
        done
    fi
else
    USERNAME=$(echo $1 | cut -d'/' -f 1)
    REPO=$(echo $1 | cut -d'/' -f 2)
fi

RES="${USERNAME}/${REPO}"

#Check for existence of Github Repo
if ! curl --output /dev/null --silent --head --fail "https://github.com/${RES}"; then
    echo -e "\e[1;31mFailed: https://github.com/${RES} does not exists.\e[0m"
    echo "Exiting..."
    exit 1 #Terminate
fi

echo "Cloning..."
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
if [[ $2 == '-w' ]]; then
    #When cloning with data
    TOKEN=<TOKEN HERE>
    
    if [[ $TR == 'tree' ]]; then
        git clone --single-branch --branch $BRANCH https://${TOKEN}@github.com/$RES.git || { onError 1; }
    else
        #Without defined branch
        git clone "https://${TOKEN}@${1:8}.git" || { onError 1; }
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

#printf "Downloaded content size: "
#du -hs "${_FOLDER}/${RES}"

echo $USERNAME" > "$REPO
printf "\n"

#Developed with Love and Frustration by Jovan
#For Personal Use Only
#License under MIT
#Model:SH-CSA-0002 - Advance Repository Cloner API
