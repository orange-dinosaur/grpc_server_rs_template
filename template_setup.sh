#!/bin/bash

old_project_name="grpc_server_rs_template"
new_project_name=$(basename "$PWD")
old_proto_service_name="GrpcServerRsTemplate"
new_proto_service_name=$(echo "$new_project_name" | sed -r 's/(^|_)([a-z])/\U\2/g; s/_(.)/\U\1/g')

# Rename proto file
mv proto/$old_project_name.proto "proto/$new_project_name.proto"

# Use find and sed to replace the string in all files within the current directory and its subdirectories
find . -type f -execdir sed -i "s/$old_project_name/$new_project_name/g" {} \;
find . -type f -execdir sed -i "s/$old_proto_service_name/$new_proto_service_name/g" {} \;

# Delete current generated grpc implementation
rm src/$old_project_name.rs

# Delete current git repo
rm -rf .git

# Build the project
cargo build

# Add env files to .gitignore
echo "" >> .gitignore
echo "# Env files" >> .gitignore
echo "*.env*" >> .gitignore

# Start clean git repo
git init

echo ""
echo "Your project is ready, enjoy!"

