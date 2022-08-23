### Basics

To write a simple bash script, we need to create a file w/ a bash "shebang".
Then, we need to make the file executable by using `chmod`.

```bash
#!/bin/bash 

echo "Hello World!"
```

```bash
# set permissions for admin only
chmod 700 FILENAME.sh
```

Then, we will likely want to add the file/folder to our path:
```bash
 export PATH=$PATH:directory
 ```
 