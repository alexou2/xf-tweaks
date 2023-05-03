#!/bin/bash

# list of dependancies to install
dependancies=("libgtk-4-1" "xfce4" "lightdm" "git")


# updates repositories

for package in $dependancies
do
# checks for libgtk version
if dpkg -s libgtk-4-1 >/dev/null 2>&1; then
    echo "libgtk-4-1 is installed"
else
    echo "libgtk-4-1 is not installed"
    echo "will start installing libgtk-4-1"
    # sudo apt install libgtk-4-1
fi
done


# clones the rust project from github
if [ -z $(find xf-tweaks)  ]
then
# git clone https://github.com/alexou2/xf-tweaks.git
echo install
fi