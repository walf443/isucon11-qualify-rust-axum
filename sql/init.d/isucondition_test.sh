#!/bin/sh

echo "create database if not exists isucondition_test;" | "${mysql[@]}"
echo "grant all on isucondition_test.* TO '"$MYSQL_USER"'@'%';" | "${mysql[@]}"
echo "flush privileges;" | "${mysql[@]}"
