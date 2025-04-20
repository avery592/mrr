# mrr
Pronounced "mirror". This is a simple Rust cli for generating vanity SSH keys.
Specifically, it generates SSH keys whose SHA-256 fingerprint matches a
given regex. For example:
```
$ mrr '^avery'
Found: SHA256:avery1FYMMQQsL00uBtkTn1yLej4kZ+M7cdttNchWeQ. Private key written to id_ed25519, public key written to id_ed25519.pub.
```

**Note**: There is no validation or warning if the regex cannot match a
fingerprint. If this happens, the program will run indefinitely. Please ensure
your regex can match a valid SSH fingerprint (base64). The above 5 character
prefix took ~7 hours on my laptop at a rate of ~320K/s. Each additional
character will add a factor of ~64 to the expected time, so don't expect to
have prefixes much longer than ~7-8 characters complete in a reasonable amount
of time.