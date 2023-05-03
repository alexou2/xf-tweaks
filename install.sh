#!/bin/bash

# updates repositories


# checks for libgtk version
if dpkg -s libgtk-4-1 >/dev/null 2>&1; then
    echo "libgtk-4-1 is installed"
else
    echo "libgtk-4-1 is not installed"
    echo "will start installing libgtk-4-1"
    sudo apt install libgtk-4-1
fi


# checks if the xfce4 desktop environment is installed 
if dpkg -s xfce4-session >/dev/null 2>&1; then
    echo "xfce4 is installed"
else
    echo "xfce4 is not installed"
    echo "will start installing XFCE4"
    sudo apt install xfce4
fi

# checks if the lightdm desktop manager is installed 
if dpkg -s lightdm >/dev/null 2>&1; then
    echo "lightdm is installed"
else
    echo "lightdm is not installed"
    echo "will start installing lightdm"
    sudo apt install lightdm
fi

# checks if git is installed 
if dpkg -s git >/dev/null 2>&1; then
    echo "git is installed"
else
    echo "git is not installed"
    echo "will start installing git"
    sudo apt install git
fi

# clones the rust project from github
git clone https://github.com/alexou2/xf-tweaks.git