```bash
# serveur écoutant sur le port 8888
ncat -l 8888
# client
ncat 127.0.0.1 8888
# serveur écoutant sur le port 8888 en UDP
ncat -l 8888
# client UDP
ncat 127.0.0.1 -u 8888
```