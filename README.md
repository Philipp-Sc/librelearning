# librelearning

**For when you really just want to <u>learn</u> with your <u>own material</u> right now!**

# Frontend 

- Flashcards
- Spaced Repetition
- Embed Audio and Images

# Backend

Host your own material:

1. ```cd file_server```
2. ```docker build -t miniserve -f Dockerfile_file_server .```
3. ```nano nginx.conf``` edit ```server 172.17.0.1:8080;``` to match the port you want to use for ```miniserve```
4. ```docker build -t cors .```
5. ```nohup docker run --rm -p 8080:8080 -v "$(pwd)/static":/usr/workspace/public miniserve miniserve --auth librelearning:123 --random-route ./public &```
6. ```nohup docker run --rm --name cors -p 8081:80 cors &``` edit ```8081``` to match the port you want to expose.
Make sure your firewall settings (only) opens port ```8081```.
