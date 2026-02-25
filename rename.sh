#!/bin/sh

set -e
set -u
export LC_ALL=C

if [ "$#" -lt 1 ]; then
	echo "Usage: $0 <new name>" 1>&2
	exit 1
fi

if pwd | grep -q rust-web-project-template; then
	echo "Clone into directory different than 'rust-web-project-template' before renaming," 1>&2
	echo "otherwise it's likely that you're corrupting template repository." 1>&2
	exit 1
fi

if sed , </dev/null 2>&1 | grep -q 'invalid command code'; then
	sed=bsd
elif sed , </dev/null 2>&1 | grep -q 'unknown command'; then
	sed=gnu
else
	echo "Unknown sed version (neither BSD, nor GNU?)" 1>&2
	exit 1
fi

placeholder_name="foobar"
target_name="$1"

echo "This command will re-init git repository in this directory (removing all git history)" 1>&2
echo "and rename all '$placeholder_name' instances (in file names or the code) to '$target_name'" 1>&2
echo -n "Is this OK [y/n]? " 1>&2
read answer
if [ "$answer" != 'y' -a "$answer" != 'Y' ]; then
	echo "Rename cancelled" 1>&2
	exit 1
fi

echo "Rename confirmed" 1>&2

rm -rf .git
rm rename.sh

find . -depth -name "*$placeholder_name*" | while read path; do
	mv "$path" "$(echo "$path" | sed -e "s|$placeholder_name|$target_name|")"
done

find . -type f | while read path; do
	if [ $sed = bsd ]; then
		sed -i '' -e "s|$placeholder_name|$target_name|" "$path"
	else
		sed -i -e "s|$placeholder_name|$target_name|" "$path"
	fi
done

git init
git add .
git commit -m "Init from rust-web-project-template"
