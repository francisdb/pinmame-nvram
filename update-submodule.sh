#!/usr/bin/env bash
echo "Updating submodules"

git submodule update
git submodule foreach git checkout master
git submodule foreach git pull origin master
