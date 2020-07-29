# Usage
## Setup:
1. Clone the repo `git clone https://github.com/evan-brass/evan-brass.github.io.git` Output:
```
Cloning into 'evan-brass.github.io'...
remote: Enumerating objects: 105, done.
remote: Counting objects: 100% (105/105), done.
remote: Compressing objects: 100% (65/65), done.
remote: Total 105 (delta 38), reused 93 (delta 28), pack-reused 0
Receiving objects: 100% (105/105), 453.05 KiB | 2.48 MiB/s, done.
Resolving deltas: 100% (38/38), done.
```
2. Change directory `cd .\evan-brass.github.io\`
3. Init the submodule `git submodule init`
```
Submodule 'publish' (https://github.com/evan-brass/evan-brass.github.io.git) registered for path 'public'
```
4. Update the submodule `git submodule update`
```
Cloning into 'C:/Users/brass/source/repos/evan-brass.github.io/public'...
remote: Enumerating objects: 62, done.
remote: Counting objects: 100% (62/62), done.
remote: Compressing objects: 100% (33/33), done.
remote: Total 62 (delta 22), reused 61 (delta 21), pack-reused 0
Unpacking objects: 100% (62/62), 427.47 KiB | 191.00 KiB/s, done.
From https://github.com/evan-brass/evan-brass.github.io
 * branch            f90b51118d96ce061101f121e2f5ce04a1c9e4a6 -> FETCH_HEAD
Submodule path 'public': checked out 'f90b51118d96ce061101f121e2f5ce04a1c9e4a6'
```
5. Copy the file '.git' from public to static `cp .\public\.git .\static\` The file should have a contents similiar to this:
```
gitdir: ../.git/modules/publish
```
6. Run a Zola build (Might fail the first time) `zola build`
```
Failed to build the site
Error: Was not able to create folder public
Reason: Access is denied. (os error 5)
```
```
Building site...
-> Creating 6 pages (0 orphan), 2 sections, and processing 0 images
Done in 306ms.
```

## Building / Publishing:
1. Make your changes.
2. Run `zola build` from the root directory until it builds successfully
3. Change into the public folder `cd public`
4. Make sure your on the master branch `checkout master`
5. Add all changes `git add .`
6. Amend the "Built" commit `git commit --amend -m "Built"`
7. Force push to origin `git push --force`
8. Change back to the root directory `cd ..`
9. Add your changes and commit as normal. Be sure to include the changes to the public submodule in your commit. Lastly push.

# Why all the rigamarole?
So... Github user pages can only be generated from the master branch.  In order to not have two repositories, we just have two disjoint branches: main where all the content is and master which contains the built version of the site.

By default, running `zola build` from the main folder puts the output into a public directory so we might as well make that a submodule that points to the master branch of the same evan-brass.github.io repository.  However, since zola overwrites the folder, it overwrites the .git file on each website build.  This makes us lose all our nice submodule features.  That's why we copy the .git file into static so that zola outputs a new one during each build.  Is it hacky? So hacky.

As far as the master branch goes, I'm planning on just always force pushing and only having a single commit.  Since it's generated, all the history is stored in the main branch.

I'm not proud of it, but.
