#!/bin/bash

#Developed by Jovan De Guia.
#For personal use only.
#No License has been provided.

#Downloaded files will go here
_FOLDER='/storage/emulated/0/Working Directory/git clone'
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

BRANCH="master" #Default
TRH=""

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

cd "${_FOLDER}"

createDir "${_FOLDER}/${_DATAFOLDER}"
createDir "${USERNAME}"

cd "${_FOLDER}/${USERNAME}"

RES=$USERNAME"/"$REPO

#Check for existence of Github Repo
if ! curl --output /dev/null --silent --head --fail "https://github.com/${RES}"; then
    echo -e "\e[1;31mFailed: https://github.com/${RES} does not exists.\e[0m"
    echo "Exiting..."
    exit 1 #Terminate
fi

echo "Cloning...${1}"

printf "\n"

cd "${_FOLDER}"

#Just enter -w. Clone with data
if [[ $2 == '-w' ]]; then
    cd "${_FOLDER}/${USERNAME}"
    #When cloning with data
    TOKEN=<TOKEN HERE>
    
    if [[ $TR == 'tree' ]]; then
        git clone --single-branch --branch $BRANCH https://$TOKEN@github.com/$RES.git
    else
        #Without defined branch
        git clone "https://${TOKEN}@${1:8}.git"
    fi
    echo -e "\e[1;32mCloned with data\e[0m"
else
    #Download Repo without .git files
    #This download will only download files not .git
    createDir "${_FOLDER}/${_DATAFOLDER}/${TMP}"
    
    if [[ -d "${_FOLDER}/${RES}" ]]; then
        echo -e "\e[1;31mFailed: ${RES} does exists on local machine.\e[0m"
        echo "Exiting..."
        exit 1 #Terminate
    fi
    
    createDir "${_FOLDER}/${RES}"
    
    #Change dir to TMP folder to put their the downloaded files
    cd "${_FOLDER}/${_DATAFOLDER}/${TMP}"
    
    #Init download
    curl -L "https://github.com/${RES}/tarball/${BRANCH}" | tar xz
    
    #To Temporary Downloaded Folder
    F="${_FOLDER}/${_DATAFOLDER}/${TMP}/$(ls)"
    
    if [[ ! -d "${F}" ]]; then
        echo -e "\e[1;31mFailed: Something went wrong.\e[0m"
        echo "Exiting..."
        
        exit 1
    fi
    
    #Move files from TMP to Desired location
    mv -T "${F}" "${_FOLDER}/${RES}"
    
    echo -e "\e[1;32mCloned without data\e[0m"
fi

#Lets create Timestamp
#User > Branch > Repository
echo "$(date) | $USERNAME > $BRANCH > $REPO" >> "$_FOLDER/log.txt"

#printf "Downloaded content size: "
#du -hs "${_FOLDER}/${RES}"

echo $USERNAME" > "$REPO
printf "\n"

#Developed with Love and Frustration by Jovan De Guia
#For Personal Use Only
#No License has been provided
#Model:SH-CSA-0001 - Advance Repository Cloner API
