# Custom File Server

- Requirements: a Linux Server with Domain (e.g. www.your-domain.com) + [Docker installed](https://docs.docker.com/engine/install).

1. ```cd file_server```
2. ```ls ./static``` This is where you can place your own material, remove the demo files and replace them with your content. (Be careful to follow the exact syntax as in the demo files!)
3. ```docker build -t miniserve -f Dockerfile_file_server .```
4.  ```nohup docker run --rm -p 8080:8080 -v "$(pwd)/static":/usr/workspace/public miniserve miniserve --auth user_name:password --random-route ./public &``` We use a docker container running miniserve to host the files, you will need to edit the user_name and password.   
5. ~~```nano nginx.conf``` edit ```server 172.17.0.1:8080;``` to match the port you used for ```miniserve``` at step 4.~~
6. ```nano nginx.conf``` update the ```server_name``` to match your domain name.
7. Use [Lets Encrypt](https://eff-certbot.readthedocs.io/en/stable/install.html#alternative-1-docker) to generate SSL keys for your domain.
8. Copy both of your keys into ```./config```. 
9. ```nano nginx.conf``` make sure the keys are referenced correctly.
10. ```docker build -t cors -f Dockerfile_nginx_cors .```
11. ```nohup docker run --rm --name cors -p 443:443 cors &```
12. Make sure your firewall settings (only) opens port ```443```.
