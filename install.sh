#!/bin/bash

# list of dependancies to install
dependancies=("libgtk-4-1" "xfce4" "lightdm" "git")


# updates repositories

for package in ${dependancies[@]}
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
        if [ "$confirm" = "y" ]
            then
            sudo apt install $package
        else
            echo "Installation of $package aborted"
        fi

    fi
done

# clones the rust project from github
if [ -z $(find xf-tweaks)  ]
    then
    echo "cloning code from github..."
    # git clone https://github.com/alexou2/xf-tweaks.git
fi