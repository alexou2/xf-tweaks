#!/bin/bash

# list of dependancies to install
dependancies=("curl" "git")


# update repositories
sudo apt update

# install dependancies
for package in "${dependancies[@]}"
    do

# checks if every package is installed
    if dpkg -s $package >/dev/null 2>&1
        then
        echo -e "$package is installed\n"
    else
        echo "$package is not installed"
        echo -e "\u0007"

# confims installation of new package
        read -p "Do you want to install $package? [Y/n] " confirm
        if [[ "$confirm" = "y" ]]
            then
            sudo apt install $package
        else
            echo "Installation of $package aborted"
        fi

    fi
done

# clones the rust project from github
if [[ -z "$(find xf-tweaks)"  ]]
    then
    echo "cloning code from github in 2 sec..."
    # leaves time to cancel for debugging
    sleep 2
    # clones the whole repository 
    git clone https://github.com/alexou2/xf-tweaks.git

fi
echo -e "\n\n Please reboot the computer in order to apply changes"