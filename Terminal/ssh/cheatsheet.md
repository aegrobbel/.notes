### Generate a new SSH key

```bash
$ ssh-keygen -t ed25519 -C "your_email@example.com"
```

### Adding your SSH key to the ssh-agent

To start the ssh-agent in the background:
```bash
$ eval "$(ssh-agent -s)"
```

Add the following to the ssh config file to automatically load keys and store passphrases in the keychain:

// in ~/.ssh/config
Host *
  AddKeysToAgent yes
  UseKeychain yes
  IdentityFile ~/.ssh/id_ed25519

Now add the key:
```bash
$ ssh-add -K ~/.ssh/id_ed25519
```
